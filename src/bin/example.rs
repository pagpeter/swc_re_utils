use std::{env, fs};
use swc_ecma_visit::VisitMutWith;
use swc_re_utils::transformations;
use swc_re_utils::utils::swc_utils;

extern crate swc_re_utils;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Needs filename as argument");

    let src = fs::read_to_string(filename).expect("Unable to read file");
    let mut ast = swc_utils::parse_func_str(src.clone());

    ast.visit_mut_with(&mut transformations::normalize_ast::Visitor {});

    ast.visit_mut_with(&mut transformations::sequence_exprs::Visitor {});
    ast.visit_mut_with(&mut transformations::constant_evaluation::Visitor {});
    ast.visit_mut_with(&mut transformations::remove_unused::DeadCodeVisitor {});

    ast.visit_mut_with(&mut transformations::cleanup::Visitor {});

    let out = swc_utils::generate_code(ast);
    fs::write(filename.replace(".js", ".out.js"), out).expect("Unable to write file");
}
