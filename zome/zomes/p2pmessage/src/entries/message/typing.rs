use hdk::prelude::*;

use super::{
    P2PTypingDetailIO,
    Signal,
};

pub fn typing_handler(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    let payload = Signal::P2PTypingDetailSignal(P2PTypingDetailIO {
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing,
    });

    let mut agents = Vec::new();

    agents.push(typing_info.agent);
    agents.push(agent_info()?.agent_latest_pubkey);

    remote_signal(&payload, agents)?;
    Ok(())
}