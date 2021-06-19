use hdk::prelude::*;

use super::{P2PTypingDetailIO, Signal, SignalDetails, TypingSignal};

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

    remote_signal(ExternIO::encode(signal_details.clone())?, agents)?;
    Ok(())
}
