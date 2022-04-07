use hdk::prelude::*;

use super::{Signal, SystemMessage, SystemMessageSignal};

pub fn receive_system_message_handler(input: SystemMessage) -> ExternResult<()> {
    let message_entry = Entry::App(input.clone().try_into()?);
    host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            SystemMessage::entry_def().id,
            message_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    let signal = Signal::SystemMessage(SystemMessageSignal { message: input });

    let signal_details = SignalDetails {
        name: "RECEIVE_SYSTEM_MESSAGE".to_string(),
        payload: signal,
    };
    emit_signal(&signal_details)?;

    Ok(())
}
