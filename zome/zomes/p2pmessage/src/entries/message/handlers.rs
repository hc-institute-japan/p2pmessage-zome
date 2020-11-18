use hdk3::prelude::*;
use crate::{timestamp::Timestamp};
use crate::utils::{
    try_from_element,
    address_deduper
};
use hdk3::host_fn::call::call;

use super::{
    MessageEntry,
    MessageOutput,
    MessageInput,
    MessageOutputOption,
    MessageListWrapper,
    MessagesByAgent,
    MessagesByAgentListWrapper,
    AgentListWrapper,
    MessageRange
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
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info!()?.zome_name, "receive_message".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "receive".into(),
            access: ().into(),
            functions,
        }
    )?;
    Ok(InitCallbackResult::Pass)
}

pub(crate) fn send_message(message_input: MessageInput) -> ExternResult<MessageOutputOption> {    
    // build entry structure to be passed
    let now = sys_time!()?;
    let message = MessageOutput {
        author: agent_info!()?.agent_latest_pubkey,
        receiver: message_input.receiver.clone(),
        payload: message_input.payload,
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        time_received: None
    };

    let payload: SerializedBytes = message.try_into()?;

    match call_remote!(
        message_input.receiver,
        zome_info!()?.zome_name,
        "receive_message".into(),
        None,
        payload
    )? {
        ZomeCallResponse::Ok(output) => {
            let message_output: MessageOutputOption = output.into_inner().try_into()?;
            match message_output.0 {
                Some(message_output) => {
                    let message_entry = MessageEntry::from_output(message_output.clone());
                    create_entry!(&message_entry)?;
                    Ok(MessageOutputOption(Some(message_output)))
                },
                None => {
                    Ok(MessageOutputOption(None))
                }
            }
            
        },
        ZomeCallResponse::Unauthorized => {
            crate::error("{\"code\": \"401\", \"message\": \"This agent has no proper authorization\"}")
        }
    }
}

pub(crate) fn receive_message(message_input: MessageOutput) -> ExternResult<MessageOutputOption> {
    
    let mut message_entry = MessageEntry::from_output(message_input.clone());
    let now = sys_time!()?;
    message_entry.time_received = Some(Timestamp(now.as_secs() as i64, now.subsec_nanos()));

    match create_entry!(&message_entry) {
        Ok(_header) => {
            let message_output = MessageOutput::from_entry(message_entry);
            Ok(MessageOutputOption(Some(message_output)))
        },
        _ => {
            Ok(MessageOutputOption(None))
        }
    }
}

pub(crate) fn get_all_messages() -> ExternResult<MessageListWrapper> {
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
            let entry = try_from_element(el);
            match entry {
                Ok(message_entry) => {
                    let message_output = MessageOutput::from_entry(message_entry);
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

    let mut agent_messages_hashmap = std::collections::HashMap::new();
    for agent in deduped_agents {
        let message_list: Vec<MessageOutput> = Vec::new();
        agent_messages_hashmap.insert(agent, message_list);                                                                                                                                                                               
    };

    let _map_result = query_result.0
        .into_iter()
        .map(|el| {
            let entry = try_from_element(el);
            match entry {
                Ok(message_entry) => {
                    let message_output = MessageOutput::from_entry(message_entry);
                    if agent_messages_hashmap.contains_key(&message_output.author) {
                        if let Some(vec) = agent_messages_hashmap.get_mut(&message_output.author) {
                            &vec.push(message_output.clone());
                        };
                    }
                    Some(message_output)
                },
                _ => None
            }
        });

    let mut agent_messages_vec: Vec<MessagesByAgent> = Vec::new();
    for (agent, list) in agent_messages_hashmap.iter() {
        agent_messages_vec.push(
            MessagesByAgent {
                author: agent.to_owned(),
                messages: (*list.to_owned()).to_vec()
            }
        );
    }

    Ok(MessagesByAgentListWrapper(agent_messages_vec))
}

pub(crate) fn get_batch_messages_on_conversation(message_range: MessageRange) -> ExternResult<MessageListWrapper> {

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

    let mut message_output_vec: Vec<MessageOutput> = Vec::new();
    for element in query_result.0 {
        let entry = try_from_element::<MessageEntry>(element);
        match entry {
            Ok(message_entry) => {
                if message_output_vec.len() <= 0 
                || (message_output_vec.len() <= batch_size && message_range.last_message_timestamp_seconds - message_entry.time_sent.0 < timegap) {
                    if message_entry.author == message_range.author {
                        if message_entry.time_sent.0 <= message_range.last_message_timestamp_seconds {
                            let message_output = MessageOutput::from_entry(message_entry);
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
