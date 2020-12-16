use derive_more::{From, Into};
use hdk3::prelude::{timestamp::Timestamp, *};
pub mod handlers;

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Status {
    Sent,      // the message has been transmitted to the network
    Delivered, // the message has successfully traversed the network and reached the receiver
    Read,      // the message has been opened by the receiver
    Failed,
}

#[hdk_entry(id = "p2pmessage", visibility = "public")]
pub struct P2PMessage {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>,
    status: Status,
    reply_to: Option<EntryHash>,
}

impl P2PMessage {
    pub fn from_parameter(message_output: MessageParameter) -> Self {
        P2PMessage {
            author: message_output.author,
            receiver: message_output.receiver,
            payload: message_output.payload,
            time_sent: message_output.time_sent,
            time_received: message_output.time_received,
            status: message_output.status,
            reply_to: message_output.reply_to,
        }
    }
}

#[hdk_entry(id = "p2pmessageasync", visibility = "public")]
pub struct P2PMessageAsync {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>,
    reply_to: Option<EntryHash>,
    status: Status
}

impl P2PMessageAsync {
    pub fn from_parameter(message_parameter: MessageParameter, status: Status) -> Self {
        P2PMessageAsync {
            author: message_parameter.author,
            receiver: message_parameter.receiver,
            payload: message_parameter.payload,
            time_sent: message_parameter.time_sent,
            time_received: message_parameter.time_received,
            reply_to: message_parameter.reply_to,
            status: status
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageParameter {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    time_received: Option<Timestamp>,
    status: Status,
    reply_to: Option<EntryHash>,
}

impl MessageParameter {
    pub fn from_entry(message_entry: P2PMessage) -> Self {
        MessageParameter {
            author: message_entry.author,
            receiver: message_entry.receiver,
            payload: message_entry.payload,
            time_sent: message_entry.time_sent,
            time_received: message_entry.time_received,
            status: message_entry.status,
            reply_to: message_entry.reply_to,
        }
    }

    pub fn from_async_entry(message_entry: P2PMessageAsync, status: Status) -> Self {
        MessageParameter {
            author: message_entry.author,
            receiver: message_entry.receiver,
            payload: message_entry.payload,
            time_sent: message_entry.time_sent,
            time_received: message_entry.time_received,
            reply_to: message_entry.reply_to,
            status: status
        }
    }
}

#[hdk_entry(id = "inbox", visibility = "public")]
pub struct Inbox {
    owner: AgentPubKey,
    tag: String
}

impl Inbox {
    pub fn new(agent_pubkey: AgentPubKey) -> Self {
        Inbox {
            owner: agent_pubkey,
            tag: "inbox".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey,
    payload: String,
    reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessagesByAgent {
    author: AgentPubKey,
    messages: Vec<MessageParameter>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageRange {
    author: AgentPubKey,
    last_message_timestamp_seconds: i64,
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
    reply: String,
}

#[hdk_entry(id = "preference", visibility = "private")]
pub struct Preference {
    typing_indicator: bool,
    read_receipt: bool,
}

#[hdk_entry(id = "per_agent_preference", visibility = "private")]
pub struct PerAgentPreference {
    typing_indicator: Vec<AgentPubKey>,
    read_receipt: Vec<AgentPubKey>,
}

#[hdk_entry(id = "per_group_preference", visibility = "private")]
pub struct PerGroupPreference {
    typing_indicator: Vec<String>,
    read_receipt: Vec<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PreferenceIO {
    typing_indicator: Option<bool>,
    read_receipt: Option<bool>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PerAgentPreferenceIO {
    typing_indicator: Option<Vec<AgentPubKey>>,
    read_receipt: Option<Vec<AgentPubKey>>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct PerGroupPreferenceIO {
    typing_indicator: Option<Vec<String>>,
    read_receipt: Option<Vec<String>>,
}

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct PreferenceWrapper(PreferenceIO);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct PerAgentPreferenceWrapper(PerAgentPreferenceIO);

#[derive(From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct PerGroupPreferenceWrapper(PerGroupPreferenceIO);

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingInfo {
    agent: AgentPubKey,
    is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct TypingSignal {
    kind: String,
    agent: AgentPubKey,
    is_typing: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageSignal {
    kind: String,
    message: MessageParameter,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Signal {
    Message(MessageSignal),
    Typing(TypingSignal),
}
