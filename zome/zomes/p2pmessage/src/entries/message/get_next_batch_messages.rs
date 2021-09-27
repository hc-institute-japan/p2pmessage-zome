use hdk::prelude::*;
use std::collections::HashMap;

use super::helpers::{get_receipts, get_replies, insert_message, insert_reply};
// use crate::utils::try_from_element;

use super::{
    AgentMessages, FileType, MessageBundle, MessageContents, P2PMessage, P2PMessageFilterBatch,
    P2PMessageHashTables, P2PMessageReceipt, Payload, ReceiptContents,
};

// fn time_check(t1: Timestamp, t2: Timestamp) -> bool {
//     let second_diff = t2.0 - t1.0;
//     if second_diff > 0 {
//         return true;
//     } else if second_diff < 0 {
//         return false;
//     } else {
//         let nano_diff = t2.1 - t1.1;
//         if nano_diff > 0 {
//             return true;
//         } else {
//             return false;
//         }
//     }
// }

pub fn get_next_batch_messages_handler(
    filter: P2PMessageFilterBatch,
) -> ExternResult<P2PMessageHashTables> {
    let mut queried_messages: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;
    queried_messages.reverse();

    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    // agent_messages.insert(format!("{:?}", filter.conversant.clone()), Vec::new());
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    let mut reply_pairs: HashMap<String, Vec<String>> = HashMap::new();

    let filter_timestamp = match filter.last_fetched_timestamp {
        Some(timestamp) => timestamp,
        None => sys_time()?,
    };

    for message in queried_messages.into_iter() {
        let message_entry: P2PMessage = message.try_into()?;
        let message_hash = hash_entry(&message_entry)?;

        if (message_entry.time_sent.as_seconds_and_nanos().0 <= filter_timestamp.as_seconds_and_nanos().0)
        // if (time_check(message_entry.time_sent, filter_timestamp))
            && (match filter.last_fetched_message_id {
                Some(ref id) if *id == message_hash => false,
                Some(ref id) if *id != message_hash => true,
                _ => false,
            } || filter.last_fetched_message_id == None)
            && (message_entry.author == filter.conversant
                || message_entry.receiver == filter.conversant)
        {
            match message_entry.payload {
                Payload::Text { .. } => {
                    if filter.payload_type == "Text" || filter.payload_type == "All" {
                        if message_entry.reply_to != None {
                            insert_reply(
                                &mut reply_pairs,
                                message_entry.clone(),
                                message_hash.clone(),
                            );
                        }

                        let current_batch_size = insert_message(
                            &mut agent_messages,
                            &mut message_contents,
                            message_entry,
                            message_hash,
                            filter.conversant.clone(),
                        )?;

                        if current_batch_size >= filter.batch_size.into() {
                            break;
                        }
                    }
                }
                Payload::File { ref file_type, .. } => match file_type {
                    FileType::Image { .. } => {
                        if filter.payload_type == "Media"
                            || filter.payload_type == "File"
                            || filter.payload_type == "All"
                        {
                            if message_entry.reply_to != None {
                                insert_reply(
                                    &mut reply_pairs,
                                    message_entry.clone(),
                                    message_hash.clone(),
                                );
                            }

                            let current_batch_size = insert_message(
                                &mut agent_messages,
                                &mut message_contents,
                                message_entry,
                                message_hash,
                                filter.conversant.clone(),
                            )?;

                            if current_batch_size >= filter.batch_size.into() {
                                break;
                            }
                        }
                    }
                    FileType::Video { .. } => {
                        if filter.payload_type == "Media"
                            || filter.payload_type == "File"
                            || filter.payload_type == "All"
                        {
                            if message_entry.reply_to != None {
                                insert_reply(
                                    &mut reply_pairs,
                                    message_entry.clone(),
                                    message_hash.clone(),
                                );
                            }

                            let current_batch_size = insert_message(
                                &mut agent_messages,
                                &mut message_contents,
                                message_entry,
                                message_hash,
                                filter.conversant.clone(),
                            )?;

                            if current_batch_size >= filter.batch_size.into() {
                                break;
                            }
                        }
                    }
                    FileType::Other { .. } => {
                        if filter.payload_type == "Other"
                            || filter.payload_type == "File"
                            || filter.payload_type == "All"
                        {
                            if message_entry.reply_to != None {
                                insert_reply(
                                    &mut reply_pairs,
                                    message_entry.clone(),
                                    message_hash.clone(),
                                );
                            }

                            let current_batch_size = insert_message(
                                &mut agent_messages,
                                &mut message_contents,
                                message_entry,
                                message_hash,
                                filter.conversant.clone(),
                            )?;

                            if current_batch_size >= filter.batch_size.into() {
                                break;
                            }
                        }
                    }
                },
            }
        }
    }

    get_receipts(&mut message_contents, &mut receipt_contents)?;

    get_replies(&mut reply_pairs, &mut message_contents)?;

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages),
        MessageContents(message_contents),
        ReceiptContents(receipt_contents),
    ))
}
