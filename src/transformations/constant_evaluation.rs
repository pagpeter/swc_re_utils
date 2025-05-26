use swc_common::GLOBALS;
use swc_common::util::take::Take;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Lit, Module, ModuleItem, Program, Script};
use swc_ecma_minifier::eval::Evaluator;
use swc_ecma_minifier::{self, eval, marks};
struct EvaluateVisitor {
    evaluator: Evaluator,
    program: Program,
}

impl EvaluateVisitor {
    pub fn new(evaluator: eval::Evaluator, program: Program) -> Self {
        Self { evaluator, program }
    }
}

impl VisitMut for EvaluateVisitor {
    fn visit_mut_program(&mut self, node: &mut Program) {
        node.visit_mut_children_with(self);
        println!("[*] Running constant evaluation 2");
        self.program = node.clone()
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

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
            let mut visitor = EvaluateVisitor::new(evaluator, Program::dummy());

            n.visit_mut_with(&mut visitor);

            *n = visitor.program;
        })
    }
}
