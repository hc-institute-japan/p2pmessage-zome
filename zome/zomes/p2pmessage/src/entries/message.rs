use derive_more::{From, Into};
use hdk3::prelude::{timestamp::Timestamp, *};
pub mod handlers;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Status {
    Sent,      // the message has been transmitted to the network
    Delivered, // the message has successfully traversed the network and reached the receiver
    Read,      // the message has been opened by the receiver
    Failed,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct P2PMessage {
    author: AgentPubKey,
    receiver: AgentPubKey,
    payload: String,
    time_sent: Timestamp,
    reply_to: Option<EntryHash>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct P2PMessageReceipt {
    id: EntryHash,
    time_received: Timestamp,
    status: Status,
}

entry_def!(P2PMessage
    EntryDef{
        id: "p2pmessage".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

entry_def!(P2PMessageReceipt
    EntryDef{
        id: "p2pmessagereceipt".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

impl P2PMessage {
    pub fn from_input(input: MessageInput) -> ExternResult<Self> {
        let now = sys_time()?;
        let message = P2PMessage {
            author: agent_info()?.agent_latest_pubkey,
            receiver: input.receiver,
            payload: input.payload,
            time_sent: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
            reply_to: input.reply_to,
        };
        Ok(message)
    }
}

impl P2PMessageReceipt {
    pub fn from_message(message: P2PMessage) -> ExternResult<Self> {
        let now = sys_time()?;
        let receipt = P2PMessageReceipt {
            id: hash_entry(&message)?,
            time_received: Timestamp(now.as_secs() as i64, now.subsec_nanos()),
            status: Status::Delivered,
        };
        Ok(receipt)
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct MessageInput {
    receiver: AgentPubKey,
    payload: String,
    reply_to: Option<EntryHash>,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct BooleanWrapper(bool);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct ReceiptHash(EntryHash);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct MessageHash(EntryHash);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct MessageBundle(P2PMessage, Vec<String>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct MessageAndReceipt(P2PMessage, P2PMessageReceipt);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct AgentMessages(HashMap<String, Vec<String>>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct MessageContents(HashMap<String, MessageBundle>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct ReceiptContents(HashMap<String, P2PMessageReceipt>);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct P2PMessageHashTables(AgentMessages, MessageContents, ReceiptContents);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct P2PMessageFilterAgentTimestamp {
    conversant: AgentPubKey,
    date: Timestamp,
}

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct BatchSize(u8);

#[derive(From, Into, Serialize, Deserialize, Clone, SerializedBytes)]
pub struct P2PMessageFilterBatch {
    conversant: AgentPubKey,
    batch_size: u8,
    last_fetched_timestamp: Timestamp, // header timestamp; oldest message in the last fetched message
    last_fetched_message_id: EntryHash,
}

// TYPING
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
    message: P2PMessage,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub enum Signal {
    Message(MessageSignal),
    Typing(TypingSignal),
}
