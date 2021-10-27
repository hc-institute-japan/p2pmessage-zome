use hdk::prelude::*;

use file_types::Payload;

use super::{P2PMessage, ReceiveMessageInput};
use crate::helpers::get_file_from_chain;
use crate::utils::error;

pub fn commit_message_to_receiver_chain_handler(message: P2PMessage) -> ExternResult<()> {
    let receive_input = ReceiveMessageInput {
        message: message.clone(),
        file: match message.payload {
            Payload::Text { .. } => None,
            Payload::File { ref metadata, .. } => {
                let file_bytes = get_file_from_chain(metadata.to_owned().file_hash)?;
                Some(file_bytes)
            }
        },
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
            debug!(
                "nicko commit message to receiver chain message {:?} receipt {:?}",
                message.clone(),
                extern_io.decode()?
            );
            Ok(())
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
