use crate::ast::*;
use std::collections::HashSet;

/// Zet v0.2 — Gerçek Leke Analizi (Taint Analysis)
/// 
/// Felsefe: "Dış dünyadan gelen hiçbir veriye güvenilmez."
/// 
/// Kurallar:
/// 1. `Untrusted` tipli fonksiyon parametreleri lekeli (tainted) olarak doğar.
/// 2. `Console.read`, `HTTP.get` gibi dış kaynaklardan alınan veriler lekelidir.
/// 3. Lekeli bir değişken, `validate` bloğunun `success` kapsamına girene kadar
///    herhangi bir kritik işlemde (aritmetik, fonksiyon argümanı, return) KULLANILAMAZ.
/// 4. `validate target { success: { ... } }` bloğunun success kapsamında
///    `target` temizlenmiş (trusted) kabul edilir.

const TAINTED_SOURCES: &[&str] = &["Console.read", "HTTP.get"];

pub struct TaintAnalyzer {
    tainted: HashSet<String>,
}

impl TaintAnalyzer {
    pub fn check(func: &FunctionDef) -> Result<(), String> {
        let mut analyzer = TaintAnalyzer {
            tainted: HashSet::new(),
        };

        // Kural 1: Untrusted tipli parametreleri lekele
        for param in &func.params {
            if param.param_type == TypeRef::Untrusted {
                analyzer.tainted.insert(param.name.clone());
            }
        }

        analyzer.visit_block(&func.body)?;
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), String> {
        for stmt in &block.statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(l) => {
                // Kural 2: Eğer sağ taraf lekeli bir kaynak ise, değişkeni lekele
                if self.is_tainted_source(&l.value) {
                    self.tainted.insert(l.name.clone());
                } else if self.is_tainted_propagation(&l.value) {
                    // Lekeli değişkenden türetilen değer de lekeli
                    self.tainted.insert(l.name.clone());
                } else {
                    // Sağ taraftaki ifadede lekeli değişken var mı kontrol et
                    self.assert_expr_clean(&l.value)?;
                }
                Ok(())
            }
            Statement::Assign { name, value } => {
                if self.is_tainted_source(value) {
                    self.tainted.insert(name.clone());
                } else if self.is_tainted_propagation(value) {
                    self.tainted.insert(name.clone());
                } else {
                    self.assert_expr_clean(value)?;
                }
                Ok(())
            }
            Statement::If { condition, then_block, else_block } => {
                self.assert_expr_clean(condition)?;
                self.visit_block(then_block)?;
                if let Some(b) = else_block { self.visit_block(b)?; }
                Ok(())
            }
            Statement::While { condition, body } => {
                self.assert_expr_clean(condition)?;
                self.visit_block(body)?;
                Ok(())
            }
            Statement::For { start, end, step, body, .. } => {
                self.assert_expr_clean(start)?;
                self.assert_expr_clean(end)?;
                if let Some(s) = step { self.assert_expr_clean(s)?; }
                self.visit_block(body)?;
                Ok(())
            }
            Statement::ScopeBlock { body, .. } => {
                self.visit_block(body)
            }
            Statement::ValidateBlock { target, success_scope, on_fail, .. } => {
                // Kural 3-4: validate bloğu lekeli değişkeni temizler
                if !self.tainted.contains(target) {
                    return Err(format!(
                        "'{}' zaten temiz (trusted) bir değişken. `validate` bloğu yalnızca lekeli (Untrusted) veriler için kullanılabilir.", target
                    ));
                }

                // on_fail bloğunda target hâlâ lekeli
                {
                    let mut fail_analyzer = TaintAnalyzer {
                        tainted: self.tainted.clone(),
                    };
                    fail_analyzer.visit_block(on_fail)?;
                }

                // success bloğunda target temizlenmiş (trusted)
                {
                    let mut success_analyzer = TaintAnalyzer {
                        tainted: self.tainted.clone(),
                    };
                    success_analyzer.tainted.remove(target);
                    success_analyzer.visit_block(success_scope)?;
                }

                Ok(())
            }
            Statement::ExprStmt(e) => {
                self.assert_expr_clean(e)
            }
            Statement::Return(Some(e)) => {
                self.assert_expr_clean(e)
            }
            Statement::Return(None) => Ok(()),
        }
    }

    /// Bir ifadenin doğrudan lekeli veri kaynağı olup olmadığını kontrol eder
    fn is_tainted_source(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call(name, _, _) => TAINTED_SOURCES.contains(&name.as_str()),
            _ => false,
        }
    }

    /// Lekeli bir değişkenden türetilmiş mi? (ör: json(tainted_var, "key"))
    fn is_tainted_propagation(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(name) => self.tainted.contains(name),
            Expr::JsonField(source, _) => self.is_tainted_propagation(source),
            Expr::Index(arr, _) => self.is_tainted_propagation(arr),
            _ => false,
        }
    }

    /// İfade içinde lekeli değişken kullanımı varsa HATA ver.
    /// Bu, Zet'in kalbidir: lekeli veri asla doğrudan kullanılamaz.
    fn assert_expr_clean(&self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Identifier(name) => {
                if self.tainted.contains(name) {
                    Err(format!(
                        "'{}' lekeli (Untrusted) bir değişken! Kullanmadan önce `validate {} {{ success: {{ ... }} }}` bloğu ile doğrulamalısınız.",
                        name, name
                    ))
                } else {
                    Ok(())
                }
            }
            Expr::Literal(_) => Ok(()),
            Expr::Binary(left, _, right) => {
                self.assert_expr_clean(left)?;
                self.assert_expr_clean(right)
            }
            Expr::Call(_, args, _) => {
                for arg in args {
                    self.assert_expr_clean(arg)?;
                }
                Ok(())
            }
            Expr::Spawn(inner) => self.assert_expr_clean(inner),
            Expr::Await(inner) => self.assert_expr_clean(inner),
            Expr::Infra(call) => {
                for arg in &call.args {
                    self.assert_expr_clean(arg)?;
                }
                Ok(())
            }
            Expr::JsonField(source, _) => self.assert_expr_clean(source),
            Expr::ArrayLiteral(elems) => {
                for e in elems { self.assert_expr_clean(e)?; }
                Ok(())
            }
            Expr::Index(arr, idx) => {
                self.assert_expr_clean(arr)?;
                self.assert_expr_clean(idx)
            }
        }
    }
}