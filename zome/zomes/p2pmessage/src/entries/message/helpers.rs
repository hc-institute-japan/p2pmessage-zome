use hdk::prelude::*;
use std::collections::HashMap;

use crate::utils::try_from_element;

use super::{MessageBundle, P2PMessage, P2PMessageReceipt, ReceiptContents, Status};

pub fn insert_message(
    agent_messages: &mut HashMap<String, Vec<String>>,
    message_contents: &mut HashMap<String, MessageBundle>,
    message_entry: P2PMessage,
    message_hash: EntryHash,
    key: AgentPubKey,
) -> ExternResult<usize> {
    let mut message_array_length = 0;
    match agent_messages.get_mut(&key.to_string()) {
        Some(messages) => {
            messages.push(message_hash.clone().to_string());
            message_array_length = messages.len();
        }
        None => {
            agent_messages.insert(key.to_string(), vec![message_hash.clone().to_string()]);
        }
    };
    message_contents.insert(
        message_hash.to_string(),
        MessageBundle(message_entry, Vec::new()),
    );

    Ok(message_array_length)
}

pub fn get_receipts(
    message_contents: &mut HashMap<String, MessageBundle>,
    receipt_contents: &mut HashMap<String, P2PMessageReceipt>,
) -> ExternResult<()> {
    let queried_receipts: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(1),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for receipt in queried_receipts.clone().into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;

        for message_id in receipt_entry.id.clone().into_iter() {
            if message_contents.contains_key(&message_id.to_string()) {
                receipt_contents.insert(receipt_hash.clone().to_string(), receipt_entry.clone());
                if let Some(message_bundle) = message_contents.get_mut(&message_id.to_string()) {
                    message_bundle.1.push(receipt_hash.to_string())
                };
            }
        }
    }

    Ok(())
}

pub fn _commit_receipts(receipts: Vec<P2PMessageReceipt>) -> ExternResult<ReceiptContents> {
    // Query all the receipts
    let query_result: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(1),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    // Get all receipts from query result
    let all_receipts = query_result
        .into_iter()
        .filter_map(|el| {
            if let Ok(Some(receipt)) = el.into_inner().1.to_app_option::<P2PMessageReceipt>() {
                return Some(receipt);
            } else {
                None
            }
        })
        .collect::<Vec<P2PMessageReceipt>>();

    // initialize hash map that will be returned
    let mut receipts_hash_map: HashMap<String, P2PMessageReceipt> = HashMap::new();

    // Iterate through the receipts in the argument and push them into the hash map
    receipts.clone().into_iter().for_each(|receipt| {
        if let Ok(hash) = hash_entry(&receipt) {
            receipts_hash_map.insert(hash.to_string(), receipt);
        }
    });

    // Iterate through the receipts to check if the receipt has been committed, remove them from the hash map if it is
    // used for loops instead of for_each because you cant break iterators
    for i in 0..all_receipts.len() {
        let receipt = all_receipts[i].clone();
        let hash = hash_entry(&receipt)?;

        if receipts_hash_map.contains_key(&hash.to_string()) {
            if let Status::Read { timestamp: _ } = receipt.status {
                receipts_hash_map.remove(&hash.to_string());
            }
        }

        if receipts_hash_map.is_empty() {
            break;
        }
    }

    // iterate the remaining contents of the hashmap
    receipts_hash_map
        .clone()
        .into_iter()
        .for_each(|(_entry_hash, receipt)| {
            create_entry(&receipt).expect("Expected P2P message receipt entry");
        });

    Ok(ReceiptContents(receipts_hash_map))
}
