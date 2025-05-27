# SWC Utilities

## What

This library aims to be a thin wrapper around the [swc](https://swc.rs/) library, to help with deobfuscating and reverse-engineering.

Links: [crates.io](https://crates.io/crates/swc_re_utils)

## How

The usage is quite simple, just run `cargo add swc_re_utils swc_ecma_visit` to get started.

```rust
use swc_re_utils::transformations;
use swc_re_utils::utils::swc_utils;
use std::fs;
use swc_ecma_visit::VisitMutWith;

fn main() {
    let src = fs::read_to_string("input.js").expect("Unable to read file");
    let mut ast = swc_utils::parse_func_str(src.clone());

    ast.visit_mut_with(&mut transformations::normalize_ast::Visitor {});

    ast.visit_mut_with(&mut transformations::sequence_exprs::Visitor {});
    ast.visit_mut_with(&mut transformations::constant_evaluation::Visitor {});
    ast.visit_mut_with(&mut transformations::remove_unused::DeadCodeVisitor {});

    ast.visit_mut_with(&mut transformations::cleanup::Visitor {});

    let out = swc_utils::generate_code(ast);
    fs::write("output.js", out).expect("Unable to write file");
}
```

### Why

This aims to be an example of how to use the latest version of the swc library (as of May 2025),
because many other repositories are either outdated or got taken down due to shady DMCA claims.
For a JavaScript/Babel equivalent, take a look at [deob-transformations](https://github.com/pagpeter/deob-transformations)

### Inspiration & Credits

I have used code & concepts from the following resources:

- [manudeobs/turnstile-rs](https://github.com/manudeobs/turnstile-rs)
- [leafypout/vercel-anti-bot](https://github.com/leafypout/vercel-anti-bot)
- [steakenthusiast blog](https://steakenthusiast.github.io/)

Also thanks to @rsa2048 for some snippets.
