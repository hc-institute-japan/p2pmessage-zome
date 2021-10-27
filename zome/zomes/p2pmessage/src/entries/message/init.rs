use hdk::prelude::*;

/*
 * ZOME INIT FUNCTION TO SET UNRESTRICTED ACCESS
 */

pub fn init_handler() -> ExternResult<InitCallbackResult> {
    let zome_name: ZomeName = zome_info()?.name;

    let mut receive_message_function: GrantedFunctions = BTreeSet::new();
    receive_message_function.insert((zome_name.clone(), "receive_message".into()));

    let mut receive_receipt_function = BTreeSet::new();
    receive_receipt_function.insert((zome_name.clone(), "receive_receipt".into()));

    let mut typing_function: GrantedFunctions = BTreeSet::new();
    typing_function.insert((zome_name.clone(), "typing".into()));

    let mut recv_remote_signal_function: GrantedFunctions = BTreeSet::new();
    recv_remote_signal_function.insert((zome_name.clone(), "recv_remote_signal".into()));

    let mut post_commit_function: GrantedFunctions = BTreeSet::new();
    post_commit_function.insert((zome_name.clone(), "post_commit".into()));

    let mut sync_pins_function: GrantedFunctions = BTreeSet::new();
    sync_pins_function.insert((zome_name.clone(), "sync_pins".into()));

    create_cap_grant(CapGrantEntry {
        tag: "receive_message".into(),
        access: CapAccess::Unrestricted,
        functions: receive_message_function,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "receive_receipt".into(),
        access: CapAccess::Unrestricted,
        functions: receive_receipt_function,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "typing".into(),
        access: CapAccess::Unrestricted,
        functions: typing_function,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "recv_remote_signal".into(),
        access: CapAccess::Unrestricted,
        functions: recv_remote_signal_function,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "post_commit".into(),
        access: CapAccess::Unrestricted,
        functions: post_commit_function,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "sync_pins".into(),
        access: CapAccess::Unrestricted,
        functions: sync_pins_function,
    })?;

    Ok(InitCallbackResult::Pass)
}
