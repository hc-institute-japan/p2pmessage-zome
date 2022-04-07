// ENTRY STRUCTURES
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemMessage {
    pub author: AgentPubKey,
    pub receiver: AgentPubKey,
    pub message: String,
}

// ENTRY DEFINITIONS
entry_def!(SystemMessage
    EntryDef {
        id: "systemmessage".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct NotificationInput {
    receiver: AgentPubKey,
    message: String,
}

// SIGNAL STRUCTURES
#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    SystemMessage(SystemMessageSignal),
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
pub struct SystemMessageSignal {
    message: SystemMessage,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
pub struct SystemMessages(HashMap<AgentPubKey, SystemMessage>);
