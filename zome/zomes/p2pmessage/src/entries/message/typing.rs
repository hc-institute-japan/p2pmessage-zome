use hdk::prelude::*;

use super::{P2PTypingDetailIO, Signal, TypingSignal};

pub fn typing_handler(typing_info: P2PTypingDetailIO) -> ExternResult<()> {
    let payload = Signal::P2PTypingDetailSignal(TypingSignal {
        name: "TYPING_P2P".to_string(),
        agent: agent_info()?.agent_latest_pubkey,
        is_typing: typing_info.is_typing,
    });

    let mut agents = Vec::new();

    agents.push(typing_info.agent);
    agents.push(agent_info()?.agent_latest_pubkey);

    // debug!(
    //     "{}",
    //     format!("typing handler reaches here for {:?}", agents.clone())
    // );
    remote_signal(ExternIO::encode(payload)?, agents)?;
    // debug!("sent remote");
    Ok(())
}
