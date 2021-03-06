use hdk::prelude::*;
use std::collections::HashMap;

use super::helpers::{get_receipts, get_replies, insert_message, insert_reply};
use crate::utils::try_from_element;

use super::{
    AgentMessages, MessageBundle, MessageContents, P2PMessage, P2PMessageFilterAgentTimestamp,
    P2PMessageHashTables, P2PMessageReceipt, Payload, ReceiptContents,
};

pub fn get_messages_by_agent_by_timestamp_handler(
    filter: P2PMessageFilterAgentTimestamp,
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
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    let mut reply_pairs: HashMap<String, Vec<String>> = HashMap::new();

    let day_start = (filter.date.0 / 86400) * 86400;
    let day_end = day_start + 86399;

    for message in queried_messages.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;

        // TODO: use header timestamp for message_time
        if message_entry.time_sent.0 >= day_start
            && message_entry.time_sent.0 <= day_end
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
                        insert_message(
                            &mut agent_messages,
                            &mut message_contents,
                            message_entry,
                            message_hash,
                            filter.conversant.clone(),
                        )?;
                    }
                }
                Payload::File { .. } => {
                    if filter.payload_type == "File" || filter.payload_type == "All" {
                        if message_entry.reply_to != None {
                            insert_reply(
                                &mut reply_pairs,
                                message_entry.clone(),
                                message_hash.clone(),
                            );
                        }

                        insert_message(
                            &mut agent_messages,
                            &mut message_contents,
                            message_entry,
                            message_hash,
                            filter.conversant.clone(),
                        )?;
                    }
                }
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
