use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable { pub functions: HashMap<String, FunctionDef> }

/// Zet v0.2 — Determinizm Analizi
///
/// Felsefe: "Saf fonksiyonlar yalnızca CPU ve RAM kullanır."
///
/// Kurallar:
/// 1. `deterministic` fonksiyon içinde `spawn`, `Infra`, `Await` kullanılamaz.
/// 2. `deterministic` fonksiyon, `nondeterministic` bir fonksiyonu ÇAĞIRAMAZ.
/// 3. Stdlib'deki bilinen I/O fonksiyonları (DB.log, Console.read, HTTP.get, Util.now, Util.to_int)
///    saf fonksiyonlardan çağrılamaz.
/// 4. `call` anahtar kelimesi yalnızca nondeterministic fonksiyon/çağrı için kullanılabilir.
///    Saf fonksiyon çağrısına `call` eklemek hatadır.

const NONDETERMINISTIC_STDLIB: &[&str] = &[
    "DB.log", "Console.read", "HTTP.get", "Util.now", "Util.to_int"
];

pub struct DeterminismAnalyzer;

impl DeterminismAnalyzer {
    pub fn check(func: &FunctionDef, symbols: &SymbolTable) -> Result<(), String> {
        if let Purity::Deterministic = func.purity {
            if !Self::is_block_pure(&func.body, symbols) {
                return Err(format!(
                    "'{}' fonksiyonu `deterministic` olarak işaretlenmiş, ancak içinde I/O veya nondeterministic çağrı var!",
                    func.name
                ));
            }
        }

        // Kural 4: `call` keyword kontrolü — tüm fonksiyonlarda geçerli
        Self::check_call_keyword(&func.body, symbols)?;

        Ok(())
    }

    fn is_block_pure(block: &Block, symbols: &SymbolTable) -> bool {
        block.statements.iter().all(|s| Self::is_stmt_pure(s, symbols))
    }

    fn is_stmt_pure(stmt: &Statement, symbols: &SymbolTable) -> bool {
        match stmt {
            Statement::Let(l) => Self::is_expr_pure(&l.value, symbols),
            Statement::Assign { value, .. } => Self::is_expr_pure(value, symbols),
            Statement::If { condition, then_block, else_block } => {
                Self::is_expr_pure(condition, symbols) 
                && Self::is_block_pure(then_block, symbols) 
                && else_block.as_ref().map(|b| Self::is_block_pure(b, symbols)).unwrap_or(true)
            }
            Statement::While { condition, body } => {
                Self::is_expr_pure(condition, symbols) && Self::is_block_pure(body, symbols)
            }
            Statement::For { start, end, step, body, .. } => {
                Self::is_expr_pure(start, symbols) 
                && Self::is_expr_pure(end, symbols) 
                && step.as_ref().map(|s| Self::is_expr_pure(s, symbols)).unwrap_or(true) 
                && Self::is_block_pure(body, symbols)
            }
            Statement::ScopeBlock { .. } => false, // scope blokları saf olamaz
            Statement::ValidateBlock { success_scope, on_fail, .. } => {
                Self::is_block_pure(success_scope, symbols) && Self::is_block_pure(on_fail, symbols)
            }
            Statement::ExprStmt(expr) | Statement::Return(Some(expr)) => Self::is_expr_pure(expr, symbols),
            Statement::Return(None) => true,
        }
    }

    fn is_expr_pure(expr: &Expr, symbols: &SymbolTable) -> bool {
        match expr {
            Expr::Literal(_) | Expr::Identifier(_) => true,
            Expr::Binary(l, _, r) => Self::is_expr_pure(l, symbols) && Self::is_expr_pure(r, symbols),
            Expr::Call(name, args, _) => {
                // Kural 3: Stdlib I/O fonksiyonları saf değildir
                if NONDETERMINISTIC_STDLIB.contains(&name.as_str()) {
                    return false;
                }
                // Kural 2: Kullanıcı tanımlı nondeterministic fonksiyon çağrısı
                if let Some(target_func) = symbols.functions.get(name) {
                    if let Purity::Nondeterministic = target_func.purity {
                        return false;
                    }
                }
                // Argümanlar da saf olmalı
                args.iter().all(|a| Self::is_expr_pure(a, symbols))
            }
            Expr::JsonField(source, _) => Self::is_expr_pure(source, symbols),
            Expr::ArrayLiteral(elems) => elems.iter().all(|e| Self::is_expr_pure(e, symbols)),
            Expr::Index(arr, idx) => Self::is_expr_pure(arr, symbols) && Self::is_expr_pure(idx, symbols),
            // spawn, await, infra → kesinlikle saf değil
            Expr::Spawn(_) | Expr::Await(_) | Expr::Infra(_) => false,
        }
    }

    /// Kural 4: `call` keyword yalnızca nondeterministic çağrılar için kullanılabilir.
    /// Saf bir fonksiyona `call` eklemek — ya da I/O fonksiyonunu `call` olmadan çağırmak hatadır.
    fn check_call_keyword(block: &Block, symbols: &SymbolTable) -> Result<(), String> {
        for stmt in &block.statements {
            Self::check_call_in_stmt(stmt, symbols)?;
        }
        Ok(())
    }

    fn check_call_in_stmt(stmt: &Statement, symbols: &SymbolTable) -> Result<(), String> {
        match stmt {
            Statement::Let(l) => Self::check_call_in_expr(&l.value, symbols),
            Statement::Assign { value, .. } => Self::check_call_in_expr(value, symbols),
            Statement::If { condition, then_block, else_block } => {
                Self::check_call_in_expr(condition, symbols)?;
                Self::check_call_keyword(then_block, symbols)?;
                if let Some(b) = else_block { Self::check_call_keyword(b, symbols)?; }
                Ok(())
            }
            Statement::While { condition, body } => {
                Self::check_call_in_expr(condition, symbols)?;
                Self::check_call_keyword(body, symbols)
            }
            Statement::For { start, end, step, body, .. } => {
                Self::check_call_in_expr(start, symbols)?;
                Self::check_call_in_expr(end, symbols)?;
                if let Some(s) = step { Self::check_call_in_expr(s, symbols)?; }
                Self::check_call_keyword(body, symbols)
            }
            Statement::ScopeBlock { body, .. } => Self::check_call_keyword(body, symbols),
            Statement::ValidateBlock { success_scope, on_fail, .. } => {
                Self::check_call_keyword(success_scope, symbols)?;
                Self::check_call_keyword(on_fail, symbols)
            }
            Statement::ExprStmt(e) | Statement::Return(Some(e)) => Self::check_call_in_expr(e, symbols),
            Statement::Return(None) => Ok(()),
        }
    }

    fn check_call_in_expr(expr: &Expr, symbols: &SymbolTable) -> Result<(), String> {
        match expr {
            Expr::Call(name, args, awaited) => {
                let is_nondet = NONDETERMINISTIC_STDLIB.contains(&name.as_str())
                    || symbols.functions.get(name).map(|f| matches!(f.purity, Purity::Nondeterministic)).unwrap_or(false);

                if *awaited && !is_nondet {
                    return Err(format!(
                        "`call {}(...)` hatalı: '{}' saf (deterministic) bir fonksiyon. `call` yalnızca I/O fonksiyonları için kullanılır.",
                        name, name
                    ));
                }
                // Not: nondeterministic fonksiyon call olmadan da çağrılabilir (spawn içinde olabilir)
                // Bu nedenle tersini zorlamıyoruz burada — spawn ile call birlikte kullanılmaz

                for arg in args { Self::check_call_in_expr(arg, symbols)?; }
                Ok(())
            }
            Expr::Binary(l, _, r) => {
                Self::check_call_in_expr(l, symbols)?;
                Self::check_call_in_expr(r, symbols)
            }
            Expr::Spawn(inner) => Self::check_call_in_expr(inner, symbols),
            Expr::Await(inner) => Self::check_call_in_expr(inner, symbols),
            Expr::Infra(call) => {
                for arg in &call.args { Self::check_call_in_expr(arg, symbols)?; }
                Ok(())
            }
            Expr::JsonField(source, _) => Self::check_call_in_expr(source, symbols),
            Expr::ArrayLiteral(elems) => {
                for e in elems { Self::check_call_in_expr(e, symbols)?; }
                Ok(())
            }
            Expr::Index(arr, idx) => {
                Self::check_call_in_expr(arr, symbols)?;
                Self::check_call_in_expr(idx, symbols)
            }
            _ => Ok(()),
        }
    }
}