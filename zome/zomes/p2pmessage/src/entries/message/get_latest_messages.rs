use hdk::prelude::*;
use std::collections::HashMap;

use crate::utils::try_from_element;
use super::helpers::insert_message;
use super::helpers::get_receipts;

use super::{
    MessageBundle,
    BatchSize,
    P2PMessageHashTables,
    P2PMessageReceipt,
    P2PMessage,
    AgentMessages,
    MessageContents,
    ReceiptContents,

};

pub fn handler(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
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
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    for message in queried_messages.into_iter() {
        
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash:EntryHash = hash_entry(&message_entry)?;


        if message_entry.author.clone() == agent_info()?.agent_latest_pubkey {
            // match agent_messages.get(&format!("{:?}", message_entry.receiver.clone())) {
            match agent_messages.get(&message_entry.receiver.clone().to_string()) {
                Some(messages) if messages.len() >= batch_size.0.into() => continue,
                Some(messages) if messages.len() < batch_size.0.into() => {


                    insert_message(
                        &mut agent_messages,
                        &mut message_contents,
                        message_entry.clone(),
                        message_hash,
                        message_entry.receiver.clone(),
                    )?;
                }
                _ => {
                    insert_message(
                        &mut agent_messages,
                        &mut message_contents,
                        message_entry.clone(),
                        message_hash,
                        message_entry.receiver.clone(),
                    )?;
                }
            }
        } else {
            // add this message to author's array in hashmap
            // match agent_messages.get(&format!("{:?}", message_entry.author.clone())) {
            match agent_messages.get(&message_entry.author.clone().to_string()) {
                Some(messages) if messages.len() >= batch_size.0.into() => continue,
                Some(messages) if messages.len() < batch_size.0.into() => {
                    insert_message(
                        &mut agent_messages,
                        &mut message_contents,
                        message_entry.clone(),
                        message_hash,
                        message_entry.author.clone(),
                    )?;
                }
                _ => {
                    insert_message(
                        &mut agent_messages,
                        &mut message_contents,
                        message_entry.clone(),
                        message_hash,
                        message_entry.author.clone(),
                    )?;
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