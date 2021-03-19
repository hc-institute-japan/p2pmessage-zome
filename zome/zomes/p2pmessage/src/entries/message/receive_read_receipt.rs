use hdk::prelude::*;

use super::helpers::commit_receipts;

use super::{

    P2PMessageReceipt,
    ReceiptContents,
    Signal,

};

pub fn receive_read_receipt_handler(receipt: P2PMessageReceipt) -> ExternResult<ReceiptContents> {
    let receipts = commit_receipts(vec![receipt])?;
    emit_signal(Signal::P2PMessageReceipt(receipts.clone()))?;
    Ok(receipts)
}