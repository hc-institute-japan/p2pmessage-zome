use hdk::prelude::*;

use p2pmessage_coordinator_types::*;
use p2pmessage_integrity_types::*;

use crate::utils::error;

use super::utils::this_zome_index;

// test_stub: this zome function is a test function to test get by timestamp
pub fn send_message_with_timestamp_handler(
    message_input: MessageWithTimestampInput,
) -> ExternResult<((EntryHash, P2PMessageData), (EntryHash, P2PMessageReceipt))> {
    let message = P2PMessage {
        author: agent_info()?.agent_latest_pubkey,
        receiver: message_input.receiver,
        payload: match message_input.payload {
            PayloadInput::Text { ref payload } => Payload::Text {
                payload: payload.to_owned(),
            },
            PayloadInput::File {
                ref metadata,
                ref file_type,
                ref file_bytes,
            } => {
                let p2pfile = P2PFileBytes(file_bytes.clone());
                let file_hash = hash_entry(&p2pfile)?;
                Payload::File {
                    metadata: FileMetadata {
                        file_name: metadata.file_name.clone(),
                        file_size: metadata.file_size.clone(),
                        file_type: metadata.file_type.clone(),
                        file_hash: file_hash,
                    },
                    file_type: file_type.clone(),
                }
            }
        },
        time_sent: message_input.timestamp,
        reply_to: message_input.reply_to,
    };

    let file = match message_input.payload {
        PayloadInput::Text { .. } => None,
        PayloadInput::File { ref file_bytes, .. } => Some(P2PFileBytes((*file_bytes).clone())),
    };

    let receive_input = ReceiveMessageInput {
        message: message.clone(),
        file: file.clone(),
    };

    let receive_call_result: ZomeCallResponse = call_remote(
        message.receiver.clone(),
        zome_info()?.name,
        "receive_message".into(),
        None,
        &receive_input,
    )?;

    match receive_call_result {
        ZomeCallResponse::Ok(extern_io) => {
            let message_entry = Entry::App(message.clone().try_into()?);
            host_call::<CreateInput, ActionHash>(
                __hc__create_1,
                CreateInput::new(
                    EntryDefLocation::app(this_zome_index()?, 0),
                    EntryVisibility::Private,
                    message_entry.clone(),
                    ChainTopOrdering::Relaxed,
                ),
            )?;

            let received_receipt_result: Result<P2PMessageReceipt, SerializedBytesError> =
                extern_io.decode();
            match received_receipt_result {
                Ok(received_receipt) => {
                    let received_receipt_entry = Entry::App(received_receipt.clone().try_into()?);
                    host_call::<CreateInput, ActionHash>(
                        __hc__create_1,
                        CreateInput::new(
                            EntryDefLocation::app(this_zome_index()?, 1),
                            EntryVisibility::Private,
                            received_receipt_entry,
                            ChainTopOrdering::Relaxed,
                        ),
                    )?;

                    if let PayloadInput::File { file_bytes, .. } = message_input.payload {
                        let p2pfile = P2PFileBytes(file_bytes.clone());
                        let p2pfile_entry = Entry::App(p2pfile.clone().try_into()?);
                        host_call::<CreateInput, ActionHash>(
                            __hc__create_1,
                            CreateInput::new(
                                EntryDefLocation::app(this_zome_index()?, 3),
                                EntryVisibility::Private,
                                p2pfile_entry,
                                ChainTopOrdering::Relaxed,
                            ),
                        )?;
                        ()
                    };

                    let message_return;
                    if let Some(ref reply_to_hash) = message.reply_to {
                        let mut queried_messages: Vec<Record> = query(
                            QueryFilter::new()
                                .entry_type(EntryType::App(AppEntryDef::new(
                                    EntryDefIndex::from(0),
                                    this_zome_index()?,
                                    EntryVisibility::Private,
                                )))
                                .include_entries(true),
                        )?;
                        queried_messages.reverse();

                        for queried_message in queried_messages.clone().into_iter() {
                            if let Ok(message_entry) =
                                TryInto::<P2PMessage>::try_into(queried_message.clone())
                            {
                                let message_hash = hash_entry(&message_entry)?;

                                if *reply_to_hash == message_hash {
                                    let replied_to_message = P2PMessageReplyTo {
                                        hash: message_hash.clone(),
                                        author: message_entry.author,
                                        receiver: message_entry.receiver,
                                        payload: message_entry.payload,
                                        time_sent: message_entry.time_sent,
                                        reply_to: None,
                                    };

                                    message_return = P2PMessageData {
                                        author: message.author.clone(),
                                        receiver: message.receiver.clone(),
                                        payload: message.payload.clone(),
                                        time_sent: message.time_sent.clone(),
                                        reply_to: Some(replied_to_message),
                                    };

                                    return Ok((
                                        (hash_entry(&message)?, message_return),
                                        (hash_entry(&received_receipt)?, received_receipt),
                                    ));
                                }
                            } else {
                                continue;
                            }
                        }
                    }

                    message_return = P2PMessageData {
                        author: message.author.clone(),
                        receiver: message.receiver.clone(),
                        payload: message.payload.clone(),
                        time_sent: message.time_sent.clone(),
                        reply_to: None,
                    };

                    Ok((
                        (hash_entry(&message)?, message_return),
                        (hash_entry(&received_receipt)?, received_receipt),
                    ))
                }
                Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e)))),
            }
        }
        ZomeCallResponse::Unauthorized(..) => {
            return error("Sorry, something went wrong. [Authorization error]");
        }
        ZomeCallResponse::NetworkError(e) => {
            return error(&e);
        }
        ZomeCallResponse::CountersigningSession(_e) => {
            return error("Sorry, something went wrong. [Countersigning error]");
        }
    }
}
