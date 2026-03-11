use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, multispace1, not_line_ending},
    combinator::{map, map_res, opt, recognize, value, verify},
    sequence::{delimited, pair, preceded, tuple, terminated},
    branch::alt,
    multi::{separated_list0, many0, many1},
    IResult,
};
use crate::ast::*;

/// Anahtar kelimeleri kelime sınırı kontrolüyle eşleştiren parser.
/// "for" yazınca "for_testi" gibi identifier'ları yanlışlıkla eşleştirmez.
fn kw<'a>(word: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (remaining, matched) = tag(word)(input)?;
        if remaining.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        }
        Ok((remaining, matched))
    }
}

fn sp<'a, E: nom::error::ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), many0(alt((value((), multispace1), value((), pair(tag("//"), not_line_ending))))))(input)
}

fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where F: FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(sp, inner, sp)
}

// ─── Temel Parçalayıcılar ───────────────────────────────────────

fn identifier(input: &str) -> IResult<&str, String> {
    verify(
        map(recognize(pair(alt((alpha1, tag("_"))), take_while(|c: char| c.is_alphanumeric() || c == '_'))), |s: &str| s.to_string()),
        |s: &String| !["if", "else", "let", "const", "mut", "while", "for", "in", "by", "scope", "spawn", "call", "json", "validate", "true", "false", "return", "break", "continue", "nondeterministic", "deterministic", "nondet", "det", "fn", "Untrusted", "i64", "f64", "bool", "char", "u8", "Void", "print", "println", "input", "inputln"].contains(&s.as_str())
    )(input)
}

fn dot_identifier(input: &str) -> IResult<&str, String> {
    map(recognize(pair(identifier, pair(char('.'), identifier))), |s: &str| s.to_string())(input)
}

fn float_number(input: &str) -> IResult<&str, f64> {
    map_res(recognize(tuple((digit1, char('.'), digit1))), |s: &str| s.parse::<f64>())(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn char_literal(input: &str) -> IResult<&str, char> {
    let (input, _) = char('\'')(input)?;
    let (input, c) = if input.starts_with("\\n") { Ok((&input[2..], '\n')) }
        else if input.starts_with("\\t") { Ok((&input[2..], '\t')) }
        else if input.starts_with("\\\\") { Ok((&input[2..], '\\')) }
        else if input.starts_with("\\'") { Ok((&input[2..], '\'')) }
        else if input.starts_with("\\0") { Ok((&input[2..], '\0')) }
        else {
            let c = input.chars().next().ok_or(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Char)))?;
            Ok((&input[c.len_utf8()..], c))
        }?;
    let (input, _) = char('\'')(input)?;
    Ok((input, c))
}

/// String interpolation destekli metin parçalayıcı: "text ${expr} more text"
fn parse_string_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = char('"')(input)?;
    let mut parts: Vec<InterpolPart> = Vec::new();
    let mut current_lit = String::new();
    let mut rest = input;
    let mut has_interpolation = false;
    
    loop {
        if rest.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(rest, nom::error::ErrorKind::Char)));
        }
        if rest.starts_with('"') {
            rest = &rest[1..];
            break;
        }
        if rest.starts_with("${") {
            has_interpolation = true;
            if !current_lit.is_empty() {
                parts.push(InterpolPart::Lit(current_lit.clone()));
                current_lit.clear();
            }
            rest = &rest[2..]; // skip ${
            if let Some(end_pos) = rest.find('}') {
                let expr_str = &rest[..end_pos];
                match parse_expr(expr_str) {
                    Ok((_, expr)) => parts.push(InterpolPart::Expr(expr)),
                    Err(_) => return Err(nom::Err::Error(nom::error::Error::new(rest, nom::error::ErrorKind::Tag))),
                }
                rest = &rest[end_pos + 1..];
            } else {
                return Err(nom::Err::Error(nom::error::Error::new(rest, nom::error::ErrorKind::Char)));
            }
        } else if rest.starts_with("\\\"") { current_lit.push('"'); rest = &rest[2..]; }
        else if rest.starts_with("\\\\") { current_lit.push('\\'); rest = &rest[2..]; }
        else if rest.starts_with("\\n") { current_lit.push('\n'); rest = &rest[2..]; }
        else if rest.starts_with("\\t") { current_lit.push('\t'); rest = &rest[2..]; }
        else {
            let c = rest.chars().next().unwrap();
            current_lit.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    
    if has_interpolation {
        if !current_lit.is_empty() {
            parts.push(InterpolPart::Lit(current_lit));
        }
        Ok((rest, Expr::Interpolation(parts)))
    } else {
        Ok((rest, Expr::Literal(Literal::Str(current_lit))))
    }
}

/// Sadece düz metin okuyan string parser (interpolation yok). JSON key vb. için.
fn plain_string_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let mut result = String::new();
    let mut rest = input;
    loop {
        if rest.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(rest, nom::error::ErrorKind::Char)));
        }
        if rest.starts_with('"') { rest = &rest[1..]; return Ok((rest, result)); }
        if rest.starts_with("\\\"") { result.push('"'); rest = &rest[2..]; }
        else if rest.starts_with("\\\\") { result.push('\\'); rest = &rest[2..]; }
        else if rest.starts_with("\\n") { result.push('\n'); rest = &rest[2..]; }
        else if rest.starts_with("\\t") { result.push('\t'); rest = &rest[2..]; }
        else {
            let c = rest.chars().next().unwrap();
            result.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
}

// ─── Dizi, JSON, Infra ─────────────────────────────────────────

fn parse_array_literal(input: &str) -> IResult<&str, Expr> {
    map(delimited(
        ws(char('[')),
        separated_list0(ws(char(',')), parse_expr),
        ws(char(']'))
    ), Expr::ArrayLiteral)(input)
}

fn parse_json_field(input: &str) -> IResult<&str, Expr> {
    let (input, _) = ws(kw("json"))(input)?;
    let (input, _) = ws(char('('))(input)?;
    let (input, source) = parse_expr(input)?;
    let (input, _) = ws(char(','))(input)?;
    let (input, key) = ws(plain_string_literal)(input)?;
    let (input, _) = ws(char(')'))(input)?;
    Ok((input, Expr::JsonField(Box::new(source), key)))
}

fn parse_infra_expr(input: &str) -> IResult<&str, Expr> {
    map(tuple((
        ws(kw("call")), identifier, ws(char('.')), identifier, ws(char('(')), separated_list0(ws(char(',')), parse_expr), ws(char(')')), 
        ws(char('{')), ws(kw("timeout")), ws(char(':')), number, ws(char('}'))
    )), |(_, s, _, m, _, a, _, _, _, _, t, _)| Expr::Infra(InfraCall { service: s, method: m, args: a, config: InfraConfig { timeout_ms: t as u64 } }))(input)
}

// ─── Spawn, Call, Print, Input ──────────────────────────────────

fn parse_spawn(input: &str) -> IResult<&str, Expr> {
    map(preceded(ws(kw("spawn")), parse_expr), |e| Expr::Spawn(Box::new(e)))(input)
}

fn parse_call_expr(input: &str) -> IResult<&str, Expr> {
    map(tuple((
        opt(ws(kw("call"))), 
        alt((dot_identifier, identifier)), 
        ws(char('(')), 
        separated_list0(ws(char(',')), parse_expr), 
        ws(char(')'))
    )), |(call_kw, n, _, a, _)| Expr::Call(n, a, call_kw.is_some()))(input)
}

fn parse_print(input: &str) -> IResult<&str, Expr> {
    map(tuple((
        ws(alt((kw("println"), kw("print")))),
        ws(char('(')),
        separated_list0(ws(char(',')), parse_expr),
        ws(char(')'))
    )), |(keyword, _, args, _)| {
        let name = if keyword == "println" { "println".to_string() } else { "print".to_string() };
        Expr::Call(name, args, false)
    })(input)
}

fn parse_input_expr(inp: &str) -> IResult<&str, Expr> {
    map(tuple((
        opt(ws(kw("call"))),
        ws(alt((kw("inputln"), kw("input")))),
        ws(char('(')),
        separated_list0(ws(char(',')), parse_expr),
        ws(char(')'))
    )), |(call_kw, keyword, _, args, _)| {
        let name = if keyword == "inputln" { "inputln".to_string() } else { "input".to_string() };
        Expr::Call(name, args, call_kw.is_some())
    })(inp)
}

// ─── Tuple & Parantez ───────────────────────────────────────────

fn parse_paren_or_tuple(input: &str) -> IResult<&str, Expr> {
    let (input, _) = ws(char('('))(input)?;
    let (input, first) = parse_expr(input)?;
    // Virgül varsa → Tuple
    if let Ok((input, _)) = ws::<_, _, nom::error::Error<&str>>(char(','))(input) {
        let (input, mut rest_exprs) = separated_list0(ws(char(',')), parse_expr)(input)?;
        let (input, _) = opt(ws(char(',')))(input)?;
        let (input, _) = ws(char(')'))(input)?;
        let mut all = vec![first];
        all.append(&mut rest_exprs);
        Ok((input, Expr::TupleLiteral(all)))
    } else {
        let (input, _) = ws(char(')'))(input)?;
        Ok((input, first))
    }
}

// ─── Primary Expressions ────────────────────────────────────────

fn parse_primary(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_spawn,
        parse_infra_expr, 
        parse_json_field,
        parse_print,
        parse_input_expr,
        parse_call_expr,  
        parse_array_literal,
        map(float_number, |f| Expr::Literal(Literal::Float(f))),
        map(number, |n| Expr::Literal(Literal::Int(n))),
        map(char_literal, |c| Expr::Literal(Literal::Char(c))),
        parse_string_expr,
        map(kw("true"), |_| Expr::Literal(Literal::Bool(true))),
        map(kw("false"), |_| Expr::Literal(Literal::Bool(false))),
        map(identifier, Expr::Identifier),
        parse_paren_or_tuple,
    ))(input)
}

// ─── Atom: primary + index ([]) + tuple index (.0) ──────────────

fn parse_atom(input: &str) -> IResult<&str, Expr> {
    let (input, mut expr) = parse_primary(input)?;
    let (mut input, indices) = many0(delimited(ws(char('[')), parse_expr, ws(char(']'))))(input)?;
    for idx in indices {
        expr = Expr::Index(Box::new(expr), Box::new(idx));
    }
    // Tuple indeks erişimi: t.0, t.1
    loop {
        let try_input = input;
        if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('.')(try_input) {
            if let Ok((rest2, idx)) = digit1::<&str, nom::error::Error<&str>>(rest) {
                if rest2.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                    break;
                }
                let index: usize = idx.parse().unwrap_or(0);
                expr = Expr::TupleIndex(Box::new(expr), index);
                input = rest2;
                continue;
            }
        }
        break;
    }
    Ok((input, expr))
}

// ─── Operatör Öncelik Hiyerarşisi (düşükten yükseğe) ───────────
//
// parse_expr       →  || (logical OR)
// parse_and        →  && (logical AND)
// parse_comparison →  ==, !=, >=, <=, >, <
// parse_bitor      →  | (bitwise OR)
// parse_bitxor     →  ^ (bitwise XOR)
// parse_bitand     →  & (bitwise AND)
// parse_shift      →  <<, >> (bit kaydırma)
// parse_term       →  +, - (toplama, çıkarma)
// parse_factor     →  *, /, % (çarpma, bölme, mod)
// parse_unary      →  !, - (tekli operatörler)
// parse_atom       →  primary + index/tuple access

fn parse_unary(input: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(ws(char('!')), parse_unary), |e| Expr::Unary(UnaryOp::Not, Box::new(e))),
        map(preceded(ws(char('-')), parse_unary), |e| Expr::Unary(UnaryOp::Neg, Box::new(e))),
        parse_atom,
    ))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_unary(input)?;
    let (input, ops) = many0(pair(ws(alt((char('*'), char('/'), char('%')))), parse_unary))(input)?;
    for (op_char, right) in ops {
        let op = match op_char { '*' => BinaryOp::Mul, '/' => BinaryOp::Div, '%' => BinaryOp::Mod, _ => unreachable!() };
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }
    Ok((input, left))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_factor(input)?;
    let (input, ops) = many0(pair(ws(alt((char('+'), char('-')))), parse_factor))(input)?;
    for (op_char, right) in ops {
        let op = match op_char { '+' => BinaryOp::Add, '-' => BinaryOp::Sub, _ => unreachable!() };
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }
    Ok((input, left))
}

fn parse_shift(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_term(input)?;
    let (input, ops) = many0(pair(ws(alt((tag("<<"), tag(">>")))), parse_term))(input)?;
    for (op_str, right) in ops {
        let op = match op_str { "<<" => BinaryOp::Shl, ">>" => BinaryOp::Shr, _ => unreachable!() };
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }
    Ok((input, left))
}

fn parse_bitand(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_shift(input)?;
    let mut rest = input;
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with('&') && !trimmed.starts_with("&&") {
            let after_amp = trimmed[1..].trim_start();
            match parse_shift(after_amp) {
                Ok((remaining, right)) => {
                    left = Expr::Binary(Box::new(left), BinaryOp::BitAnd, Box::new(right));
                    rest = remaining;
                }
                Err(_) => break,
            }
        } else {
            break;
        }
    }
    Ok((rest, left))
}

fn parse_bitxor(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_bitand(input)?;
    let (input, ops) = many0(pair(ws(char('^')), parse_bitand))(input)?;
    for (_, right) in ops {
        left = Expr::Binary(Box::new(left), BinaryOp::BitXor, Box::new(right));
    }
    Ok((input, left))
}

fn parse_bitor(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_bitxor(input)?;
    let mut rest = input;
    loop {
        let trimmed = rest.trim_start();
        if trimmed.starts_with('|') && !trimmed.starts_with("||") {
            let after_pipe = trimmed[1..].trim_start();
            match parse_bitxor(after_pipe) {
                Ok((remaining, right)) => {
                    left = Expr::Binary(Box::new(left), BinaryOp::BitOr, Box::new(right));
                    rest = remaining;
                }
                Err(_) => break,
            }
        } else {
            break;
        }
    }
    Ok((rest, left))
}

fn parse_comparison(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_bitor(input)?;
    let (input, ops) = many0(pair(ws(alt((
        tag("=="), tag("!="), 
        tag(">="), tag("<="),
        tag(">"), tag("<")
    ))), parse_bitor))(input)?;
    
    for (op_str, right) in ops {
        let op = match op_str { 
            "==" => BinaryOp::Eq, "!=" => BinaryOp::Neq, 
            ">=" => BinaryOp::Gte, "<=" => BinaryOp::Lte,
            ">" => BinaryOp::Gt, "<" => BinaryOp::Lt, 
            _ => unreachable!() 
        };
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }
    Ok((input, left))
}

fn parse_and(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_comparison(input)?;
    let (input, ops) = many0(pair(ws(tag("&&")), parse_comparison))(input)?;
    for (_, right) in ops {
        left = Expr::Binary(Box::new(left), BinaryOp::And, Box::new(right));
    }
    Ok((input, left))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_and(input)?;
    let (input, ops) = many0(pair(ws(tag("||")), parse_and))(input)?;
    for (_, right) in ops {
        left = Expr::Binary(Box::new(left), BinaryOp::Or, Box::new(right));
    }
    Ok((input, left))
}

// ─── Statement Parçalayıcıları ──────────────────────────────────

fn parse_let(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("let")), opt(ws(kw("mut"))), identifier, ws(char('=')), parse_expr)), |(_, _, n, _, v)| Statement::Let(LetStmt { name: n, value: v }))(input)
}

fn parse_const(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("const")), identifier, ws(char('=')), parse_expr)), |(_, n, _, v)| Statement::Const { name: n, value: v })(input)
}

fn parse_assign(input: &str) -> IResult<&str, Statement> {
    map(tuple((identifier, ws(char('=')), parse_expr)), |(n, _, v)| Statement::Assign { name: n, value: v })(input)
}

fn parse_return(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("return")), opt(parse_expr))), |(_, expr)| Statement::Return(expr))(input)
}

fn parse_break(input: &str) -> IResult<&str, Statement> {
    map(ws(kw("break")), |_| Statement::Break)(input)
}

fn parse_continue(input: &str) -> IResult<&str, Statement> {
    map(ws(kw("continue")), |_| Statement::Continue)(input)
}

fn parse_while(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("while")), parse_expr, ws(char('{')), parse_block_content, ws(char('}')))), |(_, cond, _, stmts, _)| Statement::While { condition: cond, body: Block { statements: stmts } })(input)
}

fn parse_for(input: &str) -> IResult<&str, Statement> {
    map(tuple((
        ws(kw("for")), identifier, ws(kw("in")), parse_expr, ws(tag("..")), parse_expr,
        opt(preceded(ws(kw("by")), parse_expr)),
        ws(char('{')), parse_block_content, ws(char('}'))
    )), |(_, var, _, start, _, end, step, _, stmts, _)| 
        Statement::For { var, start, end, step, body: Block { statements: stmts } }
    )(input)
}

fn parse_if(input: &str) -> IResult<&str, Statement> {
    map(tuple((
        ws(kw("if")), parse_expr, ws(char('{')), parse_block_content, ws(char('}')),
        opt(preceded(ws(kw("else")), alt((
            map(parse_if, |stmt| match stmt { Statement::If { .. } => vec![stmt], _ => unreachable!() }),
            delimited(ws(char('{')), parse_block_content, ws(char('}')))
        ))))
    )), |(_, cond, _, then_stmts, _, else_stmts)| {
        Statement::If { condition: cond, then_block: Block { statements: then_stmts }, else_block: else_stmts.map(|stmts| Block { statements: stmts }) }
    })(input)
}

fn parse_scope(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("scope")), identifier, ws(char('{')), parse_block_content, ws(char('}')))), |(_, n, _, s, _)| Statement::ScopeBlock { name: n, body: Block { statements: s } })(input)
}

fn parse_validate(input: &str) -> IResult<&str, Statement> {
    map(tuple((ws(kw("validate")), identifier, ws(char('{')), ws(kw("success")), ws(char(':')), ws(char('{')), parse_block_content, ws(char('}')), ws(char('}')))), 
    |(_, t, _, _, _, _, s, _, _)| Statement::ValidateBlock { target: t, schema: "Schema".to_string(), on_fail: Box::new(Block{statements:vec![]}), success_scope: Box::new(Block { statements: s }) })(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_let, parse_const, parse_if, parse_while, parse_for, parse_scope, parse_validate, parse_break, parse_continue, parse_assign, parse_return, map(terminated(parse_expr, opt(ws(char(';')))), |e| Statement::ExprStmt(e))))(input)
}

fn parse_block_content(input: &str) -> IResult<&str, Vec<Statement>> { many0(ws(parse_statement))(input) }

// ─── Tip Sistemi ────────────────────────────────────────────────

fn parse_type(input: &str) -> IResult<&str, TypeRef> {
    ws(alt((
        map(tag("Untrusted"), |_| TypeRef::Untrusted),
        map(tag("i64"), |_| TypeRef::Integer),
        map(tag("f64"), |_| TypeRef::Float),
        map(tag("bool"), |_| TypeRef::Bool),
        map(tag("char"), |_| TypeRef::Char),
        map(tag("u8"), |_| TypeRef::Byte),
        map(tag("Void"), |_| TypeRef::Void),
        map(tuple((ws(tag("Array")), ws(char('<')), parse_type, ws(char('>')))), |(_, _, t, _)| TypeRef::Array(Box::new(t))),
        map(delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_type),
            ws(char(')'))
        ), |types| if types.len() == 1 { types.into_iter().next().unwrap() } else { TypeRef::Tuple(types) }),
        map(identifier, |s| TypeRef::Custom(s))
    )))(input)
}

// ─── Fonksiyon Tanımlayıcı ─────────────────────────────────────

fn parse_function(input: &str) -> IResult<&str, FunctionDef> {
    map(tuple((
        ws(alt((
            map(alt((kw("deterministic"), kw("det"))), |_| Purity::Deterministic),
            map(alt((kw("nondeterministic"), kw("nondet"))), |_| Purity::Nondeterministic)
        ))),
        ws(kw("fn")), ws(identifier), ws(char('(')),
        separated_list0(ws(char(',')), map(tuple((ws(identifier), ws(char(':')), parse_type)), |(n, _, t)| Param { name: n, param_type: t })),
        ws(char(')')), 
        opt(preceded(ws(tag("->")), parse_type)), 
        ws(char('{')), parse_block_content, ws(char('}'))
    )), |(p, _, n, _, params, _, ret, _, stmts, _)| FunctionDef { name: n, purity: p, params, return_type: ret.map(|r| r).unwrap_or(TypeRef::Void), body: Block { statements: stmts } })(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<FunctionDef>> {
    many1(ws(parse_function))(input)
}