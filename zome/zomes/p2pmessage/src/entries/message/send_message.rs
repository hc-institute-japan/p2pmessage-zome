use hdk::prelude::*;

use file_types::{FileMetadata, Payload, PayloadInput};

use super::{
    MessageDataAndReceipt, MessageInput, P2PFileBytes, P2PMessage, P2PMessageData,
    P2PMessageReceipt, P2PMessageReplyTo, ReceiveMessageInput,
};
use crate::utils::error;

pub fn send_message_handler(message_input: MessageInput) -> ExternResult<MessageDataAndReceipt> {
    // TODO: check if receiver is blocked

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
                // create_entry(&p2pfile)?;
                let p2pfile_entry = Entry::App(p2pfile.clone().try_into()?);
                host_call::<CreateInput, HeaderHash>(
                    __create,
                    CreateInput::new(
                        P2PFileBytes::entry_def().id,
                        p2pfile_entry,
                        ChainTopOrdering::Relaxed,
                    ),
                )?;
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
        time_sent: sys_time()?,
        reply_to: message_input.reply_to,
    };

    // create_entry(&message)?;
    let message_entry = Entry::App(message.clone().try_into()?);
    host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            P2PMessage::entry_def().id,
            message_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let file = match message_input.payload {
        PayloadInput::Text { .. } => None,
        PayloadInput::File { file_bytes, .. } => Some(P2PFileBytes(file_bytes)),
    };

    let receive_input = ReceiveMessageInput(message.clone(), file.clone());

    let receive_call_result: ZomeCallResponse = call_remote(
        message.receiver.clone(),
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &receive_input,
    )?;

    match receive_call_result {
        ZomeCallResponse::Ok(extern_io) => {
            let received_receipt: P2PMessageReceipt = extern_io.decode()?;
            // create_entry(&received_receipt)?;
            let received_receipt_entry = Entry::App(received_receipt.clone().try_into()?);
            host_call::<CreateInput, HeaderHash>(
                __create,
                CreateInput::new(
                    P2PMessageReceipt::entry_def().id,
                    received_receipt_entry,
                    ChainTopOrdering::Relaxed,
                ),
            )?;

            let queried_messages: Vec<Element> = query(
                QueryFilter::new()
                    .entry_type(EntryType::App(AppEntryType::new(
                        EntryDefIndex::from(0),
                        zome_info()?.zome_id,
                        EntryVisibility::Private,
                    )))
                    .include_entries(true),
            )?;

            let message_return;
            for queried_message in queried_messages.clone().into_iter() {
                let message_entry: P2PMessage = queried_message.try_into()?;
                let message_hash = hash_entry(&message_entry)?;

                if let Some(ref reply_to_hash) = message.reply_to {
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

                        return Ok(MessageDataAndReceipt(
                            (hash_entry(&message)?, message_return),
                            (hash_entry(&received_receipt)?, received_receipt),
                        ));
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

            Ok(MessageDataAndReceipt(
                (hash_entry(&message)?, message_return),
                (hash_entry(&received_receipt)?, received_receipt),
            ))
        }
        ZomeCallResponse::Unauthorized(_, _, _, _) => {
            return error("Sorry, something went wrong. [Authorization error]");
        }
        ZomeCallResponse::NetworkError(_e) => {
            return error("Sorry, something went wrong. [Network error]");
        }
        ZomeCallResponse::CountersigningSession(_e) => {
            return error("Sorry, something went wrong. [Countersigning error]");
        }
    }
}
