use super::*;
use crate::utils::{address_deduper, try_from_element};
use crate::{timestamp::Timestamp};

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

    let mut emit_functions: GrantedFunctions = HashSet::new();
    emit_functions.insert((zome_info()?.zome_name, "is_typing".into()));

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

    //
    let mut fuctions = HashSet::new();

    // TODO: name may be changed to better suit the context of cap grant.s
    let tag: String = "create_group_cap_grant".into();
    let access: CapAccess = CapAccess::Unrestricted;

    let zome_name: ZomeName = zome_info()?.zome_name;
    let function_name: FunctionName = FunctionName("recv_remote_signal".into());

    fuctions.insert((zome_name, function_name));

    let cap_grant_entry: CapGrantEntry = CapGrantEntry::new(
        tag,    // A string by which to later query for saved grants.
        access, // Unrestricted access means any external agent can call the extern
        fuctions,
    );

    create_cap_grant(cap_grant_entry)?;

    Ok(InitCallbackResult::Pass)
}

pub(crate) fn send_message(message_input: MessageInput) -> ExternResult<MessageParameter> {
    // TODO: check if receiver is blocked

    let now = sys_time()?;
    let mut message = MessageParameter {
        author: agent_info()?.agent_latest_pubkey,
        receiver: message_input.receiver.clone(),
        payload: message_input.payload.clone(),
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        time_received: None,
        status: Status::Sent,
        reply_to: None,
    };

    if let Some(replied_message) = message_input.reply_to.clone() {
        let message_entry = P2PMessage::from_parameter(replied_message.clone());
        let message_entry_hash = hash_entry(&message_entry)?;
        message.reply_to = Some(message_entry_hash);
    };

    let receive_result: Result<MessageParameter, HdkError> = call_remote(
        message_input.clone().receiver,
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &message,
    );

    match receive_result {
        // MESSAGE HAS BEEN SENT AND DELIVERED
        Ok(receive_output) => {
            let message_entry = P2PMessage::from_parameter(receive_output.clone());
            create_entry(&message_entry)?;
            Ok(receive_output)
        }
        Err(kind) => {
            match kind {
                // TIMEOUT; RECIPIENT IS OFFLINE; MESSAGE NEEDS TO BE SENT ASYNC
                HdkError::ZomeCallNetworkError(_err) => match send_message_async(message_input) {
                    Ok(async_result) => {
                        let message_entry = P2PMessage::from_parameter(async_result.clone());
                        create_entry(&message_entry)?;
                        Ok(async_result)
                    }
                    _ => crate::err("TODO: 000", "This agent has no proper authorization"),
                },
                HdkError::UnauthorizedZomeCall(_c, _z, _f, _p) => crate::err(
                    "TODO: 000:",
                    "This case shouldn't happen because of unrestricted access to receive message",
                ),
                _ => crate::err("TODO: 000", "Unknown other error"),
            }
        }
    }
}

pub(crate) fn receive_message(message_input: MessageParameter) -> ExternResult<MessageParameter> {
    let mut message_entry = P2PMessage::from_parameter(message_input.clone());
    let now = sys_time()?;
    message_entry.time_received = Some(Timestamp(now.as_secs() as i64, now.subsec_nanos()));
    message_entry.status = Status::Delivered;
    create_entry(&message_entry)?;
    Ok(MessageParameter::from_entry(message_entry))
}

pub(crate) fn send_message_async(message_input: MessageInput) -> ExternResult<MessageParameter> {
    let now = sys_time()?;
    let message_async = P2PMessageAsync {
        author: agent_info()?.agent_latest_pubkey,
        receiver: message_input.receiver.clone(),
        payload: message_input.payload,
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        time_received: None,
        status: Status::Sent,
        reply_to: None,
    };

    create_entry(&message_async)?;

    create_link(
        message_input.receiver.into(),
        hash_entry(&message_async)?,
        LinkTag::new("async_messages"),
    )?;

    Ok(MessageParameter::from_async_entry(message_async))
}

// TODO: do we need a return value here?
pub(crate) fn fetch_async_messages() -> ExternResult<MessageListWrapper> {
    let links = get_links(
        agent_info()?.agent_latest_pubkey.into(),
        Some(LinkTag::new("async_messages")),
    )?;

    let mut message_list: Vec<MessageParameter> = Vec::new();
    for link in links.into_inner().into_iter() {
        debug!(format!("Nicko the link is {:?}", link.clone()));

        // get on an EntryHash
        // returns the "oldest live" element, i.e. header+entry
        match get(link.target.clone(), GetOptions::latest())? {
            Some(element) => {
                // BLOCK CHECK
                // let author = header.author();
                // if let true = is_user_blocked(author.clone())? { continue };
                // let entry_hash =

                let message_async_element: Result<P2PMessageAsync, HdkError> =
                    try_from_element(element.clone());
                match message_async_element {
                    Ok(message_async_entry) => {
                        match message_async_entry.status.clone() {
                            Status::Sent => {
                                let mut message_parameter =
                                    MessageParameter::from_async_entry(message_async_entry);

                                let now = sys_time()?;
                                message_parameter.time_received =
                                    Some(Timestamp(now.as_secs() as i64, now.subsec_nanos()));
                                message_parameter.status = Status::Delivered;
                                let message_entry =
                                    P2PMessage::from_parameter(message_parameter.clone());
                                create_entry(&message_entry)?;

                                let notify_delivery_result: Result<BooleanWrapper, HdkError> =
                                    call_remote(
                                        message_parameter.author.clone(),
                                        zome_info()?.zome_name,
                                        "notify_delivery".into(),
                                        None,
                                        &message_parameter,
                                    );

                                match notify_delivery_result {
                                    Ok(_notify_delivery_output) => {
                                        message_parameter.status = Status::Sent; // only for the purpose of the return value
                                        message_list.push(message_parameter.clone());
                                        Ok(())
                                    }
                                    Err(kind) => {
                                        match kind {
                                            // TIMEOUT; SENDER TO NOTIFY IS OFFLINE; NOTIFICATION SHOULD BE SENT ASYNC
                                            HdkError::ZomeCallNetworkError(_err) => {
                                                let _header = element.header_address().to_owned();
                                                let author = message_parameter.author.clone();
                                                let message_async_entry_new = P2PMessageAsync::from_parameter(message_parameter.clone());
                                                let _header_hash = create_entry(&message_async_entry_new)?;
                                                // update_entry(header, &message_async_entry_new)?;
                                                
                                                debug!(format!("Nicko the original element is {:?}", element.clone()));
                                                debug!(format!("Nicko the entry is {:?}", message_parameter.clone()));
                                                debug!(format!("Nicko the hash of the entry is {:?}", hash_entry(&message_async_entry_new)?));
                                                debug!(format!("Nicko the author is {:?}", author.clone()));
                                                
                                                create_link(
                                                    author.into(),
                                                    hash_entry(&message_async_entry_new)?,
                                                    LinkTag::new("async_messages")
                                                )?;
                                                // debug!(format!("Nicko the create_link header is {:?}", create_link_header));

                                                message_list.push(message_parameter.clone());
                                                Ok(())

                                                // match notify_delivery_async(NotifyAsyncInput(message_parameter.clone(), element.clone())) {
                                                //     Ok(_notify_delivery_async_result) => { 
                                                //         message_parameter.status = Status::Sent; // only for the purpose of the return value
                                                //         message_list.push(message_parameter.clone());
                                                //         Ok(())
                                                //     },
                                                //     _ => crate::err("TODO: 000", "Failed to update P2PMessageAsync entry")
                                                // }
                                            },
                                            HdkError::UnauthorizedZomeCall(_c,_z,_f,_p) => crate::err("TODO: 000:", "This case shouldn't happen because of unrestricted access to receive message"),
                                            _ => crate::err("TODO: 000", "Unknown other error")
                                        }
                                    }
                                }
                            }
                            Status::Delivered => {
                                let message_parameter =
                                    MessageParameter::from_async_entry(message_async_entry);
                                debug!(format!(
                                    "Nicko the async message is {:?}",
                                    message_parameter.clone()
                                ));
                                notify_delivery(message_parameter)?;
                                Ok(())
                            }
                            _ => {
                                return crate::err(
                                    "TODO: 000",
                                    "Unimplemented handlers for other status enums",
                                )
                            }
                        }?;
                    }
                    _ => return crate::err("TODO: 000", "Could not convert element"),
                }

                // delete_link(link.create_link_hash.clone())?;
            }
            _ => return crate::err("TODO: 000", "Could not get link target"),
        }
    }

    Ok(MessageListWrapper(message_list))
}

pub(crate) fn notify_delivery(message_parameter: MessageParameter) -> ExternResult<BooleanWrapper> {
    let message_entry_new = P2PMessage::from_parameter(message_parameter);
    create_entry(&message_entry_new)?;
    //TODO:  EMIT SIGNAL HERE
    Ok(BooleanWrapper(true))
}

pub(crate) fn notify_delivery_async(input: NotifyAsyncInput) -> ExternResult<BooleanWrapper> {
    let header = input.1.header_address().to_owned();
    let author = input.0.author.clone();
    let message_async_entry_new = P2PMessageAsync::from_parameter(input.0);
    update_entry(header, &message_async_entry_new)?;

    create_link(
        author.into(),
        hash_entry(&message_async_entry_new)?,
        LinkTag::new("async_messages"),
    )?;

    Ok(BooleanWrapper(true))
}

pub(crate) fn get_all_messages() -> ExternResult<MessageListWrapper> {
    let query_result = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let message_vec: Vec<MessageParameter> = query_result
        .0
        .into_iter()
        .filter_map(|el| {
            let entry = try_from_element(el);
            match entry {
                Ok(message_entry) => {
                    let message_output = MessageParameter::from_entry(message_entry);
                    Some(message_output)
                }
                _ => None,
            }
        })
        .collect();

    Ok(MessageListWrapper(message_vec))
}

pub(crate) fn get_all_messages_from_addresses(
    agent_list: AgentListWrapper,
) -> ExternResult<MessagesByAgentListWrapper> {
    let deduped_agents = address_deduper(agent_list.0);

    let query_result = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut agent_messages_hashmap = std::collections::HashMap::new();
    for agent in deduped_agents {
        let message_list: Vec<MessageParameter> = Vec::new();
        agent_messages_hashmap.insert(agent, message_list);
    }

    for element in query_result.0.into_iter() {
        let entry = try_from_element(element);
        match entry {
            Ok(message_entry) => {
                let message_output = MessageParameter::from_entry(message_entry);
                if agent_messages_hashmap.contains_key(&message_output.clone().author) {
                    if let Some(vec) =
                        agent_messages_hashmap.get_mut(&message_output.clone().author)
                    {
                        vec.push(message_output.clone());
                    } else {
                        ()
                    }
                } else {
                    ()
                }
            }
            _ => (),
        };
    }

    let mut agent_messages_vec: Vec<MessagesByAgent> = Vec::new();
    for (agent, list) in agent_messages_hashmap.iter() {
        agent_messages_vec.push(MessagesByAgent {
            author: agent.to_owned(),
            messages: list.to_owned(),
        });
    }

    Ok(MessagesByAgentListWrapper(agent_messages_vec))
}

// TODO: change implementation once query accepts timestamp range.
pub(crate) fn get_batch_messages_on_conversation(
    message_range: MessageRange,
) -> ExternResult<MessageListWrapper> {
    let timegap = 10; //in seconds
    let batch_size = 10; // number of messages

    let query_result = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(0),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    let mut message_output_vec: Vec<MessageParameter> = Vec::new();
    for element in query_result.0 {
        let entry = try_from_element::<P2PMessage>(element);
        match entry {
            Ok(message_entry) => {
                if message_output_vec.len() <= 0
                    || (message_output_vec.len() <= batch_size
                        && message_range.last_message_timestamp_seconds - message_entry.time_sent.0
                            < timegap)
                {
                    if message_entry.author == message_range.author {
                        if message_entry.time_sent.0 <= message_range.last_message_timestamp_seconds
                        {
                            let message_output = MessageParameter::from_entry(message_entry);
                            message_output_vec.push(message_output);
                        }
                    };
                    continue;
                };
                break;
            }
            _ => continue,
        }
    }

    Ok(MessageListWrapper(message_output_vec))
}

// fn is_user_blocked(agent_pubkey: AgentPubKey) -> ExternResult<bool> {
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

pub(crate) fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    let payload = Signal::P2PTypingDetailSignal(P2PTypingDetailIO {
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing,
    });

    let mut agents = Vec::new();

    agents.push(typing_info.agent);
    agents.push(agent_info()?.agent_latest_pubkey);


    remote_signal(&payload, agents)?;
    Ok(())
}

pub(crate) fn read_message(read_receipt_input: ReadReceiptInput) -> ExternResult<ReceiptContents> {
    create_entry(&read_receipt_input.receipt)?;
    call_remote(read_receipt_input.sender, zome_info()?.zome_name, FunctionName( "receive_read_receipt".into()), None, &read_receipt_input.receipt)
}

#[cfg(test)]
pub(crate) fn receive_read_receipt(receipt: P2PMessageReceipt) -> ExternResult<ReceiptContents> {
    let receipts = commit_receipts(vec![receipt])?;
    emit_signal(Signal::P2PMessageReceipt(receipts.clone()))?;
    Ok(receipts)
}

#[cfg(test)]
pub(crate) fn commit_receipts(receipts: Vec<P2PMessageReceipt>) -> ExternResult<ReceiptContents> {
    // Query all the receipts 
    let query_result = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                EntryDefIndex::from(2),
                zome_info()?.zome_id,
                EntryVisibility::Private,
            )))
            .include_entries(true),
    )?;

    // Get all receipts from query result
    let all_receipts = query_result.0.into_iter().filter_map(|el| {
        if let Ok(Some(receipt)) = el.into_inner().1.to_app_option::<P2PMessageReceipt>() {
            return Some(receipt)
        } else {
            None
        }
    }).collect::<Vec<P2PMessageReceipt>>();

    // initialize hash map that will be returned
    let mut receipts_hash_map: HashMap<String, P2PMessageReceipt> = HashMap::new();

    // Iterate through the receipts in the argument and push them into the hash map
    receipts.clone().into_iter().for_each(|receipt| {
        if let Ok(entry_hash) = hash_entry(&receipt) {
            receipts_hash_map.insert(format!("{:?}", entry_hash), receipt);
        }
    });

    // Iterate through the receipts to check if the receipt has been committed, remove them from the hash map if it is 
    // used for loops instead of for_each because you cant break iterators
    for i in 0..all_receipts.len() {
        let receipt = all_receipts[i].clone();
        let hash = format!("{:?}", hash_entry(&receipt)?);

        if receipts_hash_map.contains_key(&hash) {
            receipts_hash_map.remove(&hash);
        }

        if receipts_hash_map.is_empty() {
            break;
        }   
    }

    // iterate the remaining contents of the hashmap
    receipts_hash_map.clone().into_iter().for_each(|(_entry_hash, receipt)| {
        create_entry(&receipt).expect("Expected P2P message receipt entry");    
    });


    Ok(ReceiptContents(receipts_hash_map))
}
