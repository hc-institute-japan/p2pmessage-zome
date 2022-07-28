use hdi::prelude::*;
use p2pmessage_integrity_types::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
enum EntryTypes {
    #[entry_def(name="p2pmessage", required_validations=5, visibility="private")]
    P2PMessage(P2PMessage),
    #[entry_def(name="p2pmessagereceipt", required_validations=5, visibility="private")]
    P2PMessageReceipt(P2PMessageReceipt),
    #[entry_def(name="p2pmessagepin", required_validations=5, visibility="private")]
    P2PMessagePin(P2PMessagePin),
    #[entry_def(name="p2pfilebytes", required_validations=5, visibility="private")]
    P2PFileBytes(P2PFileBytes),
}