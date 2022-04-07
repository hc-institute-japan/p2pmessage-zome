use hdk::prelude::*;

use crate::helpers::get_system_message_from_chain;

pub fn commit_system_message_to_receiver_chain_handler(
    message_hash: EntryHash,
) -> ExternResult<()> {
    let message = get_system_message_from_chain(message_hash)?;

    if agent_info()?.agent_latest_pubkey == message.author.clone() {
        let receive_call_result: ZomeCallResponse = call_remote(
            message.receiver.clone(),
            zome_info()?.name,
            "receive_message".into(),
            None,
            &message,
        )?;

        match receive_call_result {
            ZomeCallResponse::Ok(extern_io) => {
                return Ok(());
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
}
