use hdk::prelude::*;

use super::{P2PMessageReceipt, ReadMessageInput, ReceiptContents, Status};

pub fn read_message_handler(read_message_input: ReadMessageInput) -> ExternResult<ReceiptContents> {
    let receipt = P2PMessageReceipt {
        id: read_message_input.message_hashes,
        status: Status::Read {
            timestamp: read_message_input.timestamp,
        },
    };

    // create_entry(&receipt)?;
    let read_receipt_entry = Entry::App(receipt.clone().try_into()?);
    host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            P2PMessageReceipt::entry_def().id,
            read_receipt_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let zome_call_response: ZomeCallResponse = call_remote(
        read_message_input.sender,
        zome_info()?.zome_name,
        FunctionName("receive_read_receipt".into()),
        None,
        &receipt,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            return Ok(extern_io.decode()?);
        }
        _ => return crate::error("we have an error trying to get the receive_read_receipt"),
    }
}
