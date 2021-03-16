use hdk::prelude::*;

pub fn handler() -> ExternResult<InitCallbackResult> {
    let mut receive_functions: GrantedFunctions = HashSet::new();
    receive_functions.insert((zome_info()?.zome_name, "receive_message".into()));
    let mut notify_functions: GrantedFunctions = HashSet::new();
    notify_functions.insert((zome_info()?.zome_name, "notify_delivery".into()));

    create_cap_grant(CapGrantEntry {
        tag: "receive".into(),
        access: ().into(),
        functions: receive_functions,
    })?;

    let mut emit_functions: GrantedFunctions = HashSet::new();
    emit_functions.insert((zome_info()?.zome_name, "is_typing".into()));

    create_cap_grant(CapGrantEntry {
        tag: "notify".into(),
        access: ().into(),
        functions: notify_functions,
    })?;

    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        access: ().into(),
        functions: emit_functions,
    })?;

    //
    let mut fuctions = HashSet::new();

    // TODO: name may be changed to better suit the context of cap grant.s
    let tag: String = "create_group_cap_grant".into();
    let access: CapAccess = CapAccess::Unrestricted;

    let zome_name: ZomeName = zome_info()?.zome_name;
    let function_name: FunctionName = FunctionName("recv_remote_signal".into());

    fuctions.insert((zome_name, function_name));

    let cap_grant_entry: CapGrantEntry = CapGrantEntry::new(
        tag,    // A string by which to later query for saved grants.
        access, // Unrestricted access means any external agent can call the extern
        fuctions,
    );

    create_cap_grant(cap_grant_entry)?;

    let mut receive_receipt_function: GrantedFunctions = HashSet::new();
    receive_receipt_function.insert((zome_info()?.zome_name, "receive_read_receipt".into()));

    create_cap_grant(CapGrantEntry {
        tag: "receipt".into(),
        access: ().into(),
        functions: receive_receipt_function,
    })?;

    Ok(InitCallbackResult::Pass)
}