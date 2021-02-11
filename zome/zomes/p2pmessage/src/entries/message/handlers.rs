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

    let message = P2PMessage::from_input(message_input)?;

    let receive_call_result: Result<P2PMessageReceipt, HdkError> = call_remote(
        message.receiver.clone(),
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &message,
    );

    match receive_call_result {
        Ok(receive_output) => {
            let receipt = receive_output;
            create_entry(&message)?;
            create_entry(&receipt)?;
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

pub(crate) fn receive_message(message: P2PMessage) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(message.clone())?;
    create_entry(&message)?;
    create_entry(&receipt)?;
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
            // add this message to receiver's array in hashmap
            if let Some(messages) = agent_messages.get_mut(&message_entry.receiver.to_string()) {
                if messages.len() >= batch_size.0.into() {
                    continue;
                } else {
                    messages.push(message_hash.clone().to_string())
                }
            } else {
                agent_messages.insert(
                    message_entry.receiver.to_string(),
                    vec![message_hash.clone().to_string()],
                );
            }
        } else {
            // add this message to author's array in hashmap
            if let Some(messages) = agent_messages.get_mut(&message_entry.author.to_string()) {
                if messages.len() >= batch_size.0.into() {
                    continue;
                } else {
                    messages.push(message_hash.clone().to_string())
                }
            } else {
                agent_messages.insert(
                    message_entry.author.to_string(),
                    vec![message_hash.clone().to_string()],
                );
            }
        };
        message_contents.insert(
            message_hash.clone().to_string(),
            MessageBundle(message_entry, Vec::new()),
        );
    }

    get_corresponding_receipts(&mut message_contents, &mut receipt_contents)?;

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
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;

        if message_entry.time_sent.0 <= filter.last_fetched_timestamp.0
            && message_hash != filter.last_fetched_message_id
        {
            if message_entry.author == filter.conversant
                || message_entry.receiver == filter.conversant
            {
                if let Some(messages) = agent_messages.get_mut(&filter.conversant.to_string()) {
                    if messages.len() >= filter.batch_size.into() {
                        break;
                    } else {
                        messages.push(message_hash.clone().to_string());
                        message_contents.insert(
                            message_hash.clone().to_string(),
                            MessageBundle(message_entry, Vec::new()),
                        );
                    }
                }
            }
        }
    }

    get_corresponding_receipts(&mut message_contents, &mut receipt_contents)?;

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
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    let day_start = (filter.date.0 / 86400) * 86400;
    let day_end = day_start + 86399;

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;
        if message_entry.author == filter.conversant || message_entry.receiver == filter.conversant
        {
            // TODO: use header timestamp for message_time
            let message_time = message_entry.time_sent.0;
            if message_time >= day_start && message_time <= day_end {
                match agent_messages.get_mut(&filter.conversant.to_string()) {
                    Some(messages) => messages.push(message_hash.clone().to_string()),
                    None => (),
                };
                let receipt_hashes = Vec::new();
                let bundle = MessageBundle(message_entry, receipt_hashes);
                message_contents.insert(message_hash.clone().to_string(), bundle);
            }
        }
    }

    get_corresponding_receipts(&mut message_contents, &mut receipt_contents)?;

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages),
        MessageContents(message_contents),
        ReceiptContents(receipt_contents),
    ))
}

fn get_corresponding_receipts(
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
        if message_contents.contains_key(&receipt_entry.id.to_string()) {
            receipt_contents.insert(receipt_hash.clone().to_string(), receipt_entry.clone());
            if let Some(message_bundle) = message_contents.get_mut(&receipt_entry.id.to_string()) {
                message_bundle.1.push(receipt_hash.to_string())
            };
        }
    }

    Ok(())
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
