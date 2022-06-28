#[allow(dead_code)]
use hdk::prelude::*;

mod entries;
mod utils;
use entries::message::{self};
use std::collections::HashMap;

use message::*;

use message::commit_message_to_receiver_chain::commit_message_to_receiver_chain_handler;
use message::commit_receipt_to_sender_chain::commit_receipt_to_sender_chain_handler;
use message::get_adjacent_messages::get_adjacent_messages_handler;
use message::get_file_bytes::get_file_bytes_handler;
use message::get_latest_messages::get_latest_messages_handler;
use message::get_messages_by_agent_by_timestamp::get_messages_by_agent_by_timestamp_handler;
use message::get_next_messages::get_next_messages_handler;
use message::get_pinned_messages::get_pinned_messages_handler;
use message::get_previous_messages::get_previous_messages_handler;
use message::init::init_handler;
use message::pin_message::pin_message_handler;
use message::read_message::read_message_handler;
use message::receive_message::receive_message_handler;
use message::receive_receipt::receive_receipt_handler;
use message::send_message::send_message_handler;
use message::send_message_with_timestamp::send_message_with_timestamp_handler;
use message::sync_pins::sync_pins_handler;
use message::typing::typing_handler;
use message::helpers::get_message_from_chain;

// entry_defs![
//     P2PMessage::entry_def(),
//     P2PMessageReceipt::entry_def(),
//     P2PFileBytes::entry_def(),
//     P2PMessagePin::entry_def()
// ];

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(String::from(reason))))
}

pub fn err<T>(code: &str, message: &str) -> ExternResult<T> {
    Err(wasm_error!(WasmErrorInner::Guest(format!(
        "{{\"code\": \"{}\", \"message\": \"{}\"}}",
        code, message
    ))))
}

#[hdk_extern]
fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
    let signal_detail_result: Result<SignalDetails, SerializedBytesError> = signal.decode();
    match signal_detail_result {
        Ok(signal_detail) => {
            emit_signal(&signal_detail)?;
            return Ok(())
        },
        Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e))))
    }
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    return init_handler();
}

#[hdk_extern(infallible)]
fn post_commit(actions: Vec<SignedActionHashed>) {
    for signed_action in actions.into_iter() {
        match signed_action.action() {
            Action::Create(create) => {
                match &create.entry_type {
                    EntryType::App(apptype) => match apptype.id() {
                        EntryDefIndex(0) => {
                            let message = get_message_from_chain(create.entry_hash.clone()).unwrap();
                            if message.author == message.receiver {
                                debug!("post commit return ()");
                                return ()
                            } else {
                            let _res =
                                commit_message_to_receiver_chain_handler(create.entry_hash.clone());
                            }
                        },
                        EntryDefIndex(1) => {
                            let _res =
                                commit_receipt_to_sender_chain_handler(create.entry_hash.clone());
                        }
                        _ => (),
                    },
                    _ => (),
                };
            }
            _ => (),
        }
    }
}

#[hdk_extern]
fn send_message(message_input: MessageInput) -> ExternResult<(EntryHash, P2PMessageData)> {
    return send_message_handler(message_input);
}

#[hdk_extern] // test function for sending messsages in particular dates
fn send_message_with_timestamp(
    message_input: MessageWithTimestampInput,
) -> ExternResult<((EntryHash, P2PMessageData), (EntryHash, P2PMessageReceipt))> {
    return send_message_with_timestamp_handler(message_input);
}

#[hdk_extern]
fn commit_message_to_receiver_chain(input: EntryHash) -> ExternResult<P2PMessageReceipt> {
    return commit_message_to_receiver_chain_handler(input);
}

#[hdk_extern]
fn receive_message(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    return receive_message_handler(input);
}

#[hdk_extern]
fn read_message(
    read_message_input: ReadMessageInput,
) -> ExternResult<HashMap<String, P2PMessageReceipt>> {
    return read_message_handler(read_message_input);
}

#[hdk_extern]
fn get_latest_messages(batch_size: u8) -> ExternResult<P2PMessageHashTables> {
    return get_latest_messages_handler(batch_size);
}

#[hdk_extern]
fn get_messages_by_agent_by_timestamp(
    filter: P2PMessageFilterAgentTimestamp,
) -> ExternResult<P2PMessageHashTables> {
    return get_messages_by_agent_by_timestamp_handler(filter);
}

#[hdk_extern]
fn get_previous_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    return get_previous_messages_handler(filter);
}

#[hdk_extern]
fn get_next_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    return get_next_messages_handler(filter);
}

#[hdk_extern]
fn typing(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    return typing_handler(typing_info);
}

#[hdk_extern]
fn receive_receipt(receipt: P2PMessageReceipt) -> ExternResult<HashMap<String, P2PMessageReceipt>> {
    return receive_receipt_handler(receipt);
}

#[hdk_extern]
fn get_file_bytes(file_hashes: Vec<EntryHash>) -> ExternResult<HashMap<String, P2PFileBytes>> {
    return get_file_bytes_handler(file_hashes);
}

#[hdk_extern]
fn pin_message(pin_message_input: PinMessageInput) -> ExternResult<HashMap<String, P2PMessagePin>> {
    return pin_message_handler(pin_message_input);
}

#[hdk_extern]
fn sync_pins(pin: P2PMessagePin) -> ExternResult<HashMap<String, P2PMessagePin>> {
    return sync_pins_handler(pin);
}

#[hdk_extern]
fn get_pinned_messages(conversant: AgentPubKey) -> ExternResult<P2PMessageHashTables> {
    return get_pinned_messages_handler(conversant);
}

#[hdk_extern]
fn get_adjacent_messages(filter: P2PMessageFilterBatch) -> ExternResult<P2PMessageHashTables> {
    return get_adjacent_messages_handler(filter);
}
