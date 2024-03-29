use hdk::prelude::*;

use p2pmessage_coordinator_types::*;
use p2pmessage_integrity_types::*;

use super::utils::this_zome_index;

pub fn receive_message_handler(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt {
        id: vec![hash_entry(&input.message)?],
        status: Status::Delivered {
            timestamp: sys_time()?,
        },
    };
    let receipt_entry = Entry::App(receipt.clone().try_into()?);
    let message_entry = Entry::App(input.message.clone().try_into()?);
    host_call::<CreateInput, ActionHash>(
        __hc__create_1,
        CreateInput::new(
            EntryDefLocation::app(this_zome_index()?, 0),
            EntryVisibility::Private,
            message_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;
    host_call::<CreateInput, ActionHash>(
        __hc__create_1,
        CreateInput::new(
            EntryDefLocation::app(this_zome_index()?, 1),
            EntryVisibility::Private,
            receipt_entry,
            ChainTopOrdering::Relaxed,
        ),
    )?;

    if let Some(file) = input.file.clone() {
        let file_entry = Entry::App(file.clone().try_into()?);
        host_call::<CreateInput, ActionHash>(
            __hc__create_1,
            CreateInput::new(
                EntryDefLocation::app(this_zome_index()?, 3),
                EntryVisibility::Private,
                file_entry,
                ChainTopOrdering::Relaxed,
            ),
        )?;
    };

    let mut message_return;
    message_return = P2PMessageData {
        author: input.message.author.clone(),
        receiver: input.message.receiver.clone(),
        payload: input.message.payload.clone(),
        time_sent: input.message.time_sent.clone(),
        reply_to: None,
    };
    if let Some(ref reply_to_hash) = input.message.reply_to {
        let queried_messages: Vec<Record> = query(
            QueryFilter::new()
                .entry_type(EntryType::App(AppEntryDef::new(
                    EntryDefIndex::from(0),
                    this_zome_index()?,
                    EntryVisibility::Private,
                )))
                .include_entries(true),
        )?;
        for queried_message in queried_messages.into_iter() {
            if let Ok(message_entry) = TryInto::<P2PMessage>::try_into(queried_message) {
                let message_hash = hash_entry(&message_entry)?;

                if *reply_to_hash == message_hash {
                    let replied_to_message = P2PMessageReplyTo {
                        hash: message_hash,
                        author: message_entry.author,
                        receiver: message_entry.receiver,
                        payload: message_entry.payload,
                        time_sent: message_entry.time_sent,
                        reply_to: None,
                    };

                    message_return = P2PMessageData {
                        author: input.message.author.clone(),
                        receiver: input.message.receiver.clone(),
                        payload: input.message.payload.clone(),
                        time_sent: input.message.time_sent.clone(),
                        reply_to: Some(replied_to_message),
                    };
                }
            }
        }
    }

    let signal = Signal::Message(MessageSignal {
        message: MessageDataAndReceipt(
            (hash_entry(&input.message.clone())?, message_return),
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
