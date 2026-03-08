use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::sph::Pointer;
use crate::abi::VmError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PatchiestWireSchema {
    pub v: u8, // v1.1 schema still utilizes major version 1
    pub tx_id: String,
    pub actions: Vec<Action>,
    pub purity_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "MUTATE_CALL")]
    MutateCall {
        target_urn: Pointer,
        payload: MutateCallPayload,
    },
    #[serde(rename = "MANAGE_IMPORT")]
    ManageImport {
        #[serde(skip_serializing_if = "Option::is_none")]
        target_import: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ensure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        module: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        replace_with: Option<String>,
        is_type_only: bool,
    },
    #[serde(rename = "TRANSLATE_DIALECT")]
    TranslateDialect {
        target_urn: Pointer,
        enforce_explicit_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        generate_interface: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        target_param_index: Option<usize>,
    },
    #[serde(rename = "RESTRUCTURE_TOPOLOGY")]
    RestructureTopology {
        target_urn: Pointer,
        hardcoded_dependency: String,
        extract_to_parameter: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MutateCallPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_args: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arg_index: Option<usize>,
}

impl MutateCallPayload {
    pub fn validate(&self) -> Result<(), VmError> {
        if self.rename.is_none() && self.inject_args.is_none() {
            Err(VmError::PayloadConstraintViolation)
        } else {
            Ok(())
        }
    }
}
