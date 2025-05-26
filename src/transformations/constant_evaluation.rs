use swc_common::GLOBALS;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Lit, Module, ModuleItem, Program};
use swc_ecma_minifier::eval::Evaluator;
use swc_ecma_minifier::{self, eval, marks};

use crate::utils;



struct EvaluateVisitor {
    evaluator: Evaluator,
}

impl EvaluateVisitor {
    pub fn new(evaluator: eval::Evaluator) -> Self {
        Self { evaluator }
    }
}

impl VisitMut for EvaluateVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Some(res) = self.evaluator.eval(expr) {
            match res {
                eval::EvalResult::Lit(lit) => match lit {
                    Lit::Bool(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Bool(l)),
                    Lit::Str(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Str(l)),
                    Lit::Null(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Null(l)),
                    Lit::Num(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Num(l)),
                    _ => {}
                },
                eval::EvalResult::Undefined => {}
            }
        }
    }


    fn visit_mut_expr_stmt(&mut self, stmt: &mut swc_ecma_ast::ExprStmt) {
        stmt.expr.visit_mut_children_with(self);
        
        // After visiting children, try to evaluate the expression
        if let Some(res) = self.evaluator.eval(&stmt.expr) {
            match res {
                eval::EvalResult::Lit(lit) => {
                    stmt.expr = Box::from(Expr::Lit(lit));
                }
                _ => {}
            }
        }
    }
    // Handle binary expressions with special attention to double negatives
    fn visit_mut_bin_expr(&mut self, expr: &mut swc_ecma_ast::BinExpr) {
        expr.visit_mut_children_with(self);
        
        // Special case: check if right operand is a unary minus
        if let Expr::Unary(unary) = &*expr.right {
            if unary.op == swc_ecma_ast::UnaryOp::Minus {
                if expr.op == swc_ecma_ast::BinaryOp::Sub {
                    // Convert subtraction with negative to addition: a - (-b) => a + b
                    expr.op = swc_ecma_ast::BinaryOp::Add;
                    expr.right = unary.arg.clone();
                } else if expr.op == swc_ecma_ast::BinaryOp::Add {
                    // Convert addition with negative to subtraction: a + (-b) => a - b
                    expr.op = swc_ecma_ast::BinaryOp::Sub;
                    expr.right = unary.arg.clone();
                }
            }
        }

        // Try to evaluate the entire binary expression
        if let Some(res) = self.evaluator.eval(&Expr::Bin(expr.clone())) {
            if let eval::EvalResult::Lit(lit) = res {
                *expr = swc_ecma_ast::BinExpr {
                    span: expr.span,
                    op: expr.op,
                    left: Box::new(*expr.left.clone()),
                    right: Box::new(Expr::Lit(lit)),
                };
            }
        }
    }

    // Handle assignments with constant expressions
    fn visit_mut_assign_expr(&mut self, expr: &mut swc_ecma_ast::AssignExpr) {
        expr.right.visit_mut_with(self);
        expr.left.visit_mut_with(self);
        
        // Try to evaluate the right side of the assignment
        if let Some(res) = self.evaluator.eval(&expr.right) {
            if let eval::EvalResult::Lit(lit) = res {
                expr.right = Box::new(Expr::Lit(lit));
            }
        }
    }

}

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Running constant evaluation");

        GLOBALS.set(&Default::default(), || {
            let m: marks::Marks = marks::Marks::new();
            let module = match n {
                Program::Module(module_prog) => Module {
                    body: module_prog.body.clone(),
                    ..Default::default()
                },
                Program::Script(script) => Module {
                    body: script
                        .body
                        .clone()
                        .into_iter()
                        .map(|stmt| ModuleItem::Stmt(stmt))
                        .collect(),
                    ..Default::default()
                },
            };

            let evaluator = Evaluator::new(module, m);
            let mut visitor = EvaluateVisitor::new(evaluator);

            n.visit_mut_with(&mut visitor);
        })
    }
}
