use deobfuscator::utils::logger::Logger;
use deobfuscator::utils::swc_utils;
use std::{env, fs};

extern  crate  deobfuscator;
fn main() {
    let logger = Logger::new("main");
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    if filename.is_none() {
        return logger.error("You must pass in the file path");
    }

    let filename = filename.unwrap();

    let src = fs::read_to_string(filename).expect("Unable to read file");
    logger.success(format!("Read {} chars from {}", src.len(), filename).as_str());

    let ast = swc_utils::parse_func_str(src);
    let out = swc_utils::generate_code(ast);
    fs::write(filename.replace(".js", ".out.js"), out).expect("Unable to write file");
}
