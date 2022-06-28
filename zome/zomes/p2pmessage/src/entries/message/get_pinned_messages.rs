use hdk::prelude::*;
use std::collections::HashMap;

use super::helpers::{get_receipts, insert_message};

use super::{
    P2PMessage, P2PMessageData, P2PMessageHashTables, P2PMessagePin, P2PMessageReceipt, PinStatus,
};

pub fn get_pinned_messages_handler(conversant: AgentPubKey) -> ExternResult<P2PMessageHashTables> {
    let mut queried_pins: Vec<Record> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(3),
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut queried_messages: Vec<Record> = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    queried_messages.reverse();
    queried_pins.reverse();

    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    agent_messages.insert(conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, (P2PMessageData, Vec<String>)> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    let mut unpinned_messages: HashMap<String, P2PMessagePin> = HashMap::new();
    let mut pinned_messages: HashMap<String, P2PMessagePin> = HashMap::new();

    for pin in queried_pins.into_iter() {
        if let Ok(pin_entry) = TryInto::<P2PMessagePin>::try_into(pin.clone()) {
            let _pin_hash = hash_entry(&pin_entry)?;

            if pin_entry.conversants.contains(&conversant) {
                match pin_entry.status {
                    PinStatus::Pinned { timestamp: _ } => {
                        for message_hash in &pin_entry.id {
                            match unpinned_messages.get_mut(&message_hash.clone().to_string()) {
                                Some(_pin) => None,
                                None => pinned_messages
                                    .insert(message_hash.clone().to_string(), pin_entry.clone()),
                            };
                        }
                    }
                    PinStatus::Unpinned { timestamp: _ } => {
                        for message_hash in &pin_entry.id {
                            match pinned_messages.get_mut(&message_hash.clone().to_string()) {
                                Some(_pin) => None,
                                None => unpinned_messages
                                    .insert(message_hash.clone().to_string(), pin_entry.clone()),
                            };
                        }
                    }
                }
            }
        } else {
            continue;
        }
    }

    for message in queried_messages.into_iter() {
        if let Ok(message_entry) = TryInto::<P2PMessage>::try_into(message) {
            let message_hash: EntryHash = hash_entry(&message_entry)?;

            if pinned_messages.contains_key(&message_hash.clone().to_string()) {
                insert_message(
                    &mut agent_messages,
                    &mut message_contents,
                    message_entry.clone(),
                    message_hash,
                    message_entry.receiver.clone(),
                )?;
            }
        } else {
            continue;
        }
    }

    get_receipts(&mut message_contents, &mut receipt_contents)?;

    // Ok(P2PMessageHashTables(
    //     AgentMessages(agent_messages),
    //     MessageContents(message_contents),
    //     ReceiptContents(receipt_contents),
    // ))

    Ok(P2PMessageHashTables(
        agent_messages,
        message_contents,
        receipt_contents,
    ))
}
