use derive_more::{From, Into};
use hdk::prelude::{timestamp::Timestamp, *};
use std::collections::HashMap;

//pub mod handlers; MANUEL:this file contains a method never used maybe we want to erase the file

//this are the files for each method definition
pub mod get_file_bytes;
pub mod get_latest_messages;
pub mod get_messages_by_agent_by_timestamp;
pub mod get_next_batch_messages;
pub mod helpers;
pub mod init;
pub mod read_message;
pub mod receive_message;
pub mod receive_read_receipt;
pub mod send_message;
pub mod typing;

use file_types::{Payload, PayloadInput};

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PMessage {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: Payload,
    time_sent: Timestamp,
    reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct P2PMessageReceipt {
    id: Vec<EntryHash>,
    status: Status,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct P2PFileBytes(SerializedBytes);

entry_def!(P2PMessage
    EntryDef {
        id: "p2pmessage".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

entry_def!(P2PFileBytes
    EntryDef {
        id: "p2pfilebytes".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

entry_def!(P2PMessageReceipt EntryDef {
    id: "p2pmessagereceipt".into(),
    visibility: EntryVisibility::Private,
    crdt_type: CrdtType,
    required_validations: RequiredValidations::default(),
    required_validation_type: RequiredValidationType::Element
});

impl P2PMessageReceipt {
    pub fn from_message(message: P2PMessage) -> ExternResult<Self> {
        let now = sys_time()?;
        let receipt = P2PMessageReceipt {
            id: vec![hash_entry(&message)?],
            status: Status::Delivered {
                timestamp: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
            },
        };
        Ok(receipt)
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum Status {
    Sent,
    Delivered { timestamp: Timestamp },
    Read { timestamp: Timestamp },
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey,
    payload: PayloadInput,
    reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiveMessageInput(P2PMessage, Option<P2PFileBytes>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct BooleanWrapper(bool);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct ReceiptHash(EntryHash);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageHash(EntryHash);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageBundle(P2PMessage, Vec<String>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageAndReceipt((EntryHash, P2PMessage), (EntryHash, P2PMessageReceipt));

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct AgentMessages(HashMap<String, Vec<String>>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageContents(HashMap<String, MessageBundle>);

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiptContents(HashMap<String, P2PMessageReceipt>);

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct FileContents(HashMap<String, P2PFileBytes>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageHashTables(AgentMessages, MessageContents, ReceiptContents);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterAgentTimestamp {
    conversant: AgentPubKey,
    date: Timestamp,
    payload_type: String,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct BatchSize(u8);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterBatch {
    conversant: AgentPubKey,
    batch_size: u8,
    payload_type: String,
    last_fetched_timestamp: Option<Timestamp>, // header timestamp; oldest message in the last fetched message
    last_fetched_message_id: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    Message(MessageSignal),
    P2PTypingDetailSignal(TypingSignal),
    P2PMessageReceipt(ReceiptSignal),
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct SignalDetails {
    pub name: String,
    pub payload: Signal,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageSignal {
    message: MessageAndReceipt,
}
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiptSignal {
    receipt: ReceiptContents,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingSignal {
    agent: AgentPubKey,
    is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PTypingDetailIO {
    agent: AgentPubKey,
    is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReadReceiptInput {
    receipt: P2PMessageReceipt,
    sender: AgentPubKey,
}
// to replace ReadReceiptInput
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReadMessageInput {
    message_hashes: Vec<EntryHash>,
    sender: AgentPubKey,
    timestamp: Timestamp,
}

// impl P2PMessage {
//     pub fn from_input(input: MessageInput, hash: Option<EntryHash>) -> ExternResult<Self> {
//         let now = sys_time()?;

//         let message = P2PMessage {
//             author: agent_info()?.agent_latest_pubkey,
//             receiver: input.receiver,
//             payload: match input.payload {
//                 PayloadInput::Text { payload } => Payload::Text { payload },
//                 PayloadInput::File {
//                     metadata,
//                     file_type,
//                     ..
//                 } => Payload::File {
//                     metadata: FileMetadata {
//                         file_name: metadata.file_name,
//                         file_size: metadata.file_size,
//                         file_type: metadata.file_type,
//                         file_hash: hash,
//                     },
//                     file_type: file_type,
//                 },
//             },
//             time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
//             reply_to: input.reply_to,
//         };
//         Ok(message)
//     }
// }

// impl P2PFileBytes {
//     pub fn from_input(input: MessageInput) -> ExternResult<Self> {
//         match input.payload {
//             PayloadInput::Text { .. } => crate::err("TODO: 000", "no file bytes in input"),
//             PayloadInput::File { file_bytes, .. } => Ok(P2PFileBytes(file_bytes)),
//         }
//     }
// }
// #[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
// pub struct FileMetadata {
//     file_name: String,
//     file_size: u8,
//     file_type: FileType,
//     file_hash: String,
// }

// #[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
// #[serde(tag = "type", rename_all = "camelCase")]
// pub enum FileType {
//     Image { thumbnail: SerializedBytes },
//     Video { thumbnail: SerializedBytes },
//     Others,
// }

// #[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
// #[serde(tag = "type")]
// pub enum Payload {
//     Text {
//         payload: String,
//     },
//     File {
//         metadata: FileMetadata,
//         file_type: FileType,
//     },
// }

// #[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
// #[serde(tag = "type", rename_all = "camelCase")]
// pub enum PayloadInput {
//     Text {
//         payload: String,
//     },
//     File {
//         file_name: String,
//         file_size: u8,
//         file_type: FileType,
//         file_hash: String,
//         bytes: SerializedBytes,
//     },
// }
