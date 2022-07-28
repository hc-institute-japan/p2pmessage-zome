use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_integrity_types::{P2PMessageReceipt, Status};

use crate::{
    helpers::{get_receipt_from_chain, get_message_from_chain}, 
    utils::error
};

#[allow(dead_code)]
pub fn commit_receipt_to_sender_chain_handler(
    receipt_hash: EntryHash,
) -> ExternResult<HashMap<String, P2PMessageReceipt>> {
    let receipt = get_receipt_from_chain(receipt_hash)?;

    if let Status::Delivered { .. } = receipt.status {
        let message = get_message_from_chain(receipt.id[0].clone())?;

        if message.receiver.clone() == agent_info()?.agent_latest_pubkey {
            let receive_call_result: ZomeCallResponse = call_remote(
                message.author.clone(),
                zome_info()?.name,
                "receive_receipt".into(),
                None,
                &receipt,
            )?;

            match receive_call_result {
                ZomeCallResponse::Ok(extern_io) => {
                    let result = extern_io.decode();
                    match result {
                        Ok(map) => return Ok(map),
                        Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e))))
                    }
                },
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
    }
    return error(
        "Sorry. Was not able to commit the delivered receipt to the sender's chain. Something went wrong.",
    );
}
