use hdk::prelude::*;
use std::collections::HashMap;

use crate::utils::try_from_element;
use super::helpers::insert_message;
use super::helpers::get_receipts;

use super::{

    Payload,
    P2PMessage,
    P2PMessageReceipt,
    P2PMessageHashTables,
    P2PMessageFilterAgentTimestamp,
    MessageBundle,
    AgentMessages,
    MessageContents,
    ReceiptContents,

};


pub fn handler( filter: P2PMessageFilterAgentTimestamp ) -> ExternResult<P2PMessageHashTables> {
    
    let queried_messages:Vec<Element> = query(
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

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages),
        MessageContents(message_contents),
        ReceiptContents(receipt_contents),
    ))
}