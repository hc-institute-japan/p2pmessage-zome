use derive_more::{From, Into};
use hdk3::prelude::{timestamp::Timestamp, *};
pub mod handlers;

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Status {
    Sent,      // the message has been transmitted to the network
    Delivered, // the message has successfully traversed the network and reached the receiver
    Read,      // the message has been opened by the receiver
    Failed
}

#[hdk_entry(id = "message", visibility = "private")]
pub struct MessageEntry {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>,
    status: Status,
    reply_to: Option<EntryHash>
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey,
    payload: String
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageParameter {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>,
    status: Status,
    reply_to: Option<EntryHash>
}

impl MessageEntry {
    pub fn from_parameter(message_parameter: MessageParameter) -> Self {
        MessageEntry {
            author: message_parameter.author,
            receiver: message_parameter.receiver,
            payload: message_parameter.payload,
            time_sent: message_parameter.time_sent,
            time_received: message_parameter.time_received,
            status: message_parameter.status,
            reply_to: message_parameter.reply_to
        }
    }
}

impl MessageParameter {
    pub fn from_entry(message_entry: MessageEntry) -> Self {
        MessageParameter {
            author: message_entry.author,
            receiver: message_entry.receiver,
            payload: message_entry.payload,
            time_sent: message_entry.time_sent,
            time_received: message_entry.time_received,
            status: message_entry.status,
            reply_to: message_entry.reply_to
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessagesByAgent {
    author: AgentPubKey,
    messages: Vec<MessageParameter>
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageRange {
    author: AgentPubKey,
    last_message_timestamp_seconds: i64
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct BooleanWrapper(bool);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessageParameterOption(Option<MessageParameter>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessageListWrapper(Vec<MessageParameter>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct AgentListWrapper(Vec<AgentPubKey>);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct MessagesByAgentListWrapper(Vec<MessagesByAgent>);

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct Claims(Vec<CapClaim>);

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct Reply {
    replied_message: MessageParameter,
    reply: String
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingInfo {
    agent: AgentPubKey,
    is_typing: bool
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingSignal {
    kind: String,
    agent: AgentPubKey,
    is_typing: bool
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageSignal {
    kind: String,
    message: MessageParameter
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Signal {
    Message(MessageSignal),
    Typing(TypingSignal)
}