#[cfg(test)]
mod tests {
    use crate::abi::AwsmErrorCode;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::vm::AwsmVm;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    #[derive(Deserialize)]
    struct Fixture {
        name: String,
        description: String,
        input_awsm: String,
        expected_exit_code: i32,
        expected_pws: Option<serde_json::Value>,
    }

    #[test]
    fn test_bh_cts_runner() {
        let fixtures_dir = Path::new("tests/fixtures");
        let entries = fs::read_dir(fixtures_dir).expect("Failed to read tests/fixtures dir");

        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Failed to read fixture: {:?}", path));
            let fixture: Fixture = serde_yaml::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse fixture {:?}: {}", path, e));

            println!("Running fixture: {}", fixture.name);
            println!("Description: {}", fixture.description);

            let mut lexer = Lexer::new(&fixture.input_awsm);
            let tokens = match lexer.tokenize() {
                Ok(t) => t,
                Err(e) => {
                    assert_eq!(
                        fixture.expected_exit_code,
                        AwsmErrorCode::PurityFail as i32,
                        "[{}] Lexer failed with '{}', but expected exit code {}",
                        fixture.name, e, fixture.expected_exit_code
                    );
                    assert!(fixture.expected_pws.is_none() || fixture.expected_pws == Some(serde_json::Value::Null));
                    continue;
                }
            };

            let mut parser = Parser::new(tokens);
            let mut bytecode_ast = Vec::new();
            let mut parser_failed = false;

            while parser.current.is_some() {
                match parser.parse_expr() {
                    Ok(expr) => bytecode_ast.push(expr),
                    Err(e) => {
                        assert_eq!(
                            fixture.expected_exit_code,
                            AwsmErrorCode::PurityFail as i32,
                            "[{}] Parser failed with '{}', but expected exit code {}",
                            fixture.name, e, fixture.expected_exit_code
                        );
                        assert!(fixture.expected_pws.is_none() || fixture.expected_pws == Some(serde_json::Value::Null));
                        parser_failed = true;
                        break;
                    }
                }
            }
            if parser_failed {
                continue;
            }

            let mut vm = AwsmVm::new();
            println!("AST: {:?}", bytecode_ast); 
            let mut eval_failed = None;

            for expr in bytecode_ast {
                if let Err(e) = vm.eval_expr(&expr) {
                    eval_failed = Some(e);
                    break;
                }
            }

            if let Some(e) = eval_failed {
                let actual_exit_code = e.to_exit_code();
                assert_eq!(
                    actual_exit_code,
                    fixture.expected_exit_code,
                    "[{}] Execution failed with exit code {} but expected {}",
                    fixture.name, actual_exit_code, fixture.expected_exit_code
                );
                assert!(
                    fixture.expected_pws.is_none() || fixture.expected_pws == Some(serde_json::Value::Null),
                    "[{}] Execution failed but expected Some PWS",
                    fixture.name
                );
                continue;
            }

            match vm.quench() {
                Ok(json_payload) => {
                    assert_eq!(
                        AwsmErrorCode::Success as i32,
                        fixture.expected_exit_code,
                        "[{}] Unexpected success (exit code 0), but expected {}",
                        fixture.name, fixture.expected_exit_code
                    );
                    let actual_pws: serde_json::Value = serde_json::from_str(&json_payload).unwrap();
                    let expected_pws = fixture.expected_pws.unwrap_or(serde_json::Value::Null);
                    assert_eq!(actual_pws, expected_pws, "[{}] PWS mismatch", fixture.name);
                }
                Err(e) => {
                    let actual_exit_code = e.to_exit_code();
                    assert_eq!(
                        actual_exit_code,
                        fixture.expected_exit_code,
                        "[{}] Quench failed with exit code {} but expected {}",
                        fixture.name, actual_exit_code, fixture.expected_exit_code
                    );
                    assert!(fixture.expected_pws.is_none() || fixture.expected_pws == Some(serde_json::Value::Null));
                }
            }
        }
    }
}
