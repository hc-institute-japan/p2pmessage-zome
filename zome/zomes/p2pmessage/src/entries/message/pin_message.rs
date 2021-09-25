use hdk::prelude::holo_hash::HeaderHashB64;
use hdk::prelude::*;
use std::collections::HashMap;

use super::{P2PMessagePin, PinContents, PinMessageInput, PinStatus};

pub fn pin_message_handler(pin_message_input: PinMessageInput) -> ExternResult<PinContents> {
    let mut pinned_messages: HashMap<String, P2PMessagePin> = HashMap::new();

    let pin = P2PMessagePin {
        id: pin_message_input.message_hashes,
        conversants: pin_message_input.conversants.clone(),
        status: if pin_message_input.status == "Pinned" {
            PinStatus::Pinned {
                timestamp: pin_message_input.timestamp,
            }
        } else {
            PinStatus::Unpinned {
                timestamp: pin_message_input.timestamp,
            }
        },
    };

    // let pin_hash = create_entry(&pin)?;
    let pin_entry = Entry::App(pin.clone().try_into()?);
    let pin_hash = host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            P2PMessagePin::entry_def().id,
            pin_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    pinned_messages.insert(
        HeaderHashB64::from(pin_hash.clone()).to_string(), //b64 check
        pin.clone(),
    );
    let conversant: AgentPubKey;
    if pin_message_input.conversants[0] != agent_info()?.agent_latest_pubkey {
        conversant = pin_message_input.conversants[0].clone()
    } else {
        conversant = pin_message_input.conversants[1].clone()
    }

    let zome_call_response: ZomeCallResponse = call_remote(
        conversant,
        zome_info()?.zome_name,
        FunctionName("sync_pins".into()),
        None,
        &pin,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            return Ok(extern_io.decode()?);
        }
        _ => return crate::error("we have an error trying to get the receive_read_receipt"),
    }
}
