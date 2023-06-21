use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_coordinator_types::*;
use p2pmessage_integrity_types::*;

use super::utils::this_zome_index;

pub fn receive_receipt_handler(
    receipt: P2PMessageReceipt,
) -> ExternResult<HashMap<String, P2PMessageReceipt>> {
    let receipt_entry = Entry::App(receipt.clone().try_into()?);

    let receipt_hash = host_call::<CreateInput, ActionHash>(
        __hc__create_1,
        CreateInput::new(
            EntryDefLocation::app(this_zome_index()?, 1),
            EntryVisibility::Private,
            receipt_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let mut receipt_contents: HashMap<String, P2PMessageReceipt> = HashMap::new();
    receipt_contents.insert(receipt_hash.to_string(), receipt.clone());

    let signal = Signal::P2PMessageReceipt(ReceiptSignal {
        receipt: receipt_contents.clone(),
    });

    let signal_details = SignalDetails {
        name: "RECEIVE_P2P_RECEIPT".to_string(),
        payload: signal,
    };

    emit_signal(&signal_details)?;

    Ok(receipt_contents)
}
