use std::sync::Arc;
use swc_common::FileName;
use swc_common::SourceMap;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_ecma_ast::{Lit, Program};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{EsSyntax, Parser, Syntax};

// Much credit goes to https://github.com/manudeobs/turnstile-rs/blob/master/src/ast/utils.rs

pub fn parse_func_str(script: String) -> Program {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(FileName::Custom("test.js".into()).into(), script);

    let mut parser = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        None,
    );

    let program = parser.parse_program().expect("failed to parser module");

    program
}

pub fn generate_code(ast: Program) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    let config: swc_ecma_codegen::Config = Default::default();
    let mut emitter = Emitter {
        cfg: config,
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut buf, None),
    };

    emitter.emit_program(&ast.into()).unwrap();
    let code = String::from_utf8_lossy(&buf).to_string();
    code
}

pub fn number_from_lit(lit: &Lit) -> f64 {
    let num: f64;

    match lit {
        Lit::Num(n) => num = n.value,
        _ => num = 0.0,
    }

    num
}

// Thanks to @rsa2048
pub fn node_to_string<T>(node: &T) -> String
where
    T: swc_core::ecma::codegen::Node,
{
    let source_map = Arc::<SourceMap>::default();

    let mut buf = vec![];

    let mut e = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(),
        cm: source_map.clone(),
        comments: None,
        wr: JsWriter::new(source_map, "\n", &mut buf, None),
    };

    // !! may fail, handle it if you need to
    node.emit_with(&mut e).unwrap();

    String::from_utf8_lossy(&buf).to_string()
}
