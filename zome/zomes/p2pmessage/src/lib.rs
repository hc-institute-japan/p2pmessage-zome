use hdk3::prelude::*;
mod entries;
mod utils;
use entries::message;

use message::*;

entry_defs![
    MessageEntry::entry_def(),
    Preference::entry_def(),
    PerAgentPreference::entry_def(),
    PerGroupPreference::entry_def()
];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

pub fn err<T>(code: &str, message: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(format!(
        "{{\"code\": \"{}\", \"message\": \"{}\"}}",
        code, message
    ))))
}

#[hdk_extern]
fn send_message(message_input: MessageInput) -> ExternResult<MessageParameterOption> {
    message::handlers::send_message(message_input)
}

#[hdk_extern]
fn receive_message(message_input: MessageParameter) -> ExternResult<MessageParameterOption> {
    message::handlers::receive_message(message_input)
}

#[hdk_extern]
fn get_all_messages(_: ()) -> ExternResult<MessageListWrapper> {
    message::handlers::get_all_messages()
}

#[hdk_extern]
fn get_all_messages_from_addresses(
    agent_list: AgentListWrapper,
) -> ExternResult<MessagesByAgentListWrapper> {
    message::handlers::get_all_messages_from_addresses(agent_list)
}

#[hdk_extern]
fn get_batch_messages_on_conversation(
    message_range: MessageRange,
) -> ExternResult<MessageListWrapper> {
    message::handlers::get_batch_messages_on_conversation(message_range)
}

#[hdk_extern]
fn reply_to_message(reply_input: Reply) -> ExternResult<MessageParameterOption> {
    message::handlers::reply_to_message(reply_input)
}

#[hdk_extern]
fn typing(typing_info: TypingInfo) -> ExternResult<()> {
    message::handlers::typing(typing_info)
}

#[hdk_extern]
fn get_preference(_: ()) -> ExternResult<PreferenceWrapper> {
    handlers::get_preference()
}

#[hdk_extern]
fn set_preference(preference: PreferenceIO) -> ExternResult<()> {
    handlers::set_preference(preference)
}

#[hdk_extern]
fn set_per_agent_preference(preference: PerAgentPreferenceIO) -> ExternResult<()> {
    handlers::set_per_agent_preference(preference)
}

#[hdk_extern]
fn get_per_agent_preference(_: ()) -> ExternResult<PerAgentPreferenceWrapper> {
    handlers::get_per_agent_preference()
}
