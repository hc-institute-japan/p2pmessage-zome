use hdk3::prelude::*;
use derive_more::{From, Into};
use crate::{timestamp::Timestamp};
pub mod handlers;

#[hdk_entry(id = "message", visibility = "public")]
pub struct MessageEntry {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    timestamp: Timestamp
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey, 
    payload: String
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageOutput {
    header: HeaderHash,
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    timestamp: Timestamp,
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct RemoteCallArgument {
    author: AgentPubKey,
    input: MessageInput
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


