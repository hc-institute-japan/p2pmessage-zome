use derive_more::{From, Into};
use hdk::prelude::{timestamp::Timestamp, *};
use std::collections::HashMap;

pub mod commit_message_to_receiver_chain;
pub mod commit_receipt_to_sender_chain;
pub mod get_adjacent_messages;
pub mod get_file_bytes;
pub mod get_latest_messages;
pub mod get_messages_by_agent_by_timestamp;
pub mod get_next_messages;
pub mod get_pinned_messages;
pub mod get_previous_messages;
pub mod helpers;
pub mod init;
pub mod pin_message;
pub mod read_message;
pub mod receive_message;
pub mod receive_receipt;
pub mod send_message;
pub mod send_message_with_timestamp;
pub mod sync_pins;
pub mod typing;

use file_types::{FileMetadata, FileType, Payload, PayloadInput};



// ENTRY STRUCTURES
#[derive(Clone, From)]
#[hdk_entry_helper]
pub struct P2PMessage {
    pub author: AgentPubKey,
    pub receiver: AgentPubKey,
    payload: Payload,
    time_sent: Timestamp,
    reply_to: Option<EntryHash>,
}

impl P2PMessage {
    pub fn from_input(input: MessageWithTimestampInput) -> ExternResult<Self> {
        let message = P2PMessage {
            author: agent_info()?.agent_latest_pubkey,
            receiver: input.receiver,
            payload: match input.payload {
                PayloadInput::Text { ref payload } => Payload::Text {
                    payload: payload.to_owned(),
                },
                PayloadInput::File {
                    ref metadata,
                    ref file_type,
                    ref file_bytes,
                } => {
                    let p2pfile = P2PFileBytes(file_bytes.clone());
                    let file_hash = hash_entry(&p2pfile)?;
                    Payload::File {
                        metadata: FileMetadata {
                            file_name: metadata.file_name.clone(),
                            file_size: metadata.file_size.clone(),
                            file_type: metadata.file_type.clone(),
                            file_hash: file_hash,
                        },
                        file_type: file_type.clone(),
                    }
                }
            },
            time_sent: input.timestamp,
            reply_to: input.reply_to,
        };
        Ok(message)
    }
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct P2PMessageReceipt {
    pub id: Vec<EntryHash>,
    pub status: Status,
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct P2PMessagePin {
    id: Vec<EntryHash>,
    conversants: Vec<AgentPubKey>,
    status: PinStatus,
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct P2PFileBytes(SerializedBytes);

// #[derive(EntryDefRegistration)]
// enum EntryTypes {
//     #[entry_def(name="p2pmessage", required_validations=5, visibility="private")]
//     P2PMessage(P2PMessage),
//     #[entry_def(name="p2pmessagereceipt", required_validations=5, visibility="private")]
//     P2PMessageReceipt(P2PMessageReceipt),
//     #[entry_def(name="p2pmessagepin", required_validations=5, visibility="private")]
//     P2PMessagePin(P2PMessagePin),
//     #[entry_def(name="p2pfilebytes", required_validations=5, visibility="private")]
//     P2PFileBytes(P2PFileBytes),
// }

// // ENTRY DEFINITIONS
// entry_def!(P2PMessage
//     EntryDef {
//         id: "p2pmessage".into(),
//         visibility: EntryVisibility::Private,
//         crdt_type: CrdtType,
//         required_validations: RequiredValidations::default(),
//         required_validation_type: RequiredValidationType::Element
//     }
// );

// entry_def!(P2PFileBytes
//     EntryDef {
//         id: "p2pfilebytes".into(),
//         visibility: EntryVisibility::Private,
//         crdt_type: CrdtType,
//         required_validations: RequiredValidations::default(),
//         required_validation_type: RequiredValidationType::Element
//     }
// );

// entry_def!(P2PMessageReceipt EntryDef {
//     id: "p2pmessagereceipt".into(),
//     visibility: EntryVisibility::Private,
//     crdt_type: CrdtType,
//     required_validations: RequiredValidations::default(),
//     required_validation_type: RequiredValidationType::Element
// });

// entry_def!(P2PMessagePin EntryDef {
//     id: "p2pmessagepin".into(),
//     visibility: EntryVisibility::Private,
//     crdt_type: CrdtType,
//     required_validations: RequiredValidations::default(),
//     required_validation_type: RequiredValidationType::Element
// });

// INPUT STRUCTURES FROM THE FRONTEND
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey,
    payload: PayloadInput,
    reply_to: Option<EntryHash>,
}

// test_stub: test structure for accepting timestamp as input in send_message
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageWithTimestampInput {
    receiver: AgentPubKey,
    payload: PayloadInput,
    timestamp: Timestamp,
    reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReadMessageInput {
    message_hashes: Vec<EntryHash>,
    sender: AgentPubKey,
    timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PinMessageInput {
    message_hashes: Vec<EntryHash>,
    conversants: Vec<AgentPubKey>,
    status: String,
    timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct CommitMessageInput {
    pub message_hash: EntryHash,
    pub message: Option<P2PMessage>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct CommitReceiptInput {
    receipt_hash: Option<EntryHash>,
    message: Option<P2PMessageReceipt>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiveMessageInput {
    message: P2PMessage,
    file: Option<P2PFileBytes>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiveReceiptInput {
    pub receipt: P2PMessageReceipt,
    pub receiver: AgentPubKey,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum Status {
    Sent { timestamp: Timestamp },
    Delivered { timestamp: Timestamp },
    Read { timestamp: Timestamp },
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "pinstatus", rename_all = "camelCase")]
pub enum PinStatus {
    Pinned { timestamp: Timestamp },
    Unpinned { timestamp: Timestamp },
}

// GET FILTERS
#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterAgentTimestamp {
    conversant: AgentPubKey,
    date: Timestamp,
    payload_type: String,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterBatch {
    conversant: AgentPubKey,
    batch_size: u8,
    payload_type: String,
    last_fetched_timestamp: Option<Timestamp>, // header timestamp; oldest message in the last fetched message
    last_fetched_message_id: Option<EntryHash>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageDataAndReceipt((EntryHash, P2PMessageData), (EntryHash, P2PMessageReceipt));

// OUTPUT STRUCTURES
#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PMessageReplyTo {
    hash: EntryHash,
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: Payload,
    time_sent: Timestamp,
    reply_to: Option<EntryHash>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PMessageData {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: Payload,
    time_sent: Timestamp,
    reply_to: Option<P2PMessageReplyTo>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageHashTables(
    HashMap<String, Vec<String>>,                   // AgentMessages
    HashMap<String, (P2PMessageData, Vec<String>)>, // MessageContents
    HashMap<String, P2PMessageReceipt>,             // ReceiptContents
);

// SIGNAL STRUCTURES
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    Message(MessageSignal),
    P2PTypingDetailSignal(TypingSignal),
    P2PMessageReceipt(ReceiptSignal),
    P2PPinSignal(PinSignal),
    ErrorMessage(ErrorMessage),
    ErrorReceipt(ErrorReceipt),
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct SignalDetails {
    pub name: String,
    pub payload: Signal,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageSignal {
    message: MessageDataAndReceipt,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiptSignal {
    receipt: HashMap<String, P2PMessageReceipt>,
}
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PinSignal {
    pin: HashMap<String, P2PMessagePin>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingSignal {
    agent: AgentPubKey,
    is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ErrorMessage {
    pub message: P2PMessage,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ErrorReceipt {
    pub receipt: P2PMessageReceipt,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PTypingDetailIO {
    agent: AgentPubKey,
    is_typing: bool,
}
