use hdk::prelude::*;

pub fn _try_from_record<T: TryFrom<SerializedBytes>>(record: Record) -> ExternResult<T> {
    let record_entry = record.clone().into_inner().1;
    match record_entry {
        record::RecordEntry::Present(entry) => _try_from_entry::<T>(entry.clone()),
        _ => crate::error("Sorry, something went wrong. [Conversion error]"),
    }
}

pub fn _try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Sorry, something went wrong. [Conversion error]"),
        },
        _ => crate::error("Sorry, something went wrong. [Conversion error]"),
    }
}

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(String::from(reason))))
}
