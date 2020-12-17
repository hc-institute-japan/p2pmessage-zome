use hdk3::prelude::*;
mod entries;
mod utils;
use entries::message;

use message::{
    MessageEntry,
    MessageInput,
    MessageOutput,
    MessageOutputOption,
    MessageListWrapper,
    AgentListWrapper,
    MessagesByAgentListWrapper,
    MessageRange
};

entry_defs![MessageEntry::entry_def()];

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
fn send_message(message_input: MessageInput) -> ExternResult<MessageOutputOption> {
    message::handlers::send_message(message_input)
}

#[hdk_extern]
fn receive_message(message_input: MessageOutput) -> ExternResult<MessageOutputOption> {
    message::handlers::receive_message(message_input)
}

#[hdk_extern]
fn get_all_messages(_: ()) -> ExternResult<MessageListWrapper> {
    message::handlers::get_all_messages()
}

#[hdk_extern]
fn get_all_messages_from_addresses(agent_list: AgentListWrapper) -> ExternResult<MessagesByAgentListWrapper> {
    message::handlers::get_all_messages_from_addresses(agent_list)
}

#[hdk_extern]
fn get_batch_messages_on_conversation(message_range: MessageRange) -> ExternResult<MessageListWrapper> {
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
