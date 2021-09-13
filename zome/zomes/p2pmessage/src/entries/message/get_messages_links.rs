use hdk::prelude::*;

use super::BaseAndTag;

pub fn get_messages_links_handler(input: BaseAndTag) -> ExternResult<Links> {
    let base_hash: EntryHash = input.base.into();
    let links = get_links(base_hash.clone(), Some(LinkTag::new(input.tag)))?;
    debug!(
        "nicko get message links from base {:?} are {:?}",
        base_hash, links
    );

    Ok(links)
}
