use std::collections::HashMap;
use crate::lexer::Expr;
use crate::sph::Pointer;
use crate::abi::VmError;
use crate::pws::{Action, MutateCallPayload, PatchiestWireSchema};

fn parse_kwargs<'a>(args: &'a [Expr]) -> Result<HashMap<String, Vec<&'a Expr>>, VmError> {
    let mut kwargs = HashMap::new();
    let mut current_kw = None;
    for arg in args {
        if let Expr::Keyword(kw) = arg {
            current_kw = Some(kw.clone());
            kwargs.entry(kw.clone()).or_insert_with(Vec::new);
        } else if let Some(kw) = &current_kw {
            kwargs.get_mut(kw).unwrap().push(arg);
        }
    }
    Ok(kwargs)
}

fn get_string(kwargs: &HashMap<String, Vec<&Expr>>, key: &str) -> Option<String> {
    if let Some(vals) = kwargs.get(key) {
        if let Some(Expr::String(s)) = vals.first() {
            return Some(s.clone());
        }
    }
    None
}

fn get_bool(kwargs: &HashMap<String, Vec<&Expr>>, key: &str) -> Option<bool> {
    if let Some(vals) = kwargs.get(key) {
        if let Some(Expr::Bool(b)) = vals.first() {
            return Some(*b);
        }
    }
    None
}

fn get_usize(kwargs: &HashMap<String, Vec<&Expr>>, key: &str) -> Option<usize> {
    if let Some(vals) = kwargs.get(key) {
        if let Some(Expr::Symbol(s)) = vals.first() {
            return s.parse::<usize>().ok();
        }
    }
    None
}

fn get_string_array(kwargs: &HashMap<String, Vec<&Expr>>, key: &str) -> Option<Vec<String>> {
    if let Some(vals) = kwargs.get(key) {
        let mut arr = Vec::new();
        let mut current_str = String::new();
        let mut in_string = false;

        for val in vals {
            match val {
                Expr::String(s) => {
                    arr.push(s.clone());
                }
                Expr::Symbol(raw_s) => {
                    let s = raw_s.replace("[", "").replace("]", "");
                    if s.is_empty() { continue; }

                    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                        arr.push(s[1..s.len()-1].to_string());
                    } else if s.starts_with('"') {
                        in_string = true;
                        current_str = s[1..].to_string();
                    } else if s.ends_with('"') {
                        in_string = false;
                        if !current_str.is_empty() {
                            current_str.push(' ');
                        }
                        current_str.push_str(&s[..s.len()-1]);
                        arr.push(current_str.clone());
                        current_str.clear();
                    } else if in_string {
                        current_str.push(' ');
                        current_str.push_str(&s);
                    }
                }
                _ => {}
            }
        }
        if !arr.is_empty() { return Some(arr); }
    }
    None
}

fn get_string_map(kwargs: &HashMap<String, Vec<&Expr>>, key: &str) -> Option<HashMap<String, String>> {
    if let Some(vals) = kwargs.get(key) {
        if let Some(Expr::Map(m)) = vals.first() {
            let mut smap = HashMap::new();
            for (k, v) in m {
                if let Expr::String(s) = v {
                    smap.insert(k.clone(), s.clone());
                }
            }
            return Some(smap);
        }
    }
    None
}

pub struct AwsmVm {
    current_pointer: Pointer,
    anchors: HashMap<String, Pointer>,
    transaction_stack: Vec<Vec<Action>>, 
}

impl AwsmVm {
    pub fn new() -> Self {
        Self {
            current_pointer: Pointer("urn:awsm:ROOT".to_string()),
            anchors: HashMap::new(),
            transaction_stack: vec![Vec::new()],
        }
    }

    /// Evaluates the fully parsed AOT execution graph
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<(), VmError> {
        match expr {
            Expr::List(list) => {
                if list.is_empty() { return Ok(()); }
                let head = &list[0];

                if let Expr::Symbol(sym) = head {
                    match sym.as_str() {
                        "[" => {
                            if self.transaction_stack.len() > 32 { return Err(VmError::DepthExceeded); }
                            self.transaction_stack.push(Vec::new());
                        }
                        "]" => {
                            if self.transaction_stack.len() <= 1 { return Err(VmError::InvalidQuench); }
                            let completed = self.transaction_stack.pop().unwrap();
                            self.transaction_stack.last_mut().unwrap().extend(completed);
                        }
                        "<" => self.current_pointer = Pointer("urn:awsm:ROOT".to_string()),
                        "awsm:pipe" => {
                            for e in &list[1..] { self.eval_expr(e)?; }
                        }
                        "awsm:manage-import" => {
                            let kwargs = parse_kwargs(&list[1..])?;
                            self.transaction_stack.last_mut().unwrap().push(Action::ManageImport {
                                target_import: get_string(&kwargs, "target-import"),
                                ensure: get_string(&kwargs, "ensure"),
                                module: get_string(&kwargs, "module"),
                                replace_with: get_string(&kwargs, "replace-with"),
                                is_type_only: get_bool(&kwargs, "is-type").unwrap_or(false),
                            });
                        }
                        "awsm:mutate-call" => {
                            let kwargs = parse_kwargs(&list[1..])?;
                            let payload = MutateCallPayload {
                                rename: get_string(&kwargs, "rename"),
                                inject_args: get_string_map(&kwargs, "inject"),
                                target_arg_index: get_usize(&kwargs, "target-arg-index"),
                            };
                            payload.validate()?;
                            self.transaction_stack.last_mut().unwrap().push(Action::MutateCall {
                                target_urn: self.current_pointer.clone(),
                                payload,
                            });
                        }
                        "awsm:translate-dialect" => {
                            let kwargs = parse_kwargs(&list[1..])?;
                            let enforce_explicit_type = get_string(&kwargs, "enforce-explicit-type")
                                .ok_or_else(|| VmError::PayloadConstraintViolation)?;
                            self.transaction_stack.last_mut().unwrap().push(Action::TranslateDialect {
                                target_urn: self.current_pointer.clone(),
                                enforce_explicit_type,
                                generate_interface: get_string_array(&kwargs, "generate-interface"),
                                target_param_index: get_usize(&kwargs, "target-param-index"),
                            });
                        }
                        "awsm:restructure-topology" => {
                            let kwargs = parse_kwargs(&list[1..])?;
                            let hardcoded_dependency = get_string(&kwargs, "hardcoded-dependency")
                                .ok_or_else(|| VmError::PayloadConstraintViolation)?;
                            let extract_to_parameter = get_string(&kwargs, "extract-to-parameter")
                                .ok_or_else(|| VmError::PayloadConstraintViolation)?;
                            self.transaction_stack.last_mut().unwrap().push(Action::RestructureTopology {
                                target_urn: self.current_pointer.clone(),
                                hardcoded_dependency,
                                extract_to_parameter,
                            });
                        }
                        _ => {} // Ignore unknown symbols in PoC
                    }
                }
                Ok(())
            }
            _ => Ok(()) // Ignore lone literals in PoC execution
        }
    }

    pub fn quench(mut self) -> Result<String, VmError> {
        if self.transaction_stack.len() != 1 { return Err(VmError::InvalidQuench); }
        let pws = PatchiestWireSchema {
            v: 1,
            tx_id: "pending_tx".to_string(),
            actions: self.transaction_stack.pop().unwrap(),
            purity_hash: "verified-awsm-state".to_string(),
        };
        serde_json::to_string(&pws).map_err(|_| VmError::InvalidQuench)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_coverage() {
        let mut vm = AwsmVm::new();
        // Test depth exceeded
        for _ in 0..33 {
            vm.transaction_stack.push(Vec::new());
        }
        assert!(vm.eval_expr(&Expr::List(vec![Expr::Symbol("[".to_string())])).is_err());
        
        let mut vm = AwsmVm::new();
        assert!(vm.eval_expr(&Expr::List(vec![Expr::Symbol("]".to_string())])).is_err()); // invalid quench
        
        // Test restructure-topology
        let e = Expr::List(vec![
            Expr::Symbol("awsm:restructure-topology".to_string()),
            Expr::Keyword("hardcoded-dependency".to_string()),
            Expr::String("dep".to_string()),
            Expr::Keyword("extract-to-parameter".to_string()),
            Expr::String("param".to_string()),
        ]);
        assert!(vm.eval_expr(&e).is_ok());

        let e_fail = Expr::List(vec![
            Expr::Symbol("awsm:restructure-topology".to_string()),
            Expr::Keyword("hardcoded-dependency".to_string()),
            Expr::String("dep".to_string()),
        ]);
        assert!(vm.eval_expr(&e_fail).is_err());
        
        // coverage for isolated branch
        assert!(vm.eval_expr(&Expr::Symbol("lone".to_string())).is_ok());

        // Array string coverage
        let e = Expr::List(vec![
            Expr::Symbol("awsm:translate-dialect".to_string()),
            Expr::Keyword("enforce-explicit-type".to_string()),
            Expr::String("bool".to_string()),
            Expr::Keyword("generate-interface".to_string()),
            Expr::Symbol("\"alone\"".to_string()),
            Expr::Symbol("\"start".to_string()),
            Expr::Symbol("middle".to_string()),
            Expr::Symbol("end\"".to_string()),
        ]);
        assert!(vm.eval_expr(&e).is_ok());
        
        // Scope push/pop coverage (quench)
        let mut vm = AwsmVm::new();
        assert!(vm.eval_expr(&Expr::List(vec![Expr::Symbol("[".to_string())])).is_ok());
        assert!(vm.eval_expr(&Expr::List(vec![Expr::Symbol("]".to_string())])).is_ok());
        assert!(vm.quench().is_ok());
    }
}
