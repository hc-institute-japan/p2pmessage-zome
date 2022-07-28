use hdk::prelude::*;
use std::collections::HashMap;

use p2pmessage_integrity_types::*;
use p2pmessage_coordinator_types::*;

use crate::utils::error;

pub fn pin_message_handler(
    pin_message_input: PinMessageInput,
) -> ExternResult<HashMap<String, P2PMessagePin>> {
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

    let conversant: AgentPubKey;
    if pin_message_input.conversants[0] != agent_info()?.agent_latest_pubkey {
        conversant = pin_message_input.conversants[0].clone()
    } else {
        conversant = pin_message_input.conversants[1].clone()
    }

    let zome_call_response: ZomeCallResponse = call_remote(
        conversant,
        zome_info()?.name,
        FunctionName("sync_pins".into()),
        None,
        &pin,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            let pin_entry = Entry::App(pin.clone().try_into()?);
            
            let pin_hash = host_call::<CreateInput, ActionHash>(
                __create,
                CreateInput::new(
                    EntryDefLocation::app(0, 0),
                    EntryVisibility::Private,
                    pin_entry,
                    ChainTopOrdering::Relaxed,
                ),
            )?;

            pinned_messages.insert(pin_hash.to_string(), pin);
            
            let result = extern_io.decode();
            match result {
                Ok(map) => return Ok(map),
                Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e))))
            }
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
