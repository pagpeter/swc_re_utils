// all credit to https://github.com/manudeobs/turnstile-rs/blob/c03c0e4b0e9e67fb7ca45243ce0e446e3fc0de17/src/ast/visitor/dead_code.rs

use swc_common::{SyntaxContext, util::take::Take};
use swc_ecma_ast::{BinaryOp, CondExpr, Expr, IfStmt, Program, Stmt};
use swc_ecma_visit::{VisitMut, VisitMutWith, noop_visit_mut_type};

pub struct DeadCodeVisitor {}

impl VisitMut for DeadCodeVisitor {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, path: &mut Expr) {
        path.visit_mut_children_with(self);

        if !path.is_cond() {
            return;
        }

        let CondExpr {
            test, cons, alt, ..
        } = path.as_cond().unwrap();

        let bin_expr = if let Some(bin_expr) = test.as_bin() {
            bin_expr
        } else {
            return;
        };
        let left = if let Some(lit) = bin_expr.left.as_lit() {
            if let Some(string) = lit.as_str() {
                string.value.to_string()
            } else {
                return;
            }
        } else {
            return;
        };
        let right = if let Some(lit) = bin_expr.right.as_lit() {
            if let Some(string) = lit.as_str() {
                string.value.to_string()
            } else {
                return;
            }
        } else {
            return;
        };

        let use_cons = match bin_expr.op {
            BinaryOp::EqEqEq => left == right,
            BinaryOp::NotEqEq => left != right,
            _ => panic!(),
        };

        let evaled = if use_cons { cons } else { alt };

        *path = *evaled.clone();
    }
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Deleting dead code");
        n.visit_mut_children_with(self);
    }

    fn visit_mut_stmt(&mut self, path: &mut Stmt) {
        path.visit_mut_children_with(self);

        match path {
            Stmt::If(IfStmt {
                test, cons, alt, ..
            }) => {
                let bin_expr = if let Some(bin_expr) = test.as_bin() {
                    bin_expr
                } else {
                    return;
                };
                let left = if let Some(lit) = bin_expr.left.as_lit() {
                    if let Some(string) = lit.as_str() {
                        string.value.to_string()
                    } else {
                        return;
                    }
                } else {
                    return;
                };
                let right = if let Some(lit) = bin_expr.right.as_lit() {
                    if let Some(string) = lit.as_str() {
                        string.value.to_string()
                    } else {
                        return;
                    }
                } else {
                    return;
                };

                let use_cons = match bin_expr.op {
                    BinaryOp::EqEqEq => left == right,
                    BinaryOp::NotEqEq => left != right,
                    _ => panic!(),
                };

                let resolved = if use_cons {
                    cons
                } else {
                    alt.as_ref().unwrap()
                };

                *path = *resolved.clone()
            }
            _ => {}
        }
    }

    fn visit_mut_stmts(&mut self, path: &mut Vec<Stmt>) {
        path.visit_mut_children_with(self);
        let stmts = path.drain(..).collect::<Vec<Stmt>>();
        let mut new_stmts: Vec<Stmt> = Vec::new();
        for stmt in stmts {
            new_stmts.push(stmt.clone());
            match stmt {
                Stmt::If(IfStmt {
                    test, cons, alt, ..
                }) => {
                    let bin_expr = if let Some(bin_expr) = test.as_bin() {
                        bin_expr
                    } else {
                        continue;
                    };
                    let left = if let Some(lit) = bin_expr.left.as_lit() {
                        if let Some(string) = lit.as_str() {
                            string.value.to_string()
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };
                    let right = if let Some(lit) = bin_expr.right.as_lit() {
                        if let Some(string) = lit.as_str() {
                            string.value.to_string()
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };

                    let use_cons = match bin_expr.op {
                        BinaryOp::EqEqEq => left == right,
                        BinaryOp::NotEqEq => left != right,
                        _ => panic!(),
                    };

                    let mut stmts_to_push = if use_cons { cons } else { alt.unwrap() };

                    new_stmts.pop();

                    match *stmts_to_push.take() {
                        Stmt::Block(block) => {
                            for stmt in block.stmts {
                                new_stmts.push(stmt)
                            }
                        }
                        Stmt::Return(return_stmt) => new_stmts.push(return_stmt.into()),
                        _x => {
                            new_stmts.push(_x.into());
                            //dbg!(generate_code(_x.into()));
                            //panic!()
                        }
                    }
                }
                _ => {}
            };
        }

        *path = new_stmts
    }

    fn visit_mut_while_stmt(&mut self, stmt: &mut swc_ecma_ast::WhileStmt) {
        stmt.visit_mut_children_with(self);

        if let Expr::Lit(lit) = &*stmt.test {
            if let swc_ecma_ast::Lit::Bool(bool_lit) = lit {
                if !bool_lit.value {
                    // Replace while(false) with an empty block
                    stmt.body = Box::new(Stmt::Block(swc_ecma_ast::BlockStmt {
                        span: stmt.span,
                        stmts: vec![],
                        ctxt: SyntaxContext::empty(),
                    }));
                }
            }
        }
    }

    fn visit_mut_if_stmt(&mut self, stmt: &mut swc_ecma_ast::IfStmt) {
        stmt.visit_mut_children_with(self);

        if let Expr::Lit(lit) = &*stmt.test {
            if let swc_ecma_ast::Lit::Bool(bool_lit) = lit {
                if !bool_lit.value {
                    // If there's an else clause, use it; otherwise use empty block
                    if let Some(alt) = &stmt.alt {
                        *stmt.cons = *alt.clone();
                    } else {
                        *stmt.cons = *Box::new(Stmt::Block(swc_ecma_ast::BlockStmt {
                            span: stmt.span,
                            stmts: vec![],
                            ctxt: SyntaxContext::empty(),
                        }));
                    }
                }
            }
        }
    }
}
