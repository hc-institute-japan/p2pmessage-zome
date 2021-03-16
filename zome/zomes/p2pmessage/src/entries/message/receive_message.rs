use hdk::prelude::*;

use super::{
    ReceiveMessageInput,
    P2PMessageReceipt,
};

pub fn handler(input: ReceiveMessageInput) -> ExternResult<P2PMessageReceipt> {
    let receipt = P2PMessageReceipt::from_message(input.0.clone())?;
    create_entry(&input.0)?;
    create_entry(&receipt)?;
    if let Some(file) = input.1 {
        create_entry(&file)?;
    };
    Ok(receipt)
}