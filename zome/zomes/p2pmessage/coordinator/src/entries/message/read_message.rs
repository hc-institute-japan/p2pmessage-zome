use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_integrity_types::*;
use p2pmessage_coordinator_types::*;

use crate::utils::error;

pub fn read_message_handler(
    read_message_input: ReadMessageInput,
) -> ExternResult<HashMap<String, P2PMessageReceipt>> {
    let receipt = P2PMessageReceipt {
        id: read_message_input.message_hashes,
        status: Status::Read {
            timestamp: read_message_input.timestamp,
        },
    };

    let zome_call_response: ZomeCallResponse = call_remote(
        read_message_input.sender,
        zome_info()?.name,
        FunctionName("receive_receipt".into()),
        None,
        &receipt,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            let read_receipt_entry = Entry::App(receipt.try_into()?);
            
            host_call::<CreateInput, ActionHash>(
                __create,
                CreateInput::new(
                    EntryDefLocation::app(0, 0),
                    EntryVisibility::Private,
                    read_receipt_entry,
                    ChainTopOrdering::Relaxed,
                ),
            )?;

            let result = extern_io.decode();
            match result {
                Ok(map) => return Ok(map),
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
