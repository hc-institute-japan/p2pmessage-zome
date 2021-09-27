use hdk::prelude::*;

pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    let element_entry = element.clone().into_inner().1;
    match element_entry {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => crate::error("Sorry, something went wrong. [Conversion error]"),
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Sorry, something went wrong. [Conversion error]"),
        },
        _ => crate::error("Sorry, something went wrong. [Conversion error]"),
    }
}

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(WasmError::Guest(String::from(reason)))
}
