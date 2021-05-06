use hdk::prelude::*;

mod entries;
mod utils;
use entries::message::{self};

use message::*;

use message::get_file_bytes::get_file_bytes_handler;
use message::get_latest_messages::get_latest_messages_handler;
use message::get_messages_by_agent_by_timestamp::get_messages_by_agent_by_timestamp_handler;
use message::get_next_batch_messages::get_next_batch_messages_handler;
use message::init::init_handler;
use message::read_message::read_message_handler;
use message::receive_message::receive_message_handler;
use message::receive_read_receipt::receive_read_receipt_handler;
use message::send_message::send_message_handler;
use message::typing::typing_handler;

entry_defs![
    P2PMessage::entry_def(),
    P2PMessageReceipt::entry_def(),
    P2PFileBytes::entry_def(),
    Path::entry_def()
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
fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
    let signal_detail = signal.decode()?;
    debug!("about to emit");
    emit_signal(&signal_detail)?;
    debug!("receive remote signal invoked, emitted signal");
    Ok(())
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    return init_handler();
}

#[hdk_extern]
fn send_message(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    //MessageAndReceipt
    return send_message_handler(message_input);
}

#[hdk_extern]
fn read_message(read_message_input: ReadMessageInput) -> ExternResult<ReceiptContents> {
    return read_message_handler(read_message_input);
}

#[hdk_extern]
fn receive_message(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    return receive_message_handler(input);
}

#[hdk_extern]
fn get_latest_messages(batch_size: BatchSize) -> ExternResult<P2PMessageHashTables> {
    return get_latest_messages_handler(batch_size);
}

#[hdk_extern]
fn get_next_batch_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    return get_next_batch_messages_handler(filter);
}

#[hdk_extern]
fn get_messages_by_agent_by_timestamp(
    filter: P2PMessageFilterAgentTimestamp,
) -> ExternResult<P2PMessageHashTables> {
    return get_messages_by_agent_by_timestamp_handler(filter);
}

#[hdk_extern]
fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    return typing_handler(typing_info);
}

#[hdk_extern]
fn receive_read_receipt(receipt: P2PMessageReceipt) -> ExternResult<ReceiptContents> {
    return receive_read_receipt_handler(receipt);
}

#[hdk_extern]
fn get_file_bytes(file_hashes: Vec<EntryHash>) -> ExternResult<FileContents> {
    return get_file_bytes_handler(file_hashes);
}
