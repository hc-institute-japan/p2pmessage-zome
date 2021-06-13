use hdk::prelude::*;

use file_types::{FileMetadata, Payload, PayloadInput};

use super::{
    MessageAndReceipt, MessageInput, P2PFileBytes, P2PMessage, P2PMessageReceipt,
    ReceiveMessageInput,
};
use crate::utils::error;

pub fn send_message_handler(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    // TODO: check if receiver is blocked

    let now = sys_time()?;

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
                create_entry(&p2pfile)?;
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
        time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
        reply_to: message_input.reply_to,
    };

    let receipt = P2PMessageReceipt::from_message(message.clone())?;
    create_entry(&message)?;
    create_entry(&receipt)?;

    let file = match message_input.payload {
        PayloadInput::Text { .. } => None,
        PayloadInput::File { file_bytes, .. } => Some(P2PFileBytes(file_bytes)),
    };

    // create message input to receive function of recipient
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
            let receipt: P2PMessageReceipt = extern_io.decode()?;
            create_entry(&receipt)?;

            // TODO: CREATE AND RETURN ELEMENT HERE
            Ok(MessageAndReceipt(
                (hash_entry(&message)?, message),
                (hash_entry(&receipt)?, receipt),
            ))
        }
        // This case shouldn't happen because of unrestricted access to receive message
        // keeping it here for exhaustive matching
        ZomeCallResponse::Unauthorized(_, _, _, _) => {
            return error("Sorry, something went wrong. [Authorization error]");
        }
        // Error that might happen when
        ZomeCallResponse::NetworkError(_e) => {
            return error("Sorry, something went wrong. [Source chain error]");
        }
    }
}
