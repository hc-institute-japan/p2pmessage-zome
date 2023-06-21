use derive_more::From;
use hdi::prelude::{timestamp::Timestamp, *};

/*
 * ENTRY STRUCTURES
 * AND RELATED STRUCTS
 */

#[derive(Clone, From)]
#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct P2PMessage {
    pub author: AgentPubKey,
    pub receiver: AgentPubKey,
    pub payload: Payload,
    pub time_sent: Timestamp,
    pub reply_to: Option<EntryHash>,
}

#[derive(Clone)]
#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct P2PMessageReceipt {
    pub id: Vec<EntryHash>,
    pub status: Status,
}

#[derive(Clone)]
#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct P2PMessagePin {
    pub id: Vec<EntryHash>,
    pub conversants: Vec<AgentPubKey>,
    pub status: PinStatus,
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct P2PFileBytes(pub SerializedBytes);

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

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub file_name: String,
    pub file_size: usize,
    pub file_type: String,
    pub file_hash: EntryHash,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum FileType {
    Image { thumbnail: SerializedBytes },
    Video { thumbnail: SerializedBytes },
    Other,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum Payload {
    #[serde(rename_all = "camelCase")]
    Text { payload: String },
    #[serde(rename_all = "camelCase")]
    File {
        metadata: FileMetadata,
        file_type: FileType,
    },
}
