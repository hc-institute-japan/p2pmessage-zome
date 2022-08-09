use hdk::prelude::*;

use p2pmessage_coordinator_types::*;
use p2pmessage_integrity_types::*;

use crate::receive_receipt::receive_receipt_handler;

pub fn send_message_handler(
    message_input: MessageInput,
) -> ExternResult<(EntryHash, P2PMessageData)> {
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

    let message_entry = Entry::App(message.clone().try_into()?);
    let zome_info = zome_info()?;
    host_call::<CreateInput, ActionHash>(
        __create,
        CreateInput::new(
            EntryDefLocation::app(zome_info.id, 0),
            EntryVisibility::Private,
            message_entry.clone(),
            ChainTopOrdering::Relaxed,
        ),
    )?;
    debug!("create_entry message");

    if let PayloadInput::File { ref file_bytes, .. } = message_input.payload {
        let p2pfile = P2PFileBytes(file_bytes.clone());
        let p2pfile_entry = Entry::App(p2pfile.clone().try_into()?);
        host_call::<CreateInput, ActionHash>(
            __create,
            CreateInput::new(
                EntryDefLocation::app(zome_info.id, 0),
                EntryVisibility::Private,
                p2pfile_entry,
                ChainTopOrdering::Relaxed,
            ),
        )?;
        ()
    };

    // message self
    if message.author.clone() == message.receiver.clone() {
        let received_receipt = P2PMessageReceipt {
            id: vec![hash_entry(&message)?],
            status: Status::Read {
                timestamp: sys_time()?,
            },
        };

        let _res = receive_receipt_handler(received_receipt.clone())?;
    }

    let message_return;
    if let Some(ref reply_to_hash) = message.reply_to {
        let queried_messages: Vec<Record> = query(
            QueryFilter::new()
                .entry_type(EntryType::App(AppEntryType::new(
                    EntryDefIndex::from(0),
                    zome_info.id,
                    EntryVisibility::Private,
                )))
                .include_entries(true),
        )?;

        for queried_message in queried_messages.clone().into_iter() {
            if let Ok(message_entry) = TryInto::<P2PMessage>::try_into(queried_message.clone()) {
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

                    return Ok((hash_entry(&message)?, message_return));
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

    Ok((hash_entry(&message)?, message_return))
}
