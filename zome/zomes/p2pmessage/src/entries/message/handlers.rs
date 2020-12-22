use hdk3::prelude::*;
use crate::{timestamp::Timestamp};
use crate::utils::{
    try_from_element,
    address_deduper
};

use super::{
    MessageEntry,
    MessageParameter,
    MessageInput,
    MessageParameterOption,
    MessageListWrapper,
    MessagesByAgent,
    MessagesByAgentListWrapper,
    AgentListWrapper,
    MessageRange,
    Status,
    Signal,
    Reply,
    TypingInfo,
    TypingSignal
};

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
    let mut emit_functions: GrantedFunctions = HashSet::new();
    emit_functions.insert((zome_info()?.zome_name, "emit_typing".into()));

    create_cap_grant(CapGrantEntry {
        tag: "receive".into(),
        access: ().into(),
        functions: receive_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        access: ().into(),
        functions: emit_functions
    })?;

    Ok(InitCallbackResult::Pass)
}

pub(crate) fn send_message(message_input: MessageInput) -> ExternResult<MessageParameterOption> {

    let now = sys_time()?;
    let message = MessageParameter {
        author: agent_info()?.agent_latest_pubkey,
        receiver: message_input.receiver.clone(),
        payload: message_input.payload,
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        time_received: None,
        status: Status::Sent,
        reply_to: None
    };

    let payload: SerializedBytes = message.try_into()?;

    debug!("nicko start remote call");
    match call_remote(
        message_input.receiver,
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &payload
    )? {
        ZomeCallResponse::Ok(output) => {
            let message_output = MessageParameterOption::try_from(output.into_inner())?;
            debug!("nicko remote ok returned");
            // let message_output: MessageParameterOption = output.into_inner().try_into()?;
            match message_output.0 {
                Some(message_output) => {
                    let message_entry = MessageEntry::from_parameter(message_output.clone());
                    create_entry(&message_entry)?;
                    Ok(MessageParameterOption(Some(message_output)))
                },
                None => {
                    Ok(MessageParameterOption(None))
                }
            }
        },
        ZomeCallResponse::NetworkError(e) => { 
            debug!("nicko remote net error returned");
            // Err(HdkError::ZomeCallNetworkError(e))

        },
        ZomeCallResponse::Unauthorized(_c,_z,_f,_p) => {
            debug!("nicko remote unauthorized returned");
            crate::err("401", "This agent has no proper authorization")
        }
        
    }
}

pub(crate) fn reply_to_message(reply_input: Reply) -> ExternResult<MessageParameterOption> {
    // construct entry hash
    let message_entry = MessageEntry::from_parameter(reply_input.replied_message.clone());
    let message_entry_hash = hash_entry(&message_entry)?;

    // build entry structure to be passed
    let now = sys_time()?;
    let reply_message_payload = MessageParameter {
        author: agent_info()?.agent_latest_pubkey,
        receiver: reply_input.replied_message.author.clone(),
        payload: reply_input.reply,
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        time_received: None,
        status: Status::Sent,
        reply_to: Some(message_entry_hash),
    };

    // let payload: SerializedBytes = reply_message_payload.try_into()?;

    match call_remote(
        reply_input.replied_message.author,
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &reply_message_payload
    )? {
        ZomeCallResponse::Ok(output) => {
            let message_output: MessageParameterOption = output.into_inner().try_into()?;
            match message_output.0 {
                Some(message_output) => {
                    let message_entry = MessageEntry::from_parameter(message_output.clone());
                    create_entry(&message_entry)?;
                    Ok(MessageParameterOption(Some(message_output)))
                }
                None => Ok(MessageParameterOption(None)),
            }
        },
        ZomeCallResponse::Unauthorized(_c,_z,_f,_p) => {
            crate::err("401", "This agent has no proper authorization")
        },
        ZomeCallResponse::NetworkError(e) => Err(HdkError::ZomeCallNetworkError(e))
        
    }
}

pub(crate) fn receive_message(
    message_input: MessageParameter,
) -> ExternResult<MessageParameterOption> {
    debug!("nicko received reached");
    let mut message_entry = MessageEntry::from_parameter(message_input.clone());
    let now = sys_time()?;
    message_entry.time_received = Some(Timestamp(now.as_secs() as i64, now.subsec_nanos()));
    message_entry.status = Status::Delivered;

    debug!("nicko in receive before create");
    match create_entry(&message_entry) {
        Ok(header) => {
            debug!("nicko in receive ok create");
            debug!(header);
            let message_output = MessageParameter::from_entry(message_entry);
            debug!("nicko in receive parameter");
            Ok(MessageParameterOption(Some(message_output)))
        },
        _ => {
            debug!("nicko in receive error create");
            Ok(MessageParameterOption(None))
        }
    }
}

pub(crate) fn get_all_messages() -> ExternResult<MessageListWrapper> {
    let query_result = query(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    let message_vec: Vec<MessageParameter> = query_result.0
        .into_iter()
        .filter_map(|el| {
            let entry = try_from_element(el);
            match entry {
                Ok(message_entry) => {
                    let message_output = MessageParameter::from_entry(message_entry);
                    Some(message_output)
                },
                _ => None
            }
        })
        .collect();

    Ok(MessageListWrapper(message_vec))
}

pub(crate) fn get_all_messages_from_addresses(agent_list: AgentListWrapper) -> ExternResult<MessagesByAgentListWrapper> {
    let deduped_agents = address_deduper(agent_list.0);

    let query_result = query(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    let mut agent_messages_hashmap = std::collections::HashMap::new();
    for agent in deduped_agents {
        let message_list: Vec<MessageParameter> = Vec::new();
        agent_messages_hashmap.insert(agent, message_list);                                                                                                                                                                               
    };

    for element in query_result.0.into_iter() {
        let entry = try_from_element(element);
        match entry {
            Ok(message_entry) => {
                let message_output = MessageParameter::from_entry(message_entry);
                if agent_messages_hashmap.contains_key(&message_output.clone().author) {
                    if let Some(vec) = agent_messages_hashmap.get_mut(&message_output.clone().author) {
                        vec.push(message_output.clone());
                    } else { () }
                } else { () }
            },
            _ => ()
        };
    };

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

// TODO: change implementation once query accepts timestamp range.
pub(crate) fn get_batch_messages_on_conversation(message_range: MessageRange) -> ExternResult<MessageListWrapper> {

    let timegap = 10; //in seconds
    let batch_size = 10; // number of messages

    let query_result = query(
        QueryFilter::new()
        .entry_type(
            EntryType::App(
                AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info()?.zome_id,
                    EntryVisibility::Public
                )
            )
        )
        .include_entries(true)
    )?;

    let mut message_output_vec: Vec<MessageParameter> = Vec::new();
    for element in query_result.0 {
        let entry = try_from_element::<MessageEntry>(element);
        match entry {
            Ok(message_entry) => {
                if message_output_vec.len() <= 0 
                || (message_output_vec.len() <= batch_size && message_range.last_message_timestamp_seconds - message_entry.time_sent.0 < timegap) {
                    if message_entry.author == message_range.author {
                        if message_entry.time_sent.0 <= message_range.last_message_timestamp_seconds {
                            let message_output = MessageParameter::from_entry(message_entry);
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

fn _emit_typing(typing_info: TypingInfo) -> ExternResult<()> {
    emit_signal(&Signal::Typing(TypingSignal {
        kind: "message_sent".to_owned(),
        agent: typing_info.agent.to_owned(),
        is_typing: typing_info.is_typing
    }))?;
    Ok(())
}

pub(crate) fn typing(typing_info: TypingInfo) -> ExternResult<()> {
    let payload =         TypingInfo {
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
