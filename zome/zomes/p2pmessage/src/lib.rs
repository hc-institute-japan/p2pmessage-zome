use hdk::prelude::*;

mod entries;
mod utils;
use entries::message::{self};

use message::*;

entry_defs![
    P2PMessage::entry_def(),
    P2PMessageReceipt::entry_def(),
    P2PFileBytes::entry_def()
];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(WasmError::Guest(String::from(reason)))
}

pub fn err<T>(code: &str, message: &str) -> ExternResult<T> {
    Err(WasmError::Guest(format!(
        "{{\"code\": \"{}\", \"message\": \"{}\"}}",
        code, message
    )))
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}

#[hdk_extern]
fn init(_:())->ExternResult<InitCallbackResult> {
    return message::init::handler();
}

#[hdk_extern]
fn send_message(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    return message::send_message::handler(message_input);
}

#[hdk_extern]
fn read_message(read_receipt_input: ReadReceiptInput) -> ExternResult<ReceiptContents> {
    return message::read_message::handler(read_receipt_input);
}

#[hdk_extern]
fn receive_message(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    return message::receive_message::handler(input);
}

#[hdk_extern]
fn get_latest_messages(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
    return message::get_latest_messages::handler(batch_size);
}

#[hdk_extern]
fn get_next_batch_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    return message::get_next_batch_messages::handler(filter);
}

#[hdk_extern]
fn get_messages_by_agent_by_timestamp(filter: P2PMessageFilterAgentTimestamp ) -> ExternResult<P2PMessageHashTables> {
    return message::get_messages_by_agent_by_timestamp::handler(filter);
}

#[hdk_extern]
fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    return message::typing::handler(typing_info);
}

#[hdk_extern]
fn receive_read_receipt(receipt: P2PMessageReceipt) -> ExternResult<ReceiptContents> {
    return message::receive_read_receipt::handler(receipt);
}

//COMENTED METHODS

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