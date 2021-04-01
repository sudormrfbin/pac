use crate::package;
use crate::Result;
use clap::ArgMatches;

pub fn exec(_matches: &ArgMatches) {
    let _ = update_paconfig();
}

// TODO: code repetition, refactor
fn update_paconfig() -> Result<()> {
    let mut packs = package::fetch()?;

    packs.sort_by(|a, b| a.idname.cmp(&b.idname));
    package::update_pac_plugin(&packs)?;

    Ok(())
}
