use hdk3::prelude::*;
use derive_more::{From, Into};
use crate::{timestamp::Timestamp};
pub mod handlers;

#[hdk_entry(id = "message", visibility = "public")]
pub struct MessageEntry {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey, 
    payload: String
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageOutput {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>
}

impl MessageEntry {
    pub fn from_output(message_output: MessageOutput) -> Self {
        MessageEntry {
            author: message_output.author,
            receiver: message_output.receiver,
            payload: message_output.payload,
            time_sent: message_output.time_sent,
            time_received: message_output.time_received
        }
    }
}

impl MessageOutput {
    pub fn from_entry(message_entry: MessageEntry) -> Self {
        MessageOutput {
            author: message_entry.author,
            receiver: message_entry.receiver,
            payload: message_entry.payload,
            time_sent: message_entry.time_sent,
            time_received: message_entry.time_received
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessagesByAgent {
    author: AgentPubKey,
    messages: Vec<MessageOutput>
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageRange {
    author: AgentPubKey,
    last_message_timestamp_seconds: i64
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct BooleanWrapper(bool);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessageOutputOption(Option<MessageOutput>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessageListWrapper(Vec<MessageOutput>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct AgentListWrapper(Vec<AgentPubKey>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessagesByAgentListWrapper(Vec<MessagesByAgent>);