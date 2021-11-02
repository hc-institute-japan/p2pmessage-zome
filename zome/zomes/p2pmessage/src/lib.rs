#[allow(dead_code)]
use hdk::prelude::*;

mod entries;
mod utils;
use entries::message::{self};
use std::collections::HashMap;

use message::*;

use message::commit_message_to_receiver_chain::commit_message_to_receiver_chain_handler;
// use message::commit_receipt_to_sender_chain::commit_receipt_to_sender_chain_handler;
use message::get_adjacent_messages::get_adjacent_messages_handler;
use message::get_file_bytes::get_file_bytes_handler;
use message::get_latest_messages::get_latest_messages_handler;
use message::get_messages_by_agent_by_timestamp::get_messages_by_agent_by_timestamp_handler;
use message::get_next_messages::get_next_messages_handler;
use message::get_pinned_messages::get_pinned_messages_handler;
use message::get_previous_messages::get_previous_messages_handler;
use message::helpers::get_message_from_chain;
use message::init::init_handler;
use message::pin_message::pin_message_handler;
use message::read_message::read_message_handler;
use message::receive_message::receive_message_handler;
use message::receive_receipt::receive_receipt_handler;
use message::send_message::send_message_handler;
use message::send_message_with_timestamp::send_message_with_timestamp_handler;
use message::sync_pins::sync_pins_handler;
use message::typing::typing_handler;

entry_defs![
    P2PMessage::entry_def(),
    P2PMessageReceipt::entry_def(),
    P2PFileBytes::entry_def(),
    P2PMessagePin::entry_def()
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
    let signal_detail: SignalDetails = signal.decode()?;
    emit_signal(&signal_detail)?;
    Ok(())
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    return init_handler();
}

#[hdk_extern]
fn post_commit(headers: Vec<SignedHeaderHashed>) -> ExternResult<PostCommitCallbackResult> {
    for signed_header in headers.into_iter() {
        match signed_header.header() {
            Header::Create(create) => {
                let agent_pubkey = agent_info()?.agent_latest_pubkey;
                match &create.entry_type {
                    EntryType::App(apptype) => match apptype.id() {
                        EntryDefIndex(0) => {
                            let message_entry = get_message_from_chain(create.entry_hash.clone())?;
                            if agent_pubkey == message_entry.author.clone() {
                                commit_message_to_receiver_chain_handler(message_entry.clone())?;
                            }
                        }
                        EntryDefIndex(1) => {
                            ()
                            // let receipt_entry = get_receipt_from_chain(create.entry_hash.clone())?;
                            // let receipt_status: Status = receipt_entry.status.clone();
                            // if let Status::Delivered { .. } = receipt_status {
                            //     let message_entry =
                            //         get_message_from_chain(receipt_entry.id[0].clone())?;
                            //     if agent_pubkey == message_entry.receiver {
                            //         let input = ReceiveReceiptInput {
                            //             receipt: receipt_entry.clone(),
                            //             receiver: message_entry.author,
                            //         };
                            //         commit_receipt_to_sender_chain_handler(input)?;
                            //     }
                            // }
                        }
                        _ => (),
                    },
                    _ => (),
                };
            }
            _ => (),
        }
    }
    Ok(PostCommitCallbackResult::Success)
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
fn commit_message_to_receiver_chain(
    input: MessageWithTimestampInput,
) -> ExternResult<P2PMessageReceipt> {
    let message = P2PMessage::from_input(input)?;
    return commit_message_to_receiver_chain_handler(message);
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
