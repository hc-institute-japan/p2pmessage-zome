use hdk::prelude::*;

use p2pmessage_coordinator_types::*;

pub fn typing_handler(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    let signal = Signal::P2PTypingDetailSignal(TypingSignal {
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing,
    });

    let signal_details = SignalDetails {
        name: "TYPING_P2P".to_string(),
        payload: signal,
    };

    let mut agents: Vec<AgentPubKey> = Vec::new();

    agents.push(typing_info.agent);

    let signal_details_result: Result<ExternIO, SerializedBytesError> = ExternIO::encode(signal_details);
    match signal_details_result {
        Ok(signal_details) => {
            remote_signal(signal_details, agents)?;
            return Ok(())
        }
        Err(e) => return Err(wasm_error!(WasmErrorInner::Guest(String::from(e))))
    }
}
