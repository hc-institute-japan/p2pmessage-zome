use hdk::prelude::*;
use std::collections::HashMap;

// use super::helpers::commit_receipts;

use super::{P2PMessageReceipt, ReceiptContents, ReceiptSignal, Signal, SignalDetails};

pub fn receive_read_receipt_handler(receipt: P2PMessageReceipt) -> ExternResult<ReceiptContents> {
    // let receipts = commit_receipts(vec![receipt.clone()])?; //input is only a single receipt with a vector of messages hashes
    // let receipt_hash = create_entry(&receipt)?;

    let receipt_entry = Entry::App(receipt.clone().try_into()?);
    let receipt_hash = host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            P2PMessageReceipt::entry_def().id,
            receipt_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    receipt_contents.insert(
        receipt_hash.clone().to_string(), //b64 check
        receipt.clone(),
    );

    let signal = Signal::P2PMessageReceipt(ReceiptSignal {
        receipt: ReceiptContents(receipt_contents.clone()),
    });

    let signal_details = SignalDetails {
        name: "RECEIVE_P2P_RECEIPT".to_string(),
        payload: signal,
    };

    emit_signal(&signal_details)?;

    Ok(ReceiptContents(receipt_contents))
}
