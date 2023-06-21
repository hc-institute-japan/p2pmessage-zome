use hdk::prelude::*;

/*
 * ZOME INIT FUNCTION TO SET UNRESTRICTED ACCESS
 */

pub fn init_handler() -> ExternResult<InitCallbackResult> {
    let zome_name: ZomeName = zome_info()?.name;

    let mut receive_message_function = BTreeSet::new();
    receive_message_function.insert((zome_name.clone(), "receive_message".into()));
    let receive_message_functions: GrantedFunctions =
        GrantedFunctions::Listed(receive_message_function);

    let mut receive_receipt_function = BTreeSet::new();
    receive_receipt_function.insert((zome_name.clone(), "receive_receipt".into()));
    let receive_receipt_functions: GrantedFunctions =
        GrantedFunctions::Listed(receive_receipt_function);

    let mut typing_function = BTreeSet::new();
    typing_function.insert((zome_name.clone(), "typing".into()));
    let typing_functions: GrantedFunctions = GrantedFunctions::Listed(typing_function);

    let mut recv_remote_signal_function = BTreeSet::new();
    recv_remote_signal_function.insert((zome_name.clone(), "recv_remote_signal".into()));
    let recv_remote_signal_functions: GrantedFunctions =
        GrantedFunctions::Listed(recv_remote_signal_function);

    let mut post_commit_function = BTreeSet::new();
    post_commit_function.insert((zome_name.clone(), "post_commit".into()));
    let post_commit_functions: GrantedFunctions = GrantedFunctions::Listed(post_commit_function);

    let mut sync_pins_function = BTreeSet::new();
    sync_pins_function.insert((zome_name.clone(), "sync_pins".into()));
    let sync_pins_functions: GrantedFunctions = GrantedFunctions::Listed(sync_pins_function);

    create_cap_grant(CapGrantEntry {
        tag: "receive_message".into(),
        access: CapAccess::Unrestricted,
        functions: receive_message_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "receive_receipt".into(),
        access: CapAccess::Unrestricted,
        functions: receive_receipt_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "typing".into(),
        access: CapAccess::Unrestricted,
        functions: typing_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "recv_remote_signal".into(),
        access: CapAccess::Unrestricted,
        functions: recv_remote_signal_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "post_commit".into(),
        access: CapAccess::Unrestricted,
        functions: post_commit_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "sync_pins".into(),
        access: CapAccess::Unrestricted,
        functions: sync_pins_functions,
    })?;

    Ok(InitCallbackResult::Pass)
}
