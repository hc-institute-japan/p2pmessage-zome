use hdk3::prelude::*;
use crate::{timestamp::Timestamp};

use super::{
    MessageEntry,
    MessageInput,
    MessageOutput,
    MessageOutputOption,
    MessageListWrapper,
    MessagesByAgent,
    MessagesByAgentListWrapper,
    AgentListWrapper,
    BooleanWrapper,
    RemoteCallArgument,
    MessageRange
};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info!()?.zome_name, "receive_message".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "receive".into(),
            access: ().into(),
            functions,
        }
    )?;

    let mut functions2: GrantedFunctions = HashSet::new();
    functions2.insert((zome_info!()?.zome_name, "needs_cap_claim".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "needs_cap_claim".into(),
            access: ().into(),
            functions: functions2
        }
    )?;

    Ok(InitCallbackResult::Pass)
}

// GRANTS AND CLAIMS FUNCTIONS

#[derive(serde::Serialize, serde::Deserialize, SerializedBytes)]
pub struct CapFor(CapSecret, AgentPubKey);

// check to see if callee requires a cap_claim for the functions in this zome
#[hdk_extern]
fn needs_cap_claim(_: ()) -> ExternResult<BooleanWrapper> {
    Ok(BooleanWrapper(false))
}

// tries an existing cap_claim if still valid
#[hdk_extern]
fn try_cap_claim(cap_for: CapFor) -> ExternResult<ZomeCallResponse> {
    let result: ZomeCallResponse = call_remote!(
        cap_for.1,
        zome_info!()?.zome_name,
        "needs_cap_claim".to_string().into(),
        Some(cap_for.0),
        ().try_into()?
    )?;

    Ok(result)
}

// #[hdk_extern]
// fn request_cap_grant(agent_address: AgentPubKey) -> ExternResult<CapGrantEntry> {
//     let payload: SerializedBytes = agent_address.clone().try_into()?;

//     match call_remote!(
//         agent_address,
//         zome_info!()?.zome_name,
//         "send_chain_grant".into(),
//         None,
//         payload
//     )? {
//         ZomeCallResponse::Ok(output) => {
//             debug!("nicko call remote authorized")?;

//             let sb = output.into_inner();
//             let message_output: MessageOutputOption = sb.try_into()?;

//             match message_output.0 {
//                 Some(message_output) => {
//                     let return_val = message_output.clone();
//                     let message_entry = MessageEntry {
//                         author: message_output.author,
//                         receiver: message_output.receiver,
//                         payload: message_output.payload,
//                         timestamp: message_output.timestamp
//                     };
//                     create_entry!(&message_entry)?;
//                     debug!("nicko call remote caller success")?;
//                     Ok(MessageOutputOption(Some(return_val)))
//                 },
//                 None => {
//                     debug!("nicko call remote caller fail")?;
//                     Ok(MessageOutputOption(None))
//                 }
//             }
            
//         },
//         ZomeCallResponse::Unauthorized => {
//             debug!("nicko call remote unauthorized")?;
//             crate::error("{\"code\": \"401\", \"message\": \"This agent has no proper authorization\"}")
//         }
//     }
// }

// #[hdk_extern]
// fn send_cap_grant(_: ()) -> ExternResult<CapGrantEntry> {
//     let query_result = query!(
//         QueryFilter::new()
//         .entry_type(EntryType::CapGrant)
//         .include_entries(true)
//     )?;

//     let grants_vec: Vec<CapGrantEntry> = query_result.0
//         .into_iter()
//         .filter_map(|el| {
//             let header_hash = el.header_address().to_owned();
//             let entry = el.into_inner().1.as_option();
//             match entry {
//                 Ok(Some(message_entry)) => {
//                     let message_output = MessageOutput {
//                         header: header_hash,
//                         author: message_entry.author,
//                         receiver: message_entry.receiver,
//                         payload: message_entry.payload,
//                         timestamp: message_entry.timestamp
//                     };
//                     Some(message_output)
//                 },
//                 _ => None
//             }
//         })
//         .collect();
// }


pub(crate) fn send_message(message_input: MessageInput) -> ExternResult<MessageOutputOption> {
    
    debug!("nicko call remote start")?;

    let receiver = message_input.receiver.clone();
    let remote_input = RemoteCallArgument {
        author: agent_info!()?.agent_latest_pubkey, 
        input: message_input
    };
    let payload: SerializedBytes = remote_input.clone().try_into()?;

    match call_remote!(
        receiver,
        zome_info!()?.zome_name,
        "receive_message".into(),
        None,
        payload
    )? {
        ZomeCallResponse::Ok(output) => {
            debug!("nicko call remote authorized")?;

            let sb = output.into_inner();
            let message_output: MessageOutputOption = sb.try_into()?;

            match message_output.0 {
                Some(message_output) => {
                    let return_val = message_output.clone();
                    let message_entry = MessageEntry {
                        author: message_output.author,
                        receiver: message_output.receiver,
                        payload: message_output.payload,
                        timestamp: message_output.timestamp
                    };
                    create_entry!(&message_entry)?;
                    debug!("nicko call remote caller success")?;
                    Ok(MessageOutputOption(Some(return_val)))
                },
                None => {
                    debug!("nicko call remote caller fail")?;
                    Ok(MessageOutputOption(None))
                }
            }
            
        },
        ZomeCallResponse::Unauthorized => {
            debug!("nicko call remote unauthorized")?;
            crate::error("{\"code\": \"401\", \"message\": \"This agent has no proper authorization to send the message to the receiver\"}")
        }
    }
}

pub(crate) fn receive_message(remote_input: RemoteCallArgument) -> ExternResult<MessageOutputOption> {
    
    debug!("nicko call remote callee entered")?;
    
    let author = remote_input.author;
    let message_input = remote_input.input;

    // remote call's author is the callee's address
    // debug!(format!("agent info address: {:?}", agent_info!()?.agent_latest_pubkey))?;
    // debug!(format!("recipient address: {:?}", message_input.receiver.clone()))?;

    let now = sys_time!()?;
    let message_entry = MessageEntry {
        author: author,
        receiver: agent_info!()?.agent_latest_pubkey,
        payload: message_input.payload,
        timestamp: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
    };

    match create_entry!(&message_entry) {
        Ok(header) => {
            let message_output = MessageOutput {
                header: header,
                author: message_entry.author,
                receiver: message_entry.receiver,
                payload: message_entry.payload,
                timestamp: message_entry.timestamp
            };
            debug!("nicko call remote callee success")?;
            Ok(MessageOutputOption(Some(message_output)))
        },
        _ => {
            debug!("nicko call remote callee fail")?;
            Ok(MessageOutputOption(None))
        }
    }
}

pub(crate) fn get_all_messages() -> ExternResult<MessageListWrapper> {
    debug!("nicko get all messages start")?;

    let query_result = query!(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info!()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    let message_vec: Vec<MessageOutput> = query_result.0
        .into_iter()
        .filter_map(|el| {
            let header_hash = el.header_address().to_owned();
            let entry: Result<Option<MessageEntry>, SerializedBytesError> = el.into_inner().1.to_app_option();
            match entry {
                Ok(Some(message_entry)) => {
                    let message_output = MessageOutput {
                        header: header_hash,
                        author: message_entry.author,
                        receiver: message_entry.receiver,
                        payload: message_entry.payload,
                        timestamp: message_entry.timestamp
                    };
                    Some(message_output)
                },
                _ => None
            }
        })
        .collect();

    debug!("nicko get all messages end")?;

    Ok(message_vec.into())
}

pub(crate) fn get_all_messages_from_addresses(agent_list: AgentListWrapper) -> ExternResult<MessagesByAgentListWrapper> {
    
    // remove duplicate addresses
    let deduped_agents = address_deduper(agent_list.0);

    // get all messages from source chain
    let query_result = query!(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info!()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    debug!(format!("nicko query result: {:?}", query_result))?;

    // initialize the hashmap for bucket sorting
    // [agent_1: [message_1_a, message_1_b, ...], agent_2: [message_2_a, message_2_b,...]]
    let mut agent_messages_hashmap = std::collections::HashMap::new();
    for agent in deduped_agents {
        let message_list: Vec<MessageOutput> = Vec::new();
        agent_messages_hashmap.insert(agent, message_list);                                                                                                                                                                               
    };

    // bucket sort: iterate over all messages and filter by author
    let _map_result: Vec<Option<MessageOutput>> = query_result.0
        .into_iter()
        .map(|el| {
            // get header and entry
            let header_hash = el.header_address().to_owned();
            let entry: Result<Option<MessageEntry>, SerializedBytesError> = el.into_inner().1.to_app_option();
            match entry {
                // if entry is a valid MessageEntry, construct MessageOutput
                Ok(Some(message_entry)) => {
                    let message_output = MessageOutput {
                        header: header_hash,
                        author: message_entry.author,
                        receiver: message_entry.receiver,
                        payload: message_entry.payload,
                        timestamp: message_entry.timestamp
                    };
                    // check if the message author is one of the agents being fetched
                    if agent_messages_hashmap.contains_key(&message_output.author) {
                        // add MessageOutput to bucket
                        if let Some(vec) = agent_messages_hashmap.get_mut(&message_output.author) {
                            &vec.push(message_output.clone());
                        };
                    }
                    Some(message_output)
                },
                _ => None
            }

        })
        .collect();

    // construct return value
    let mut agent_messages_vec: Vec<MessagesByAgent> = Vec::new();
    for (agent, list) in agent_messages_hashmap.iter() {
        agent_messages_vec.push(
            MessagesByAgent {
                author: agent.to_owned(),
                messages: list.to_owned()
            }
        );
    }

    Ok(MessagesByAgentListWrapper(agent_messages_vec))
}

// TODO: change implementation once query! macro accepts timestamp range.
pub(crate) fn get_batch_messages_on_conversation(message_range: MessageRange) -> ExternResult<MessageListWrapper> {

    // batch parameters
    let timegap = 10; //in seconds
    let batch_size = 10; // number of messages

    let query_result = query!(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info!()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;
    // assume query results are sorted

    let mut message_output_vec: Vec<MessageOutput> = Vec::new();
    for element in query_result.0 {
        let header_hash = element.header_address().to_owned();
        let entry: Result<Option<MessageEntry>, SerializedBytesError> = element.into_inner().1.to_app_option();
        match entry {
            Ok(Some(message_entry)) => {
                if message_output_vec.len() <= 0 
                || (message_output_vec.len() <= batch_size && message_range.last_message_timestamp_seconds - message_entry.timestamp.0 < timegap) {
                    if message_entry.author == message_range.author {
                        if message_entry.timestamp.0 <= message_range.last_message_timestamp_seconds {
                            let message_output = MessageOutput {
                                header: header_hash,
                                author: message_entry.author,
                                receiver: message_entry.receiver,
                                payload: message_entry.payload,
                                timestamp: message_entry.timestamp
                            };
                            message_output_vec.push(message_output);
                        }
                    };
                    continue
                };
                break
            },
            _ => continue
        }
    };

    Ok(MessageListWrapper(message_output_vec))
}

// HELPER FUNCTIONS
fn address_deduper(agent_vec: Vec<AgentPubKey>) -> Vec<AgentPubKey> {
    let mut ids = agent_vec;
    ids.sort();
    ids.dedup_by(|a, b| a==b);
    return ids
}
