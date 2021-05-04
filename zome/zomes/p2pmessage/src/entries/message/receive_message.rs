use hdk::prelude::*;

use super::{MessageAndReceipt, MessageSignal, P2PMessageReceipt, ReceiveMessageInput, Signal};

pub fn receive_message_handler(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(input.0.clone())?;
    create_entry(&input.0)?;
    create_entry(&receipt)?;
    if let Some(file) = input.1 {
        let file_hash = create_entry(&file)?;
        debug!(
            "{}",
            format!(
                "The file hash for the newly committed file is {}",
                file_hash
            )
        );
    };

    let signal = Signal::Message(MessageSignal {
        name: "RECEIVE_P2P_MESSAGE".to_string(),
        // message: input.0.clone(),
        message: MessageAndReceipt(
            (hash_entry(&input.0.clone())?, input.0.clone()),
            (hash_entry(&receipt.clone())?, receipt.clone()),
        ),
    });
    // let agents = vec![input.0.receiver.clone()];
    // remote_signal(&signal_payload, agents)?;
    emit_signal(&signal)?;

    Ok(receipt)
}
