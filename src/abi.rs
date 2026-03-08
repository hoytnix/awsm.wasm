#[link(wasm_import_module = "env")]
extern "C" {
    pub fn awsm_read_topology(ptr: *mut u8, len: i32) -> i32;
    pub fn awsm_validate_ast(ptr: *const u8, len: i32) -> i32;
    pub fn awsm_log_event(ptr: *const u8, len: i32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwsmErrorCode {
    Success = 0x00,
    ReadDenied = 0x10,
    PurityFail = 0x20,
    GasExhausted = 0x30,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    SeekVoid,
    AmbiguousPath,
    MarkExists,
    MarkNotFound,
    DepthExceeded,
    InvalidQuench,
    PayloadConstraintViolation,
    SyntaxError(String),
}

impl VmError {
    pub fn to_exit_code(&self) -> i32 {
        match self {
            VmError::DepthExceeded => AwsmErrorCode::GasExhausted as i32,
            _ => AwsmErrorCode::PurityFail as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_exit_code() {
        assert_eq!(VmError::DepthExceeded.to_exit_code(), AwsmErrorCode::GasExhausted as i32);
        assert_eq!(VmError::SeekVoid.to_exit_code(), AwsmErrorCode::PurityFail as i32);
    }
}
