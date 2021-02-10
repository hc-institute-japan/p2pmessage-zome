use hdk3::prelude::*;
use crate::utils::{try_from_element};
use super::*;

use std::collections::HashMap as HashMap;

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
        functions: emit_functions
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
        &message
    );

    match receive_call_result {
        // MESSAGE HAS BEEN SENT AND DELIVERED
        Ok(receive_output) => {
            let receipt = receive_output;
            create_entry(&message)?;
            create_entry(&receipt)?;
            // TODO: create and return element here
            // element <- <signedheaderhashed, option<entry>>
            // signedheaderhashed <- <header, signature>
            // let receipts = vec![hash_entry(&receipt)?];
            // Ok(MessageBundle(message, receipts))
            Ok(MessageAndReceipt(message, receipt))
        },
        Err(kind) => {
            match kind {
                // TIMEOUT; RECIPIENT IS OFFLINE; MESSAGE NEEDS TO BE SENT ASYNC
                // will be implemented once ephemeral delivery system rolls out
                HdkError::ZomeCallNetworkError(_err) => crate::err("TODO: 000", "Unknown other error"),
                HdkError::UnauthorizedZomeCall(_c,_z,_f,_p) => crate::err("TODO: 000:", "This case shouldn't happen because of unrestricted access to receive message"),
                _ => crate::err("TODO: 000", "Unknown other error")
            }
        }
    }
}

pub(crate) fn receive_message(message: P2PMessage) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(message.clone())?;
    create_entry(&message)?;
    create_entry(&receipt)?;
    // let receipts = vec![hash_entry(&receipt)?];
    // let message_bundle = MessageBundle(message, receipts);
    Ok(receipt)
}

// pub(crate) fn send_message_async(message_input: MessageInput) -> ExternResult<MessageParameter> {
//     let now = sys_time()?;
//     let message_async = P2PMessageAsync {
//         author: agent_info()?.agent_latest_pubkey,
//         receiver: message_input.receiver.clone(),
//         payload: message_input.payload,
//         time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
//         time_received: None,
//         status: Status::Sent,
//         reply_to: None
//     };

//     create_entry(&message_async)?;

//     create_link(
//         message_input.receiver.into(),
//         hash_entry(&message_async)?,
//         LinkTag::new("async_messages")
//     )?;

//     Ok(MessageParameter::from_async_entry(message_async))
// }

// TODO: do we need a return value here?
// pub(crate) fn fetch_async_messages() -> ExternResult<MessageListWrapper> {

//     let links = get_links(agent_info()?.agent_latest_pubkey.into(), Some(LinkTag::new("async_messages")))?;

//     let mut message_list: Vec<MessageParameter> = Vec::new();
//     for link in links.into_inner().into_iter() {
//         debug!(format!("Nicko the link is {:?}", link.clone()));

//         // get on an EntryHash
//         // returns the "oldest live" element, i.e. header+entry
//         match get(link.target.clone(), GetOptions::latest())? {
//             Some(element) => {

//                 // BLOCK CHECK
//                 // let author = header.author();
//                 // if let true = is_user_blocked(author.clone())? { continue };
//                 // let entry_hash =

//                 let message_async_element: Result<P2PMessageAsync, HdkError> = try_from_element(element.clone());
//                 match message_async_element {
//                     Ok(message_async_entry) => {
//                         match message_async_entry.status.clone() {
//                             Status::Sent => {
//                                 let mut message_parameter = MessageParameter::from_async_entry(message_async_entry);

//                                 let now = sys_time()?;
//                                 message_parameter.time_received = Some(Timestamp(now.as_secs() as i64, now.subsec_nanos()));
//                                 message_parameter.status = Status::Delivered;
//                                 let message_entry = P2PMessage::from_parameter(message_parameter.clone());
//                                 create_entry(&message_entry)?;

//                                 let notify_delivery_result: Result<BooleanWrapper, HdkError> = call_remote(
//                                     message_parameter.author.clone(),
//                                     zome_info()?.zome_name,
//                                     "notify_delivery".into(),
//                                     None,
//                                     &message_parameter
//                                 );
                            
//                                 match notify_delivery_result {
//                                     Ok(_notify_delivery_output) => {
//                                         message_parameter.status = Status::Sent; // only for the purpose of the return value
//                                         message_list.push(message_parameter.clone());
//                                         Ok(())
//                                     },
//                                     Err(kind) => {
//                                         match kind {
//                                             // TIMEOUT; SENDER TO NOTIFY IS OFFLINE; NOTIFICATION SHOULD BE SENT ASYNC
//                                             HdkError::ZomeCallNetworkError(_err) => {
//                                                 let _header = element.header_address().to_owned();
//                                                 let author = message_parameter.author.clone();
//                                                 let message_async_entry_new = P2PMessageAsync::from_parameter(message_parameter.clone());
//                                                 let _header_hash = create_entry(&message_async_entry_new)?;
//                                                 // update_entry(header, &message_async_entry_new)?;
                                                
//                                                 debug!(format!("Nicko the original element is {:?}", element.clone()));
//                                                 debug!(format!("Nicko the entry is {:?}", message_parameter.clone()));
//                                                 debug!(format!("Nicko the hash of the entry is {:?}", hash_entry(&message_async_entry_new)?));
//                                                 debug!(format!("Nicko the author is {:?}", author.clone()));
                                                
//                                                 create_link(
//                                                     author.into(),
//                                                     hash_entry(&message_async_entry_new)?,
//                                                     LinkTag::new("async_messages")
//                                                 )?;
//                                                 // debug!(format!("Nicko the create_link header is {:?}", create_link_header));

//                                                 message_list.push(message_parameter.clone());
//                                                 Ok(())

//                                                 // match notify_delivery_async(NotifyAsyncInput(message_parameter.clone(), element.clone())) {
//                                                 //     Ok(_notify_delivery_async_result) => { 
//                                                 //         message_parameter.status = Status::Sent; // only for the purpose of the return value
//                                                 //         message_list.push(message_parameter.clone());
//                                                 //         Ok(())
//                                                 //     },
//                                                 //     _ => crate::err("TODO: 000", "Failed to update P2PMessageAsync entry")
//                                                 // }
//                                             },
//                                             HdkError::UnauthorizedZomeCall(_c,_z,_f,_p) => crate::err("TODO: 000:", "This case shouldn't happen because of unrestricted access to receive message"),
//                                             _ => crate::err("TODO: 000", "Unknown other error")
//                                         }
//                                     }
//                                 }
//                             },
//                             Status::Delivered => {
//                                 let message_parameter = MessageParameter::from_async_entry(message_async_entry);
//                                 debug!(format!("Nicko the async message is {:?}", message_parameter.clone()));
//                                 notify_delivery(message_parameter)?;
//                                 Ok(())
//                             },
//                             _ => return crate::err("TODO: 000", "Unimplemented handlers for other status enums")
//                         }?;
//                     },
//                     _ => return crate::err("TODO: 000", "Could not convert element")
//                 }

//                 // delete_link(link.create_link_hash.clone())?;
//             },
//             _ => return crate::err("TODO: 000", "Could not get link target")
//         }
//     }
    
//     Ok(MessageListWrapper(message_list))
// }

// pub(crate) fn notify_delivery(message: P2PMessage) -> ExternResult<BooleanWrapper> {
//     let message_entry_new = P2PMessage::from_parameter(message_parameter);
//     create_entry(&message_entry_new)?;
//     //TODO:  EMIT SIGNAL HERE    
//     Ok(BooleanWrapper(true))
// }

// pub(crate) fn notify_delivery_async(input: NotifyAsyncInput) -> ExternResult<BooleanWrapper> {
//     let header = input.1.header_address().to_owned();
//     let author = input.0.author.clone();
//     let message_async_entry_new = P2PMessageAsync::from_parameter(input.0);
//     update_entry(header, &message_async_entry_new)?;

//     create_link(
//         author.into(),
//         hash_entry(&message_async_entry_new)?,
//         LinkTag::new("async_messages")
//     )?;
    
//     Ok(BooleanWrapper(true))
// }

pub(crate) fn get_all_messages() -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(0), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let queried_receipts= query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(1), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let mut agent_messages: HashMap<String, Vec<EntryHash>> = HashMap::new();
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;
        match agent_messages.get_mut(&message_entry.author.to_string()) {
            Some(messages) => messages.push(message_hash.clone()),
            None => {
                let mut message_hashes = Vec::new();
                message_hashes.push(message_hash.clone());
                agent_messages.insert(message_entry.author.clone().to_string(), message_hashes);
            }
        };
        let receipt_hashes = Vec::new();
        let bundle = MessageBundle(message_entry, receipt_hashes);
        message_contents.insert(message_hash.clone().to_string(), bundle);
    };

    for receipt in queried_receipts.clone().0.into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;
        if message_contents.contains_key(&receipt_entry.id.to_string()) {
            receipt_contents.insert(receipt_hash.clone().to_string(), receipt_entry.clone());
            if let Some(message_bundle) = message_contents.get_mut(&receipt_entry.id.to_string()) {
                message_bundle.1.push(receipt_hash)
            };
        }
    };

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages), 
        MessageContents(message_contents),
        ReceiptContents(receipt_contents)
    ))
}

pub(crate) fn get_messages_by_agent_by_timestamp(filter: P2PMessageFilterAgentTimestamp) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(0), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let queried_receipts= query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(1), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let mut agent_messages: HashMap<String, Vec<EntryHash>> = HashMap::new();
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    let day_start = (filter.date.0 / 86400) * 86400;
    let day_end = day_start + 86399; 

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;
        if message_entry.author == filter.conversant || message_entry.receiver == filter.conversant {
            // TODO: use header timestamp for message_time
            let message_time = message_entry.time_sent.0;
            if message_time >= day_start && message_time <= day_end {
                match agent_messages.get_mut(&filter.conversant.to_string()) {
                    Some(messages) => messages.push(message_hash.clone()),
                    None => { () }
                };
                let receipt_hashes = Vec::new();
                let bundle = MessageBundle(message_entry, receipt_hashes);
                message_contents.insert(message_hash.clone().to_string(), bundle);    
            }
        }
    };

    for receipt in queried_receipts.clone().0.into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;
        if message_contents.contains_key(&receipt_entry.id.to_string()) {
            receipt_contents.insert(receipt_hash.clone().to_string().to_string(), receipt_entry.clone());
            if let Some(message_bundle) = message_contents.get_mut(&receipt_entry.id.to_string()) {
                message_bundle.1.push(receipt_hash)
            };
        }
    };

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages), 
        MessageContents(message_contents),
        ReceiptContents(receipt_contents)
    ))
}

pub(crate) fn get_latest_messages(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(0), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let queried_receipts= query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(1), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let mut agent_messages: HashMap<String, Vec<EntryHash>> = HashMap::new();
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;
        // also check the receiver here
        if message_entry.author.clone() == agent_info()?.agent_latest_pubkey {
            // add this message to receiver's array in hashmap
            if let Some(messages) = agent_messages.get_mut(&message_entry.receiver.to_string()) {
                if messages.len() >= batch_size.0.into() { continue }
                else { messages.push(message_hash.clone()) }
            } else {
                agent_messages.insert(message_entry.receiver.to_string(), vec![message_hash.clone()]);
            }
        } else {
            // add this message to author's array in hashmap
            if let Some(messages) = agent_messages.get_mut(&message_entry.author.to_string()) {
                if messages.len() >= batch_size.0.into() { continue }
                else { messages.push(message_hash.clone()) }
            } else {
                agent_messages.insert(message_entry.author.to_string(), vec![message_hash.clone()]);
            }
        };
        message_contents.insert(message_hash.clone().to_string(), MessageBundle(message_entry, Vec::new()));    
    };

    for receipt in queried_receipts.clone().0.into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;
        if message_contents.contains_key(&receipt_entry.id.to_string()) {
            receipt_contents.insert(receipt_hash.clone().to_string().to_string(), receipt_entry.clone());
            if let Some(message_bundle) = message_contents.get_mut(&receipt_entry.id.to_string()) {
                message_bundle.1.push(receipt_hash)
            };
        }
    };

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages), 
        MessageContents(message_contents),
        ReceiptContents(receipt_contents)
    ))
}

pub(crate) fn get_next_batch_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    let queried_messages = query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(0), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let queried_receipts= query(
        QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(EntryDefIndex::from(1), zome_info()?.zome_id, EntryVisibility::Private)))
        .include_entries(true)
    )?;

    let mut agent_messages: HashMap<String, Vec<EntryHash>> = HashMap::new();
    agent_messages.insert(filter.conversant.clone().to_string(), Vec::new());
    let mut message_contents: HashMap<String, MessageBundle> = HashMap::new();
    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();   

    for message in queried_messages.0.into_iter() {
        let message_entry: P2PMessage = try_from_element(message)?;
        let message_hash = hash_entry(&message_entry)?;

        if message_entry.time_sent.0 <= filter.last_fetched_timestamp.0 && message_hash != filter.last_fetched_message_id {
            if message_entry.author == filter.conversant || message_entry.receiver == filter.conversant {
                if let Some(messages) = agent_messages.get_mut(&filter.conversant.to_string()) {
                    if messages.len() >= filter.batch_size.into() { break }
                    else { messages.push(message_hash.clone()) }
                }
            }
        message_contents.insert(message_hash.clone().to_string(), MessageBundle(message_entry, Vec::new()));
        }
    };

    for receipt in queried_receipts.clone().0.into_iter() {
        let receipt_entry: P2PMessageReceipt = try_from_element(receipt)?;
        let receipt_hash = hash_entry(&receipt_entry)?;
        if message_contents.contains_key(&receipt_entry.id.to_string()) {
            receipt_contents.insert(receipt_hash.clone().to_string().to_string(), receipt_entry.clone());
            if let Some(message_bundle) = message_contents.get_mut(&receipt_entry.id.to_string()) {
                message_bundle.1.push(receipt_hash)
            };
        }
    };

    Ok(P2PMessageHashTables(
        AgentMessages(agent_messages), 
        MessageContents(message_contents),
        ReceiptContents(receipt_contents)
    ))
}

// pub(crate) fn get_corresponding_receipts(message_id: EntryHash) -> ExternResult<Vec<P2PMessageReceipt>> {
    
// }

// pub(crate) fn get_all_messages_from_addresses(agent_list: AgentListWrapper) -> ExternResult<MessagesByAgentListWrapper> {
//     let deduped_agents = address_deduper(agent_list.0);

//     let query_result = query(
//         QueryFilter::new()
//         .entry_type(
//             EntryType::App(
//                 AppEntryType::new(
//                     EntryDefIndex::from(0),
//                     zome_info()?.zome_id,
//                     EntryVisibility::Private
//                 )
//             )
//         )
//         .include_entries(true)
//     )?;

//     let mut agent_messages_hashmap = std::collections::HashMap::new();
//     for agent in deduped_agents {
//         let message_list: Vec<MessageParameter> = Vec::new();
//         agent_messages_hashmap.insert(agent, message_list);                                                                                                                                                                               
//     };

//     for element in query_result.0.into_iter() {
//         let entry = try_from_element(element);
//         match entry {
//             Ok(message_entry) => {
//                 let message_output = MessageParameter::from_entry(message_entry);
//                 if agent_messages_hashmap.contains_key(&message_output.clone().author) {
//                     if let Some(vec) = agent_messages_hashmap.get_mut(&message_output.clone().author) {
//                         vec.push(message_output.clone());
//                     } else { () }
//                 } else { () }
//             },
//             _ => ()
//         };
//     };

//     let mut agent_messages_vec: Vec<MessagesByAgent> = Vec::new();
//     for (agent, list) in agent_messages_hashmap.iter() {
//         agent_messages_vec.push(
//             MessagesByAgent {
//                 author: agent.to_owned(),
//                 messages: list.to_owned()
//             }
//         );
//     }

//     Ok(MessagesByAgentListWrapper(agent_messages_vec))
// }

// pub(crate) fn get_batch_messages_on_conversation(message_range: MessageRange) -> ExternResult<MessageListWrapper> {

//     let timegap = 10; //in seconds
//     let batch_size = 10; // number of messages

//     let query_result = query(
//         QueryFilter::new()
//         .entry_type(
//             EntryType::App(
//                 AppEntryType::new(
//                     EntryDefIndex::from(0),
//                     zome_info()?.zome_id,
//                     EntryVisibility::Private
//                 )
//             )
//         )
//         .include_entries(true)
//     )?;

//     let mut message_output_vec: Vec<MessageParameter> = Vec::new();
//     for element in query_result.0 {
//         let entry = try_from_element::<P2PMessage>(element);
//         match entry {
//             Ok(message_entry) => {
//                 if message_output_vec.len() <= 0 
//                 || (message_output_vec.len() <= batch_size && message_range.last_message_timestamp_seconds - message_entry.time_sent.0 < timegap) {
//                     if message_entry.author == message_range.author {
//                         if message_entry.time_sent.0 <= message_range.last_message_timestamp_seconds {
//                             let message_output = MessageParameter::from_entry(message_entry);
//                             message_output_vec.push(message_output);
//                         }
//                     };
//                     continue
//                 };
//                 break
//             },
//             _ => continue
//         }
//     };

//     Ok(MessageListWrapper(message_output_vec))
// }

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
        is_typing: typing_info.is_typing
    }))?;
    Ok(())
}

pub(crate) fn typing(typing_info: TypingInfo) -> ExternResult<()> { 
    let payload = TypingInfo {
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing
    };

    call_remote(
        typing_info.agent,
        zome_info()?.zome_name,
        "emit_typing".to_string().into(),
        None,
        &payload
    )?;
    Ok(())
}