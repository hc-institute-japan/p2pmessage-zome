use hdk3::prelude::*;
mod entries;
mod utils;
use entries::message;

use message::*;

entry_defs![
    P2PMessage::entry_def(),
    P2PMessageReceipt::entry_def(),
    P2PFileBytes::entry_def()
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
fn send_message(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    message::handlers::send_message(message_input)
}

#[hdk_extern]
fn receive_message(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    message::handlers::receive_message(input)
}

#[hdk_extern]
fn get_latest_messages(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
    message::handlers::get_latest_messages(batch_size)
}

#[hdk_extern]
fn get_next_batch_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    message::handlers::get_next_batch_messages(filter)
}

#[hdk_extern]
fn get_messages_by_agent_by_timestamp(
    filter: P2PMessageFilterAgentTimestamp,
) -> ExternResult<P2PMessageHashTables> {
    message::handlers::get_messages_by_agent_by_timestamp(filter)
}

// #[hdk_extern]
// fn send_message_async(message_input: MessageInput) -> ExternResult<MessageParameter> {
//     message::handlers::send_message_async(message_input)
// }

// #[hdk_extern]
// fn notify_delivery(message: P2PMessage) -> ExternResult<BooleanWrapper> {
//     message::handlers::notify_delivery(message)
// }

// #[hdk_extern]
// fn notify_delivery_async(input: NotifyAsyncInput) -> ExternResult<BooleanWrapper> {
//     message::handlers::notify_delivery_async(input)
// }

// #[hdk_extern]
// fn fetch_async_messages(_: ()) -> ExternResult<MessageListWrapper> {
//     message::handlers::fetch_async_messages()
// }

#[hdk_extern]
fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    message::handlers::typing(typing_info)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
