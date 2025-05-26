use swc_common::GLOBALS;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Lit, Module, ModuleItem, Program};
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
