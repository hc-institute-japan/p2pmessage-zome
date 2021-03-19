use hdk::prelude::*;

use super::{
    ReadReceiptInput,
    ReceiptContents,

};

pub fn read_message_handler(read_receipt_input: ReadReceiptInput) -> ExternResult<ReceiptContents> {

    create_entry(&read_receipt_input.receipt)?;

    let zome_call_response:ZomeCallResponse = call_remote(
        read_receipt_input.sender,
        zome_info()?.zome_name,
        FunctionName("receive_read_receipt".into()),
        None,
        &read_receipt_input.receipt,
    )?;

    match zome_call_response {

        ZomeCallResponse::Ok(extern_io) =>{
            return Ok(extern_io.decode()?);
        },
        _=> return crate::error("we have an error trying to get the receive_read_receipt"), 
    }
}