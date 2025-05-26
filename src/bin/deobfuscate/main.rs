use deobfuscator::transformations;
use deobfuscator::utils::logger::Logger;
use deobfuscator::utils::swc_utils;
use std::{env, fs};
use swc_ecma_visit::VisitMutWith;

extern crate deobfuscator;
fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let logger = Logger::new("main");
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    if filename.is_none() {
        return logger.error("You must pass in the file path");
    }

    let filename = filename.unwrap();

    let src = fs::read_to_string(filename).expect("Unable to read file");
    logger.success(format!("Read {} chars from {}", src.len(), filename).as_str());

    let mut ast = swc_utils::parse_func_str(src.clone());

    ast.visit_mut_with(&mut transformations::normalize_ast::Visitor {});
    ast.visit_mut_with(&mut transformations::sequence_exprs::Visitor {});
    ast.visit_mut_with(&mut transformations::constant_evaluation::Visitor {});
    ast.visit_mut_with(&mut transformations::remove_unused::DeadCodeVisitor {});

    ast.visit_mut_with(&mut transformations::obfuscatorio::proxy_functions::Visitor {});
    ast.visit_mut_with(&mut transformations::obfuscatorio::string_array::Visitor::new(src));

    ast.visit_mut_with(&mut transformations::cleanup::Visitor {});

    let out = swc_utils::generate_code(ast);
    fs::write(filename.replace(".js", ".out.js"), out).expect("Unable to write file");
}
