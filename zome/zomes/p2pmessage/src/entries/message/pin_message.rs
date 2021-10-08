use crate::utils::error;
use hdk::prelude::*;
use std::collections::HashMap;

use super::{P2PMessagePin, PinMessageInput, PinStatus};

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
        zome_info()?.zome_name,
        FunctionName("sync_pins".into()),
        None,
        &pin,
    )?;

    match zome_call_response {
        ZomeCallResponse::Ok(extern_io) => {
            let pin_entry = Entry::App(pin.clone().try_into()?);
            let pin_hash = host_call::<CreateInput, HeaderHash>(
                __create,
                CreateInput::new(
                    P2PMessagePin::entry_def().id,
                    pin_entry,
                    ChainTopOrdering::Relaxed,
                ),
            )?;
            pinned_messages.insert(pin_hash.to_string(), pin);
            return Ok(extern_io.decode()?);
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
