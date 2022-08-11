use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_integrity_types::*;

pub fn get_file_bytes_handler(
    file_hashes: Vec<EntryHash>,
) -> ExternResult<HashMap<String, P2PFileBytes>> {
    let queried_files: Vec<Record> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(2),
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut files: HashMap<String, P2PFileBytes> = HashMap::new();

    for file in queried_files.into_iter() {
        if let Ok(file_entry) = TryInto::<P2PFileBytes>::try_into(file.clone()) {
            let file_hash = hash_entry(&file_entry)?;

            if file_hashes.contains(&file_hash) {
                match files.get(&file_hash.clone().to_string()) {
                    Some(_file) => continue,
                    _ => {
                        files.insert(file_hash.to_string(), file_entry);
                        ()
                    }
                }
            }
        } else {
            continue;
        }
    }

    Ok(files)
}
