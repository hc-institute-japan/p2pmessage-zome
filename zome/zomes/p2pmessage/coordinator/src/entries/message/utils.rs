use hdk::prelude::*;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(String::from(reason))))
}

pub fn _err<T>(code: &str, message: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(format!(
        "{{\"code\": \"{}\", \"message\": \"{}\"}}",
        code, message
    ))))
}