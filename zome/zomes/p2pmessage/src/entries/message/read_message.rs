use crate::utils::error;
use hdk::prelude::*;
use std::collections::HashMap;

use super::{P2PMessageReceipt, ReadMessageInput, Status};

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
        zome_info()?.zome_name,
        FunctionName("receive_read_receipt".into()),
        None,
        &receipt,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            let read_receipt_entry = Entry::App(receipt.try_into()?);
            host_call::<CreateInput, HeaderHash>(
                __create,
                CreateInput::new(
                    P2PMessageReceipt::entry_def().id,
                    read_receipt_entry,
                    ChainTopOrdering::Relaxed,
                ),
            )?;
            return Ok(extern_io.decode()?);
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
