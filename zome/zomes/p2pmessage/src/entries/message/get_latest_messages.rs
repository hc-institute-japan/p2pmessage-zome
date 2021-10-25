use hdk::prelude::*;
use std::collections::HashMap;

use super::helpers::{get_receipts, get_replies, insert_message, insert_reply};

use super::{P2PMessage, P2PMessageData, P2PMessageHashTables, P2PMessageReceipt};

pub fn get_latest_messages_handler(batch_size: u8) -> ExternResult<P2PMessageHashTables> {
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
    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    let mut message_contents: HashMap<String, (P2PMessageData, Vec<String>)> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    let mut reply_pairs: HashMap<String, Vec<String>> = HashMap::new();

    for message in queried_messages.into_iter() {
        if let Ok(message_entry) = TryInto::<P2PMessage>::try_into(message.clone()) {
            let message_hash: EntryHash = hash_entry(&message_entry)?;

            if message_entry.author.clone() == agent_info()?.agent_latest_pubkey {
                match agent_messages.get(&message_entry.receiver.clone().to_string()) {
                    Some(messages) if messages.len() >= batch_size.into() => {
                        continue; // continue to fill in other agent's hashmaps
                    }
                    Some(messages) if messages.len() < batch_size.into() => {
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
                            message_entry.clone(),
                            message_hash,
                            message_entry.receiver.clone(),
                        )?;
                    }
                    _ => {
                        if message_entry.reply_to != None {
                            insert_reply(
                                &mut reply_pairs,
                                message_entry.clone(),
                                message_hash.clone(),
                            );
                        };
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
                match agent_messages.get(&message_entry.author.clone().to_string()) {
                    Some(messages) if messages.len() >= batch_size.into() => continue, // break instead?
                    Some(messages) if messages.len() < batch_size.into() => {
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
                            message_entry.clone(),
                            message_hash,
                            message_entry.author.clone(),
                        )?;
                    }
                    _ => {
                        if message_entry.reply_to != None {
                            insert_reply(
                                &mut reply_pairs,
                                message_entry.clone(),
                                message_hash.clone(),
                            );
                        };
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
        } else {
            continue;
        }
    }

    get_receipts(&mut message_contents, &mut receipt_contents)?;

    get_replies(&mut reply_pairs, &mut message_contents)?;

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
