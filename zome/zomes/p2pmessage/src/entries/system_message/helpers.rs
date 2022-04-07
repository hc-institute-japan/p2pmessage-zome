use hdk::prelude::*;

use super::SystemMessage;

pub fn get_system_message_from_chain(
    system_message_hash: EntryHash,
) -> ExternResult<SystemMessage> {
    let queried_system_messages: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(3),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for element in queried_system_messages.into_iter() {
        if let Ok(system_message_entry) = TryInto::<SystemMessage>::try_into(element.clone()) {
            let entry_hash = hash_entry(&system_message_entry)?;

            if entry_hash == system_message_hash {
                return Ok(system_message_entry);
            }
        } else {
            continue;
        }
    }
    return error("Sorry. System message entry for hash not found.");
}
