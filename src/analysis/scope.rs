use crate::ast::*;
use std::collections::HashSet;

/// Zet v0.2 — Kapsam ve Yapısal Eşzamanlılık Analizi
///
/// Felsefe: "Sıktığınız her mermi (thread), şarjöre (scope) hesap vermek zorundadır."
///
/// Kurallar:
/// 1. Tanımlanmamış değişken kullanımı yasaktır.
/// 2. `spawn` ifadesi yalnızca bir `scope` bloğu içinde kullanılabilir.
///    Scope dışında spawn → derleme hatası.
/// 3. Bu sayede zombi süreç oluşması imkânsız hale gelir.

pub struct ScopeAnalyzer {
    defined_vars: HashSet<String>,
    /// Şu an bir scope bloğunun içinde miyiz?
    in_scope: bool,
    /// İç içe scope derinliği
    scope_depth: usize,
    /// Döngü içinde miyiz? (break/continue kontrolü)
    in_loop: bool,
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        Self {
            defined_vars: HashSet::new(),
            in_scope: false,
            scope_depth: 0,
            in_loop: false,
        }
    }

    pub fn analyze(&mut self, func: &FunctionDef) -> Result<(), String> {
        self.defined_vars.clear();
        self.in_scope = false;
        self.scope_depth = 0;
        self.in_loop = false;
        for param in &func.params {
            self.defined_vars.insert(param.name.clone());
        }
        self.visit_block(&func.body)
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), String> {
        let backup = self.defined_vars.clone();
        for stmt in &block.statements {
            self.visit_stmt(stmt)?;
        }
        self.defined_vars = backup;
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(l) => {
                self.visit_expr(&l.value)?;
                self.defined_vars.insert(l.name.clone());
            }
            Statement::Assign { name, value } => {
                if !self.defined_vars.contains(name) {
                    return Err(format!("Tanımsız değişken: '{}'", name));
                }
                self.visit_expr(value)?;
            }
            Statement::If { condition, then_block, else_block } => {
                self.visit_expr(condition)?;
                self.visit_block(then_block)?;
                if let Some(else_b) = else_block { self.visit_block(else_b)?; }
            }
            Statement::Const { name, value } => {
                self.visit_expr(value)?;
                self.defined_vars.insert(name.clone());
            }
            Statement::Break => {
                if !self.in_loop {
                    return Err("`break` yalnızca bir döngü (while/for) içinde kullanılabilir!".to_string());
                }
            }
            Statement::Continue => {
                if !self.in_loop {
                    return Err("`continue` yalnızca bir döngü (while/for) içinde kullanılabilir!".to_string());
                }
            }
            Statement::While { condition, body } => {
                self.visit_expr(condition)?;
                let was_in_loop = self.in_loop;
                self.in_loop = true;
                self.visit_block(body)?;
                self.in_loop = was_in_loop;
            }
            Statement::For { var, start, end, step, body } => {
                self.visit_expr(start)?;
                self.visit_expr(end)?;
                if let Some(s) = step { self.visit_expr(s)?; }
                self.defined_vars.insert(var.clone());
                let was_in_loop = self.in_loop;
                self.in_loop = true;
                self.visit_block(body)?;
                self.in_loop = was_in_loop;
            }
            Statement::ScopeBlock { body, .. } => {
                // Scope'a giriyoruz — spawn artık izinli
                let was_in_scope = self.in_scope;
                self.in_scope = true;
                self.scope_depth += 1;
                self.visit_block(body)?;
                self.scope_depth -= 1;
                self.in_scope = was_in_scope;
            }
            Statement::ValidateBlock { target, success_scope, on_fail, .. } => {
                if !self.defined_vars.contains(target) {
                    return Err(format!("Tanımsız değişken (validate): '{}'", target));
                }
                self.visit_block(on_fail)?;
                self.visit_block(success_scope)?;
            }
            Statement::ExprStmt(expr) => self.visit_expr(expr)?,
            Statement::Return(Some(e)) => self.visit_expr(e)?,
            Statement::Return(None) => {},
        }
        Ok(())
    }

    fn visit_expr(&self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Identifier(name) => {
                if !self.defined_vars.contains(name) {
                    return Err(format!("Tanımsız değişken: '{}'", name));
                }
            }
            Expr::Binary(l, _, r) => { self.visit_expr(l)?; self.visit_expr(r)?; }
            Expr::Unary(_, inner) => { self.visit_expr(inner)?; }
            Expr::Interpolation(parts) => {
                for p in parts {
                    if let InterpolPart::Expr(e) = p { self.visit_expr(e)?; }
                }
            }
            Expr::TupleLiteral(elems) => {
                for e in elems { self.visit_expr(e)?; }
            }
            Expr::TupleIndex(expr, _) => { self.visit_expr(expr)?; }
            Expr::Call(_, args, _) => {
                for arg in args { self.visit_expr(arg)?; }
            }
            Expr::Spawn(e) => {
                // KURAL 2: spawn yalnızca scope içinde kullanılabilir
                if !self.in_scope {
                    return Err(
                        "`spawn` yalnızca bir `scope` bloğu içinde kullanılabilir! Zombi süreçleri engellemek için tüm arka plan işlemleri bir kapsam (scope) içinde yaşamalıdır.".to_string()
                    );
                }
                self.visit_expr(e)?;
            }
            Expr::Await(e) => self.visit_expr(e)?,
            Expr::Infra(call) => {
                for arg in &call.args { self.visit_expr(arg)?; }
            }
            Expr::JsonField(source, _) => self.visit_expr(source)?,
            Expr::ArrayLiteral(elems) => {
                for e in elems { self.visit_expr(e)?; }
            }
            Expr::Index(arr, idx) => {
                self.visit_expr(arr)?;
                self.visit_expr(idx)?;
            }
            _ => {}
        }
        Ok(())
    }
}