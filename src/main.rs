// awsm.rs
// The Complete AWSM-VM & AOT Compiler (v1.0 - PWS v1.1 Parity)
// Target: wasm32-wasi

pub mod abi;
pub mod lexer;
pub mod parser;
pub mod pws;
pub mod sph;
pub mod vm;
pub mod tests;

use std::io::{self, Read, Write};

use crate::abi::AwsmErrorCode;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::AwsmVm;

fn main() {
    let mut script = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut script) {
        script = r#"
        ;; RECIPE: standardize-api-errors
        (awsm:pipe (<)
            (awsm:manage-import :ensure "core.exceptions" :module "with_standard_error_handling" :is-type false)
            (awsm:mutate-call :target @route_fn :inject {:decorators ["with_standard_error_handling"]})
        )
        "#.to_string();
    }

    let mut lexer = Lexer::new(&script);
    let tokens = lexer.tokenize().unwrap_or_else(|e| {
        eprintln!("LEXER_ERROR: {}", e);
        std::process::exit(AwsmErrorCode::PurityFail as i32);
    });

    let mut parser = Parser::new(tokens);
    let mut bytecode_ast = Vec::new();
    while parser.current.is_some() {
        match parser.parse_expr() {
            Ok(expr) => bytecode_ast.push(expr),
            Err(e) => {
                eprintln!("PARSER_ERROR: {}", e);
                std::process::exit(AwsmErrorCode::PurityFail as i32);
            }
        }
    }

    let mut vm = AwsmVm::new();
    for expr in bytecode_ast {
        if let Err(e) = vm.eval_expr(&expr) {
            eprintln!("TRANSFORMATION_FAILURE: {:?}", e);
            std::process::exit(e.to_exit_code()); 
        }
    }

    match vm.quench() {
        Ok(json_payload) => {
            io::stdout().write_all(json_payload.as_bytes()).unwrap();
            std::process::exit(AwsmErrorCode::Success as i32);
        }
        Err(e) => {
            eprintln!("QUENCH_FAILURE: {:?}", e);
            std::process::exit(e.to_exit_code());
        }
    }
}