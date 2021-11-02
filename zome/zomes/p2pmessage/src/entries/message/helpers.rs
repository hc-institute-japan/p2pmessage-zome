use hdk::prelude::*;
use std::collections::HashMap;

use super::{P2PFileBytes, P2PMessage, P2PMessageData, P2PMessageReceipt, P2PMessageReplyTo};

use crate::utils::error;

pub fn insert_message(
    agent_messages: &mut HashMap<String, Vec<String>>,
    message_contents: &mut HashMap<String, (P2PMessageData, Vec<String>)>,
    message_entry: P2PMessage,
    message_hash: EntryHash,
    key: AgentPubKey,
) -> ExternResult<usize> {
    let mut message_array_length = 0;
    match agent_messages.get_mut(&key.clone().to_string()) {
        Some(messages) => {
            messages.push(message_hash.clone().to_string());
            message_array_length = messages.len();
        }
        None => {
            agent_messages.insert(key.to_string(), vec![message_hash.clone().to_string()]);
        }
    };
    let message_data = P2PMessageData {
        author: message_entry.author,
        receiver: message_entry.receiver,
        payload: message_entry.payload,
        time_sent: message_entry.time_sent,
        reply_to: None,
    };
    message_contents.insert(message_hash.to_string(), (message_data, Vec::new()));

    Ok(message_array_length)
}

pub fn insert_reply(
    reply_pairs: &mut HashMap<String, Vec<String>>,
    message_entry: P2PMessage,
    message_hash: EntryHash,
) -> () {
    if let Some(ref reply_to_hash) = message_entry.reply_to {
        match reply_pairs.get_mut(&reply_to_hash.clone().to_string()) {
            Some(message_hashes) => {
                message_hashes.push(message_hash.clone().to_string());
            }
            None => {
                reply_pairs.insert(
                    reply_to_hash.clone().to_string(),
                    vec![message_hash.clone().to_string()],
                );
            }
        }
    }
}

pub fn get_receipts(
    message_contents: &mut HashMap<String, (P2PMessageData, Vec<String>)>,
    receipt_contents: &mut HashMap<String, P2PMessageReceipt>,
) -> ExternResult<()> {
    let queried_receipts: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(1),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for receipt in queried_receipts.into_iter() {
        if let Ok(receipt_entry) = TryInto::<P2PMessageReceipt>::try_into(receipt) {
            let receipt_hash = hash_entry(&receipt_entry)?;
            // distribute this receipt to every messsage it belongs to
            for message_id in receipt_entry.id.clone().into_iter() {
                if message_contents.contains_key(&message_id.clone().to_string()) {
                    receipt_contents
                        .insert(receipt_hash.clone().to_string(), receipt_entry.clone());
                    if let Some(message_bundle) =
                        message_contents.get_mut(&message_id.clone().to_string())
                    {
                        message_bundle.1.push(receipt_hash.clone().to_string())
                    };
                }
            }
        } else {
            continue;
        }
    }

    Ok(())
}

pub fn get_replies(
    reply_pairs: &mut HashMap<String, Vec<String>>,
    message_contents: &mut HashMap<String, (P2PMessageData, Vec<String>)>,
) -> ExternResult<()> {
    let queried_messages: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for message in queried_messages.clone().into_iter() {
        if let Ok(message_entry) = TryInto::<P2PMessage>::try_into(message) {
            let message_hash = hash_entry(&message_entry)?;

            // iterating over all p2pmesssages, if the message has been replied to
            if reply_pairs.contains_key(&message_hash.clone().to_string()) {
                match reply_pairs.get(&message_hash.clone().to_string()) {
                    Some(message_hashes) => {
                        // build reply_to data
                        let replied_to_message = P2PMessageReplyTo {
                            hash: message_hash.clone(),
                            author: message_entry.author,
                            receiver: message_entry.receiver,
                            payload: message_entry.payload,
                            time_sent: message_entry.time_sent,
                            reply_to: None,
                        };

                        for reply_hash in message_hashes {
                            // append reply_to data to reply
                            if let Some(message_bundle) =
                                message_contents.get_mut(&reply_hash.to_string())
                            //b64 check
                            {
                                message_bundle.0.reply_to = Some(replied_to_message.clone())
                            }
                        }
                    }
                    None => continue,
                }
            }
        } else {
            continue;
        }
    }

    Ok(())
}

pub fn get_message_from_chain(hash: EntryHash) -> ExternResult<P2PMessage> {
    let mut queried_messages: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;
    queried_messages.reverse();

    for element in queried_messages.into_iter() {
        // let element_header = element.clone().header_address();
        // let element_entry = element.clone().entry();
        let message_entry = TryInto::<P2PMessage>::try_into(element.clone())?;
        let message_hash = hash_entry(message_entry.clone())?;
        // match hash {
        //     Header => {
        //         if hash == element_header {
        //             let message_entry = TryInto::<P2PMessage>::try_into(element.clone())?;
        //             return Ok(message_entry);
        //         }
        //     }
        //     Entry => {
        //         let message_entry = TryInto::<P2PMessage>::try_into(element.clone())?;
        //         let entry_hash = hash_entry(message_entry.clone())?;
        //         if entry_hash == hash.into() {
        //             return Ok(message_entry);
        //         }
        //     }
        // }
        if hash == message_hash {
            return Ok(message_entry);
        }
    }

    return error("Sorry. Entry not found.");
}

pub fn get_file_from_chain(file_hash: EntryHash) -> ExternResult<P2PFileBytes> {
    let queried_files: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(2),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for file in queried_files.into_iter() {
        if let Ok(file_entry) = TryInto::<P2PFileBytes>::try_into(file.clone()) {
            let entry_hash = hash_entry(&file_entry)?;

            if entry_hash == file_hash {
                return Ok(file_entry);
            }
        } else {
            continue;
        }
    }
    return error("Sorry. File not found.");
}

#[allow(dead_code)]
pub fn get_receipt_from_chain(receipt_hash: EntryHash) -> ExternResult<P2PMessageReceipt> {
    let queried_receipts: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(1),
                zome_info()?.id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for receipt in queried_receipts.into_iter() {
        if let Ok(receipt_entry) = TryInto::<P2PMessageReceipt>::try_into(receipt.clone()) {
            let entry_hash = hash_entry(&receipt_entry)?;

            if entry_hash == receipt_hash {
                return Ok(receipt_entry);
            }
        } else {
            continue;
        }
    }
    return error("Sorry. Receipt not found.");
}

// pub fn _commit_receipts(receipts: Vec<P2PMessageReceipt>) -> ExternResult<ReceiptContents> {
//     // Query all the receipts
//     let query_result: Vec<Element> = query(
//         QueryFilter::new()
//             .entry_type(EntryType::App(AppEntryType::new(
//                 EntryDefIndex::from(1),
//                 zome_info()?.zome_id,
//                 EntryVisibility::Private,
//             )))
//             .include_entries(true),
//     )?;

//     // Get all receipts from query result
//     let all_receipts = query_result
//         .into_iter()
//         .filter_map(|el| {
//             if let Ok(Some(receipt)) = el.into_inner().1.to_app_option::<P2PMessageReceipt>() {
//                 return Some(receipt);
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<P2PMessageReceipt>>();

//     // initialize hash map that will be returned
//     let mut receipts_hash_map: HashMap<String, P2PMessageReceipt> = HashMap::new();

//     // Iterate through the receipts in the argument and push them into the hash map
//     receipts.clone().into_iter().for_each(|receipt| {
//         if let Ok(hash) = hash_entry(&receipt) {
//             receipts_hash_map.insert(hash.to_string(), receipt);
//         }
//     });

//     // Iterate through the receipts to check if the receipt has been committed, remove them from the hash map if it is
//     // used for loops instead of for_each because you cant break iterators
//     for i in 0..all_receipts.len() {
//         let receipt = all_receipts[i].clone();
//         let hash = hash_entry(&receipt)?;

//         if receipts_hash_map.contains_key(&hash.clone().to_string()) {
//             if let Status::Read { timestamp: _ } = receipt.status {
//                 receipts_hash_map.remove(&hash.clone().to_string());
//             }
//         }

//         if receipts_hash_map.is_empty() {
//             break;
//         }
//     }

//     // iterate the remaining contents of the hashmap
//     receipts_hash_map
//         .clone()
//         .into_iter()
//         .for_each(|(_entry_hash, receipt)| {
//             create_entry(&receipt).expect("Expected P2P message receipt entry");
//         });

//     Ok(ReceiptContents(receipts_hash_map))
// }
