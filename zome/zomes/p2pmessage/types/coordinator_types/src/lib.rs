use derive_more::{From, Into};
use hdk::prelude::{timestamp::Timestamp, *};
use std::collections::HashMap;

use p2pmessage_integrity_types::*;

/* 
 * INPUT STRUCTURES FOR THE FRONTEND
*/

// MESSAGE ZOME INPUT FROM THE FRONTEND
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    pub receiver: AgentPubKey,
    pub payload: PayloadInput,
    pub reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageWithTimestampInput {
    pub receiver: AgentPubKey,
    pub payload: PayloadInput,
    pub timestamp: Timestamp,
    pub reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReadMessageInput {
    pub message_hashes: Vec<EntryHash>,
    pub sender: AgentPubKey,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PinMessageInput {
    pub message_hashes: Vec<EntryHash>,
    pub conversants: Vec<AgentPubKey>,
    pub status: String,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct CommitMessageInput {
    pub message_hash: EntryHash,
    pub message: Option<P2PMessage>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct CommitReceiptInput {
    pub receipt_hash: Option<EntryHash>,
    pub message: Option<P2PMessageReceipt>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiveMessageInput {
    pub message: P2PMessage,
    pub file: Option<P2PFileBytes>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiveReceiptInput {
    pub receipt: P2PMessageReceipt,
    pub receiver: AgentPubKey,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PTypingDetailIO {
    pub agent: AgentPubKey,
    pub is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadataInput {
    pub file_name: String,
    pub file_size: usize,
    pub file_type: String,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum PayloadInput {
    #[serde(rename_all = "camelCase")]
    Text { payload: String },
    #[serde(rename_all = "camelCase")]
    File {
        metadata: FileMetadataInput,
        file_type: FileType,
        file_bytes: SerializedBytes,
    },
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum PayloadType {
    Text,
    File,
    Media,
    All,
}

// GET FILTERS
#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterAgentTimestamp {
    pub conversant: AgentPubKey,
    pub date: Timestamp,
    pub payload_type: String,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageFilterBatch {
    pub conversant: AgentPubKey,
    pub batch_size: u8,
    pub payload_type: String,
    pub last_fetched_timestamp: Option<Timestamp>, // header timestamp; oldest message in the last fetched message
    pub last_fetched_message_id: Option<EntryHash>,
}

// OUTPUT STRUCTURES
#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PMessageReplyTo {
    pub hash: EntryHash,
    pub author: AgentPubKey,
    pub receiver: AgentPubKey,
    pub payload: Payload,
    pub time_sent: Timestamp,
    pub reply_to: Option<EntryHash>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct P2PMessageData {
    pub author: AgentPubKey,
    pub receiver: AgentPubKey,
    pub payload: Payload,
    pub time_sent: Timestamp,
    pub reply_to: Option<P2PMessageReplyTo>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct P2PMessageHashTables(
    pub HashMap<String, Vec<String>>,                   // AgentMessages
    pub HashMap<String, (P2PMessageData, Vec<String>)>, // MessageContents
    pub HashMap<String, P2PMessageReceipt>,             // ReceiptContents
);

// SIGNAL STRUCTURES
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    Message(MessageSignal),
    P2PMessageReceipt(ReceiptSignal),
    P2PPinSignal(PinSignal),
    P2PTypingDetailSignal(TypingSignal),
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
    pub message: MessageDataAndReceipt,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ReceiptSignal {
    pub receipt: HashMap<String, P2PMessageReceipt>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PinSignal {
    pub pin: HashMap<String, P2PMessagePin>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingSignal {
    pub agent: AgentPubKey,
    pub is_typing: bool,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct MessageDataAndReceipt(pub (EntryHash, P2PMessageData), pub (EntryHash, P2PMessageReceipt));

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ErrorMessage {
    pub message: P2PMessage,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct ErrorReceipt {
    pub receipt: P2PMessageReceipt,
}
