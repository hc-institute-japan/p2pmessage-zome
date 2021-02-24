use hdk3::prelude::*;
mod entries;
mod utils;
use entries::message;

use message::*;

entry_defs![
    P2PMessage::entry_def(),
    P2PMessageAsync::entry_def(),
    P2PMessage::entry_def()
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
fn send_message(message_input: MessageInput) -> ExternResult<MessageParameter> {
    message::handlers::send_message(message_input)
}

#[hdk_extern]
fn send_message_async(message_input: MessageInput) -> ExternResult<MessageParameter> {
    message::handlers::send_message_async(message_input)
}

#[hdk_extern]
fn receive_message(message_input: MessageParameter) -> ExternResult<MessageParameter> {
    message::handlers::receive_message(message_input)
}

#[hdk_extern]
fn notify_delivery(message_parameter: MessageParameter) -> ExternResult<BooleanWrapper> {
    message::handlers::notify_delivery(message_parameter)
}

#[hdk_extern]
fn notify_delivery_async(input: NotifyAsyncInput) -> ExternResult<BooleanWrapper> {
    message::handlers::notify_delivery_async(input)
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
fn fetch_async_messages(_: ()) -> ExternResult<MessageListWrapper> {
    message::handlers::fetch_async_messages()
}

#[hdk_extern]
fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    message::handlers::typing(typing_info)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}

#[hdk_extern]
fn read_message(read_receipt_input: ReadReceiptInput) -> ExternResult<ReceiptContents> {
    message::handlers::read_message(read_receipt_input)
}
