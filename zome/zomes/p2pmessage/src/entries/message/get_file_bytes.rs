use hdk::prelude::*;
use std::collections::HashMap;

use crate::utils::try_from_element;

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

    debug!(
        "{}",
        format!("the query results {:?}", queried_files.clone())
    );

    let mut files: HashMap<String, P2PFileBytes> = HashMap::new();

    for file in queried_files.into_iter() {
        let file_entry: P2PFileBytes = try_from_element(file.clone())?;
        let file_hash = hash_entry(&file_entry)?;

        debug!(
            "{}",
            format!("the element file hash is {:?}", file_hash.clone())
        );

        debug!("{}", format!("the file entry is {:?}", file_entry.clone()));

        // if  file_hash is being requested
        if file_hashes.contains(&file_hash) {
            // check if file is in return hash map
            match files.get(&file_hash.clone().to_string()) {
                Some(_file) => continue,
                _ => {
                    // let file_entry = try_from_element(file)?;
                    files.insert(file_hash.to_string(), file_entry);
                    ()
                }
            }
        }
    }

    // debug!(format!(
    //     "file_hashes {:?}, queried_files {:?}, hashmap {:?}",
    //     file_hashes, queried, files
    // ));

    Ok(FileContents(files))
}
