use super::SystemMessage;
use hdk::prelude::*;

pub fn announce_handler(input: NotificationInput) -> ExternResult<()> {
    let notification = SystemMessage {
        author: agent_info()?.agent_latest_pubkey,
        receiver: input.receiver,
        message: input.message,
    };

    let notification_entry = Entry::App(notification.clone().try_into()?);
    host_call::<CreateInput, HeaderHash>(
        __create,
        CreateInput::new(
            SystemMessage::entry_def().id,
            notification_entry.clone(),
            ChainTopOrdering::Relaxed,
        ),
    )?;

    // post commit?
}
