use hdk::prelude::*;
use std::collections::HashMap;

// use super::helpers::commit_receipts;

use super::{P2PMessagePin, PinContents, PinSignal, Signal, SignalDetails};

pub fn sync_pins_handler(pin: P2PMessagePin) -> ExternResult<PinContents> {
    let pin_entry = Entry::App(pin.clone().try_into()?);
    let pin_hash = host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            P2PMessagePin::entry_def().id,
            pin_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let mut pin_contents: HashMap<String, P2PMessagePin> = HashMap::new();
    pin_contents.insert(
        pin_hash.clone().to_string(), //b64 check
        pin.clone(),
    );

    let signal = Signal::P2PPinSignal(PinSignal {
        pin: PinContents(pin_contents.clone()),
    });

    let signal_details = SignalDetails {
        name: "SYNC_P2P_PINS".to_string(),
        payload: signal,
    };

    emit_signal(&signal_details)?;

    Ok(PinContents(pin_contents))
}
