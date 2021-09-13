use hdk::prelude::*;

pub fn _try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    let element_entry = element.clone().into_inner().1;
    debug!(
        "nicko utils the element entry is {:?}",
        element_entry.clone()
    );
    match element_entry {
        element::ElementEntry::Present(entry) => _try_from_entry::<T>(entry.clone()),
        _ => {
            debug!("nicko try from element error {:?}", element.entry());
            crate::error("Sorry, something went wrong. [Conversion error]")
        }
    }
}

pub fn _try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    let entry_copy = entry.clone();
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => {
                debug!("nicko try from entry error inner {:?}", entry_copy);
                crate::error("Sorry, something went wrong. [Conversion error]")
            }
        },
        _ => {
            debug!("nicko try from entry error outer {:?}", entry_copy);
            crate::error("Sorry, something went wrong. [Conversion error]")
        }
    }
}

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(WasmError::Guest(String::from(reason)))
}
