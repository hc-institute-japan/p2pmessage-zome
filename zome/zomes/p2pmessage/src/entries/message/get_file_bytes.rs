use hdk::prelude::*;
use std::collections::HashMap;

use super::{FileContents, P2PFileBytes};

pub fn get_file_bytes_handler(file_hashes: Vec<EntryHash>) -> ExternResult<FileContents> {
    let queried_files: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(2),
                zome_info()?.zome_id,
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
            debug!("The hidden entry is {:?}", file);
            continue;
        }
    }

    Ok(FileContents(files))
}
