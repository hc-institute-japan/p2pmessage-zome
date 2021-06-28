use hdk::prelude::*;
use std::collections::HashMap;

use super::helpers::{get_receipts, get_replies, insert_message};
use crate::utils::try_from_element;

use super::{
    AgentMessages, FileType, MessageBundle, MessageContents, P2PMessage, P2PMessageFilterBatch,
    P2PMessageHashTables, P2PMessageReceipt, Payload, ReceiptContents,
};

pub fn get_next_batch_messages_handler(
    filter: P2PMessageFilterBatch,
) -> ExternResult<P2PMessageHashTables> {
    let queried_messages: Vec<Element> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    // agent_messages.insert(format!("{:?}", filter.conversant.clone()), Vec::new());
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    let mut reply_pairs: HashMap<String, String> = HashMap::new();

    let filter_timestamp = match filter.last_fetched_timestamp {
        Some(timestamp) => timestamp,
        None => {
            let now = sys_time()?;
            Timestamp(now.as_secs() as i64, 0)
        }
    };

    for message in queried_messages.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;

        if (message_entry.time_sent.0 <= filter_timestamp.0)
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
                        reply_pairs.insert(
                            if let Some(ref reply_to_hash) = message_entry.reply_to {
                                reply_to_hash.to_string()
                            } else {
                                "".to_string()
                            },
                            message_hash.to_string(),
                        );

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
                            reply_pairs.insert(
                                if let Some(ref reply_to_hash) = message_entry.reply_to {
                                    reply_to_hash.to_string()
                                } else {
                                    "".to_string()
                                },
                                message_hash.to_string(),
                            );

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
                            reply_pairs.insert(
                                if let Some(ref reply_to_hash) = message_entry.reply_to {
                                    reply_to_hash.to_string()
                                } else {
                                    "".to_string()
                                },
                                message_hash.to_string(),
                            );

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
                            reply_pairs.insert(
                                if let Some(ref reply_to_hash) = message_entry.reply_to {
                                    reply_to_hash.to_string()
                                } else {
                                    "".to_string()
                                },
                                message_hash.to_string(),
                            );

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
