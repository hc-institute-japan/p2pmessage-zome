use hdk::prelude::*;

use super::ReceiveReceiptInput;
use crate::utils::error;

pub fn commit_receipt_to_sender_chain_handler(
    receive_input: ReceiveReceiptInput,
) -> ExternResult<()> {
    let receive_call_result: ZomeCallResponse = call_remote(
        receive_input.receiver.clone(),
        zome_info()?.name,
        "receive_receipt".into(),
        None,
        &receive_input.receipt,
    )?;

    match receive_call_result {
        ZomeCallResponse::Ok(_) => Ok(()),
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
