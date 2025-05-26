use swc_common::GLOBALS;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Lit, Program};
use swc_ecma_minifier::eval::Evaluator;
use swc_ecma_minifier::{self, eval, marks};
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

        println!("Eval");
        if let Some(res) = self.evaluator.eval(expr) {
            match res {
                eval::EvalResult::Lit(lit) => match lit {
                    Lit::Bool(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Bool(l.to_owned())),
                    // Lit::Str(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Str(l.to_owned())),
                    // Lit::Null(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Null(l.to_owned())),
                    // Lit::Num(l) => *expr = Expr::Lit(swc_ecma_ast::Lit::Num(l.to_owned())),
                    _ => {}
                },
                eval::EvalResult::Undefined => {}
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
            let evaluator = match n.as_module() {
                Some(module) => Evaluator::new(module.to_owned(), m),
                None => return, // Exit early if not a module
            };
            let mut visitor = EvaluateVisitor::new(evaluator);

            n.visit_mut_children_with(&mut visitor);
            n.visit_mut_children_with(self);
        })
    }
}
