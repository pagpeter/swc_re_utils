use swc_common::{DUMMY_SP, util::take::Take};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{
    BlockStmt, Expr, ForStmt, IdentName, IfStmt, Lit, MemberExpr, MemberProp, Number, Program,
};

pub struct Visitor {}

impl VisitMut for Visitor {
    fn visit_mut_number(&mut self, n: &mut Number) {
        n.visit_mut_children_with(self);
        *n = Number {
            value: n.value,
            raw: None,
            span: DUMMY_SP,
        };
    }

    fn visit_mut_if_stmt(&mut self, path: &mut IfStmt) {
        path.visit_mut_children_with(self);

        if !path.cons.is_block() {
            path.cons = BlockStmt {
                stmts: vec![*path.cons.take()],
                ..Default::default()
            }
            .into();
        }

        let alt = if let Some(x) = path.alt.as_ref() {
            x
        } else {
            return;
        };

        if !alt.is_block() {
            path.alt = Some(
                BlockStmt {
                    stmts: vec![*path.alt.take().unwrap().take()],
                    ..Default::default()
                }
                .into(),
            );
        }
    }

    fn visit_mut_for_stmt(&mut self, path: &mut ForStmt) {
        path.visit_mut_children_with(self);

        if !path.body.is_block() {
            path.body = BlockStmt {
                stmts: vec![*path.body.take()],
                ..Default::default()
            }
            .into();
        }
    }

    fn visit_mut_member_expr(&mut self, member_expr: &mut MemberExpr) {
        member_expr.visit_mut_children_with(self);

        if let MemberProp::Computed(property) = &member_expr.prop {
            if let Expr::Lit(Lit::Str(s)) = &*property.expr {
                if !s.value.contains('-') {
                    member_expr.prop =
                        MemberProp::Ident(IdentName::new(s.value.clone(), property.span));
                }
            }
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);
        
        // Replace ![] with true
        if let Expr::Unary(unary) = expr {
            if unary.op == swc_ecma_ast::UnaryOp::Bang && 
               matches!(&*unary.arg, Expr::Array(arr) if arr.elems.is_empty()) {
                *expr = Expr::Lit(Lit::Bool(swc_ecma_ast::Bool {
                    span: DUMMY_SP,
                    value: true,
                }));
            }
        }
        
        // Replace !![] with false
        if let Expr::Unary(unary) = expr {
            if unary.op == swc_ecma_ast::UnaryOp::Bang && 
               matches!(&*unary.arg, Expr::Unary(inner) if 
                       inner.op == swc_ecma_ast::UnaryOp::Bang && 
                       matches!(&*inner.arg, Expr::Array(arr) if arr.elems.is_empty())) {
                *expr = Expr::Lit(Lit::Bool(swc_ecma_ast::Bool {
                    span: DUMMY_SP,
                    value: false,
                }));
            }
        }
    }

    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Normalizing ast");
        n.visit_mut_children_with(self);
    }
}
