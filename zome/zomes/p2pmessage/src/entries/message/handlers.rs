use super::*;
use crate::utils::try_from_element;
// use hdk3::prelude::*;

use std::collections::HashMap;

/* TODO:
 * - proper error codes
 * - sending messages to self
 */

/*
 * ZOME FUNCTIONS ARE UNRESTRICTED BY DEFAULT
 * USERS OF THIS ZOME COULD IMPLEMENT
 * A WAY TO SET AND GET CAPABILITY GRANTS AND CLAIMS FOR CALL_REMOTE
 * TO SET SELECTED ACCESS TO ZOME FUNCTIONS
 */

/*
 * ZOME INIT FUNCTION TO SET UNRESTRICTED ACCESS
 */
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut receive_functions: GrantedFunctions = HashSet::new();
    receive_functions.insert((zome_info()?.zome_name, "receive_message".into()));
    let mut notify_functions: GrantedFunctions = HashSet::new();
    notify_functions.insert((zome_info()?.zome_name, "notify_delivery".into()));
    let mut emit_functions: GrantedFunctions = HashSet::new();
    emit_functions.insert((zome_info()?.zome_name, "emit_typing".into()));

    create_cap_grant(CapGrantEntry {
        tag: "receive".into(),
        access: ().into(),
        functions: receive_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "notify".into(),
        access: ().into(),
        functions: notify_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        access: ().into(),
        functions: emit_functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

pub(crate) fn send_message(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    // TODO: check if receiver is blocked

    let message = P2PMessage::from_input(message_input.clone())?;

    let file = match message_input.payload {
        PayloadInput::File { .. } => Some(P2PFileBytes::from_input(message_input)?),
        _ => None,
    };

    let receive_input = ReceiveMessageInput(message.clone(), file.clone());

    let receive_call_result: Result<P2PMessageReceipt, HdkError> = call_remote(
        message.receiver.clone(),
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &receive_input,
    );

    match receive_call_result {
        Ok(receive_output) => {
            let receipt = receive_output;
            create_entry(&message)?;
            create_entry(&receipt)?;
            if let Some(file) = file {
                create_entry(&file)?;
            };
            // TODO: CREATE AND RETURN ELEMENT HERE
            Ok(MessageAndReceipt(message, receipt))
        }
        Err(kind) => {
            match kind {
                // TIMEOUT; RECIPIENT IS OFFLINE; MESSAGE NEEDS TO BE SENT ASYNC
                // WILL BE IMPLEMENTED ONCE EPHEMERAL STORAGE IS IN PLACE
                HdkError::ZomeCallNetworkError(_err) => {
                    crate::err("TODO: 000", "Unknown other error")
                }
                HdkError::UnauthorizedZomeCall(_c, _z, _f, _p) => crate::err(
                    "TODO: 000:",
                    "This case shouldn't happen because of unrestricted access to receive message",
                ),
                _ => crate::err("TODO: 000", "Unknown other error"),
            }
        }
    }
}

pub(crate) fn receive_message(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(input.0.clone())?;
    create_entry(&input.0)?;
    create_entry(&receipt)?;
    if let Some(file) = input.1 {
        create_entry(&file)?;
    };
    Ok(receipt)
}

pub(crate) fn get_latest_messages(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
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

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;
        if message_entry.author.clone() == agent_info()?.agent_latest_pubkey {
            match agent_messages.get(&format!("{:?}", message_entry.receiver.clone())) {
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
            match agent_messages.get(&format!("{:?}", message_entry.author.clone())) {
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

pub(crate) fn get_next_batch_messages(
    filter: P2PMessageFilterBatch,
) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    agent_messages.insert(format!("{:?}", filter.conversant.clone()), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    let filter_timestamp = match filter.last_fetched_timestamp {
        Some(timestamp) => timestamp,
        None => {
            let now = sys_time()?;
            Timestamp(now.as_secs() as i64 / 84600, 0)
        }
    };

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;

        if message_entry.time_sent.0 <= filter_timestamp.0
            && (match filter.last_fetched_message_id {
                Some(ref id) if *id == message_hash => false,
                Some(ref id) if *id != message_hash => true,
                _ => false,
            })
            || filter.last_fetched_message_id == None
                && (message_entry.author == filter.conversant
                    || message_entry.receiver == filter.conversant)
        {
            match message_entry.payload {
                Payload::Text { .. } => {
                    if filter.payload_type == "Text" || filter.payload_type == "All" {
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
                Payload::File { .. } => {
                    if filter.payload_type == "File" || filter.payload_type == "All" {
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

pub(crate) fn get_messages_by_agent_by_timestamp(
    filter: P2PMessageFilterAgentTimestamp,
) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut agent_messages: HashMap<String, Vec<String>> = HashMap::new();
    agent_messages.insert(format!("{:?}", filter.conversant.clone()), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    let day_start = (filter.date.0 / 86400) * 86400;
    let day_end = day_start + 86399;

    for message in queried_messages.0.into_iter() {
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

fn get_receipts(
    message_contents: &mut HashMap<String, MessageBundle>,
    receipt_contents: &mut HashMap<String, P2PMessageReceipt>,
) -> ExternResult<()> {
    let queried_receipts = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(1),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    for receipt in queried_receipts.clone().0.into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;
        if message_contents.contains_key(&format!("{:?}", &receipt_entry.id)) {
            receipt_contents.insert(format!("{:?}", receipt_hash.clone()), receipt_entry.clone());
            if let Some(message_bundle) =
                message_contents.get_mut(&format!("{:?}", &receipt_entry.id))
            {
                message_bundle.1.push(format!("{:?}", receipt_hash))
            };
        }
    }

    Ok(())
}

fn insert_message(
    agent_messages: &mut HashMap<String, Vec<String>>,
    message_contents: &mut HashMap<String, MessageBundle>,
    message_entry: P2PMessage,
    message_hash: EntryHash,
    key: AgentPubKey,
) -> ExternResult<usize> {
    let mut message_array_length = 0;
    match agent_messages.get_mut(&format!("{:?}", key)) {
        Some(messages) => {
            messages.push(format!("{:?}", message_hash.clone()));
            message_array_length = messages.len();
        }
        None => {
            agent_messages.insert(
                format!("{:?}", key),
                vec![format!("{:?}", message_hash.clone())],
            );
        }
    };
    message_contents.insert(
        format!("{:?}", message_hash),
        MessageBundle(message_entry, Vec::new()),
    );

    Ok(message_array_length)
}

// fn _is_user_blocked(agent_pubkey: AgentPubKey) -> ExternResult<bool> {
//     match call::<AgentPubKey, BooleanWrapper>(
//         None,
//         "contacts".into(),
//         "in_blocked".into(),
//         None,
//         &agent_pubkey.clone()
//     ) {
//         Ok(output) => Ok(output.0),
//         _ => return crate::error("{\"code\": \"401\", \"message\": \"This agent has no proper authorization\"}")
//     }

//     let block_result: Result<BooleanWrapper, HdkError> = call_remote(
//         message_input.clone().receiver,
//         "contacts".into(),
//         "in_blocked".into(),
//         None,
//         &agent_pubkey
//     );

//     match block_result {
//         Ok(receive_output) => {
//             let message_entry = P2PMessage::from_parameter(receive_output.clone());
//             create_entry(&message_entry)?;
//             Ok(receive_output)
//         },
//         Err(kind) => {
//             match kind {
//                 // TIMEOUT; RECIPIENT IS OFFLINE; MESSAGE NEEDS TO BE SENT ASYNC
//                 HdkError::ZomeCallNetworkError(_err) => {
//                     match send_message_async(message_input) {
//                         Ok(async_result) => {
//                             let message_entry = P2PMessage::from_parameter(async_result.clone());
//                             create_entry(&message_entry)?;
//                             Ok(async_result)
//                         },
//                         _ => crate::err("TODO: 000", "This agent has no proper authorization")
//                     }
//                 },
//                 HdkError::UnauthorizedZomeCall(_c,_z,_f,_p) => crate::err("TODO: 000:", "This case shouldn't happen because of unrestricted access to receive message"),
//                 _ => crate::err("TODO: 000", "Unknown other error")
//             }
//         }
//     }
// }

fn _emit_typing(typing_info: TypingInfo) -> ExternResult<()> {
    emit_signal(&Signal::Typing(TypingSignal {
        kind: "message_sent".to_owned(),
        agent: typing_info.agent.to_owned(),
        is_typing: typing_info.is_typing,
    }))?;
    Ok(())
}

pub(crate) fn typing(typing_info: TypingInfo) -> ExternResult<()> {
    let payload = TypingInfo {
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing,
    };

    call_remote(
        typing_info.agent,
        zome_info()?.zome_name,
        "emit_typing".to_string().into(),
        None,
        &payload,
    )?;
    Ok(())
}
