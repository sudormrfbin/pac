use crate::package::{self, Package};
use crate::{Error, Result};

use clap::ArgMatches;
use std::fs;

#[derive(Debug)]
struct UninstallArgs {
    plugins: Vec<String>,
}

impl UninstallArgs {
    fn from_matches(m: &ArgMatches) -> UninstallArgs {
        UninstallArgs {
            plugins: m.values_of_lossy("package").unwrap_or_else(|| vec![]),
        }
    }
}

pub fn exec(matches: &ArgMatches) {
    let args = UninstallArgs::from_matches(matches);

    if let Err(e) = uninstall_plugins(&args.plugins) {
        die!("{}", e);
    }
}

/// Uninstall multiple plugins based on plugin names.
fn uninstall_plugins(plugins: &[String]) -> Result<()> {
    let mut packs = package::fetch()?;

    let to_uninstall = plugins
        .iter()
        .map(
            |plugin| match packs.iter().find(|pack| &pack.idname == plugin) {
                Some(p) => Ok(p),
                None => Err(Error::plugin_not_installed(plugin)),
            },
        )
        .collect::<Result<Vec<&Package>>>()?;

    for pack in to_uninstall {
        uninstall_plugin(pack)?;
    }

    packs.retain(|x| !plugins.contains(&x.idname)); // keep only installed plugins
    packs.sort_by(|a, b| a.idname.cmp(&b.idname));
    package::update_pack_plugin(&packs)?;
    package::save(packs)?;

    println!();
    println!("Uninstalled {}", plugins.join(", "));
    Ok(())
}

/// Uninstall a specific plugin.
fn uninstall_plugin(plugin: &Package) -> Result<()> {
    let plugin_path = plugin.path();

    if plugin_path.is_dir() {
        fs::remove_dir_all(&plugin_path)?;
    }

    Ok(())
}
