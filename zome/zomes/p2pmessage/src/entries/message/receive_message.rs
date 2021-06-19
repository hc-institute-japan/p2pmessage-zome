use hdk::prelude::*;

use super::{
    MessageAndReceipt, MessageSignal, P2PMessageReceipt, ReceiveMessageInput, Signal, SignalDetails,
};

pub fn receive_message_handler(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(input.0.clone())?;
    create_entry(&input.0)?;
    create_entry(&receipt)?;
    if let Some(file) = input.1 {
        create_entry(&file)?;
    };

    let signal = Signal::Message(MessageSignal {
        message: MessageAndReceipt(
            (hash_entry(&input.0.clone())?, input.0.clone()),
            (hash_entry(&receipt.clone())?, receipt.clone()),
        ),
    });

    let signal_details = SignalDetails {
        name: "RECEIVE_P2P_MESSAGE".to_string(),
        payload: signal,
    };
    emit_signal(&signal_details)?;

    Ok(receipt)
}
