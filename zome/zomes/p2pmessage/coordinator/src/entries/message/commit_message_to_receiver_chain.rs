use hdk::prelude::*;

use p2pmessage_integrity_types::*;
use p2pmessage_coordinator_types::*;

use crate::{
    helpers::{get_file_from_chain, get_message_from_chain}, 
    utils::error
};

pub fn commit_message_to_receiver_chain_handler(
    message_hash: EntryHash,
) -> ExternResult<P2PMessageReceipt> {
    let message = get_message_from_chain(message_hash.clone())?;

    if agent_info()?.agent_latest_pubkey == message.author.clone() {
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
                let received_receipt_result: Result<P2PMessageReceipt, SerializedBytesError> = extern_io.decode();
                match received_receipt_result {
                    Ok(received_receipt) => return Ok(received_receipt),
                    Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e))))
                }
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

    return error(
        "Sorry. Was not able to commit the message to receiver's chain. Something went wrong.",
    );
}
