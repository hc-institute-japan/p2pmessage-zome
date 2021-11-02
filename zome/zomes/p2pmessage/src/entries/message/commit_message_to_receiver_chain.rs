use hdk::prelude::*;

use file_types::Payload;

use super::{P2PMessage, P2PMessageReceipt, ReceiveMessageInput};
use crate::helpers::get_file_from_chain;
use crate::utils::error;

pub fn commit_message_to_receiver_chain_handler(
    message: P2PMessage,
) -> ExternResult<P2PMessageReceipt> {
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
            let received_receipt = extern_io.decode()?;
            call_remote(
                agent_info()?.agent_latest_pubkey.clone(),
                zome_info()?.name,
                "receive_receipt".into(),
                None,
                &received_receipt,
            )?;

            Ok(received_receipt)
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
