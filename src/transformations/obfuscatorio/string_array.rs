use regex::Regex;
use swc_atoms::Atom;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{ArrayLit, Expr, Lit, Program, Str};
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Default)]
struct FindInteger {
    ints: Vec<f64>,
}
impl Visit for FindInteger {
    fn visit_number(&mut self, n: &swc_ecma_ast::Number) {
        self.ints.push(n.value)
    }
}

#[derive(Default)]

struct ReplaceProxyCalls {
    subtract: i32,
    strings: Vec<String>,
}

impl ReplaceProxyCalls {
    pub fn new(subtract: i32, strings: Vec<String>) -> Self {
        Self { subtract, strings }
    }
}

impl VisitMut for ReplaceProxyCalls {
    fn visit_mut_expr(&mut self, expr: &mut swc_ecma_ast::Expr) {
        expr.visit_mut_children_with(self);

        if !expr.is_call() {
            return;
        }
        let n = expr.as_call().unwrap();

        if n.args.len() != 1 {
            return;
        }

        let arg = n.args[0].expr.as_lit();
        if let Some(p) = arg {
            let mut find = FindInteger::default();
            p.to_owned().visit_children_with(&mut find);
            if find.ints.len() == 1 {
                let i: i32 = find.ints[0] as i32;

                let works = usize::try_from(i - self.subtract);
                if let Ok(res) = works {
                    if self.strings.len() > res {
                        let str = self.strings[res].to_owned();
                        *expr = Expr::Lit(Lit::Str(Str::from(str)));
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct FindStringArray {
    done_string: bool,
    strings: Vec<String>,
}

impl VisitMut for FindStringArray {
    fn visit_mut_array_lit(&mut self, node: &mut ArrayLit) {
        if self.done_string {
            return;
        }

        if node.elems.len() >= 20
            && node.elems.iter().all(|e| {
            if let Some(expr) = e {
                matches!(
                &*expr.expr,
                Expr::Lit(Lit::Str(_))
            )
            } else {
                false
            }
        })
        {
            self.done_string = true;
            self.strings = node
                .elems
                .iter()
                .map(|e| {
                    if let Some(expr) = e {
                        if let Expr::Lit(Lit::Str(s)) = &*expr.expr {
                            s.value.to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                })
                .collect();
        }
    }

}

pub struct Visitor {
    source: String,
    stringify: i32,
    subtract: i32,
}

impl Visitor {
    pub fn new(source: String) -> Self {
        Self {
            source,
            stringify: 0,
            subtract: 0,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        println!("[*] Finding string array");
        let mut obf_strings = FindStringArray::default();
        program.visit_mut_children_with(&mut obf_strings);
        if !obf_strings.done_string  {
            println!("  [!] Error finding string array");
            return;
        }

        println!("  [+] Found array with {} strings", obf_strings.strings.len());


    }
}
