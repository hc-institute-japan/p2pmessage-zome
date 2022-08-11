use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_coordinator_types::*;
use p2pmessage_integrity_types::*;

pub fn sync_pins_handler(pin: P2PMessagePin) -> ExternResult<HashMap<String, P2PMessagePin>> {
    let pin_entry = Entry::App(pin.clone().try_into()?);
    let pin_hash = host_call::<CreateInput, ActionHash>(
        __create,
        CreateInput::new(
            EntryDefLocation::app(2),
            EntryVisibility::Private,
            pin_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let mut pin_contents: HashMap<String, P2PMessagePin> = HashMap::new();
    pin_contents.insert(pin_hash.to_string(), pin);

    let signal = Signal::P2PPinSignal(PinSignal {
        pin: pin_contents.clone(),
    });

    let signal_details = SignalDetails {
        name: "SYNC_P2P_PINS".to_string(),
        payload: signal,
    };

    emit_signal(&signal_details)?;

    Ok(pin_contents)
}
