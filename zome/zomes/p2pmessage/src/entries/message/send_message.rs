use hdk::prelude::*;

use super::{

    P2PMessage,
    MessageInput,
    MessageAndReceipt,
    P2PFileBytes,
    PayloadInput,
    ReceiveMessageInput,
    P2PMessageReceipt,

};

pub fn handler(message_input: MessageInput) -> ExternResult<MessageAndReceipt> {
    
    // TODO: check if receiver is blocked

    let message = P2PMessage::from_input(message_input.clone())?;

    let file = match message_input.payload {
        PayloadInput::File { .. } => Some(P2PFileBytes::from_input(message_input)?),
        _ => None,
    };

    let receive_input = ReceiveMessageInput(message.clone(), file.clone());

    let receive_call_result: ZomeCallResponse = call_remote(
        message.receiver.clone(),
        zome_info()?.zome_name,
        "receive_message".into(),
        None,
        &receive_input,
    )?;

    match receive_call_result {

        ZomeCallResponse::Ok(extern_io)=>{
            let receipt:P2PMessageReceipt = extern_io.decode()?;
            create_entry(&message)?;
            create_entry(&receipt)?;
        
            if let Some(file) = file {
                create_entry(&file)?;
             };

        // TODO: CREATE AND RETURN ELEMENT HERE
            return Ok(MessageAndReceipt(message, receipt));
        },
        ZomeCallResponse::Unauthorized(_,_,_,_) =>{ return crate::err("TODO: 000:", "This case shouldn't happen because of unrestricted access to receive message"); },
        ZomeCallResponse::NetworkError(_) =>{ return crate::err("TODO: 000", "Unknown other error"); },

    }
}