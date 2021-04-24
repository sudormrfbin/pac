use crate::git::GitRepo;
use crate::package::{self, Package};
use crate::task::{TaskManager, TaskType};
use crate::{Error, Result};

use clap::{value_t, ArgMatches};

#[derive(Debug)]
struct InstallArgs {
    plugins: Vec<String>,
    on: Option<String>,
    for_: Option<String>,
    as_: Option<String>,
    threads: Option<usize>,
    opt: bool,
    category: String,
    build: Option<String>,
    rev: Option<String>,
}

impl InstallArgs {
    fn from_matches(m: &ArgMatches) -> InstallArgs {
        InstallArgs {
            plugins: m.values_of_lossy("package").unwrap_or_default(),
            on: value_t!(m, "on", String).ok(),
            for_: value_t!(m, "for", String).ok(),
            as_: value_t!(m, "as", String).ok(),
            threads: value_t!(m, "threads", usize).ok(),
            opt: m.is_present("opt"),
            category: value_t!(m, "category", String).unwrap_or_default(),
            build: value_t!(m, "build", String).ok(),
            rev: value_t!(m, "rev", String).ok(),
        }
    }
}

pub fn exec(matches: &ArgMatches) {
    let args = InstallArgs::from_matches(matches);

    // This check cannot be done with clap
    if args.as_.is_some() && args.plugins.len() > 1 {
        die!("Multiple plugins cannot be specified with --as");
    }

    let threads = match args.threads {
        Some(t) => t,
        _ => num_cpus::get(),
    };

    if threads < 1 {
        die!("Threads should be greater than 0");
    }

    let opt = args.on.is_some() || args.for_.is_some() || args.opt;
    let types = args
        .for_
        .clone() // map consumes value but we need it in next block
        .map(|e| e.split(',').map(|e| e.to_string()).collect::<Vec<String>>())
        .unwrap_or_default();

    let plugins = args
        .plugins
        .iter()
        .map(|plug| {
            // URL to git clone from
            let remote = if !plug.starts_with("https://") {
                format!("https://github.com/{}", plug)
            } else {
                plug.clone()
            };

            // Install package under this name. Defaults to repo name
            let name = args
                .as_
                // unwrap_or_else consumes self so deref to Option<&str>
                .as_deref()
                .unwrap_or_else(|| remote.rsplitn(2, '/').next().unwrap());

            // FIXME: too many clones
            Package {
                name: name.to_string(),
                idname: Package::idname_from_remote(&remote),
                remote,
                revision: args.rev.clone(),
                category: args.category.clone(),
                opt,
                for_types: types.clone(),
                load_command: args.on.clone(),
                build_command: args.build.clone(),
            }
        })
        .collect::<Vec<_>>();

    if let Err(e) = install_plugins(plugins, threads) {
        die!("Err: {}", e);
    }
}

// FIXME: refactor this
fn install_plugins(toinstall_packs: Vec<Package>, threads: usize) -> Result<()> {
    let mut installed_packs = package::fetch()?;
    {
        let mut manager = TaskManager::new(TaskType::Install, threads);

        if toinstall_packs.is_empty() {
            for pack in &installed_packs {
                manager.add(pack.clone());
            }
        } else {
            for mut toins_pack in toinstall_packs {
                let having = match installed_packs
                    .iter_mut()
                    .find(|ins_pack| ins_pack.idname == toins_pack.idname)
                {
                    Some(ins_pack) => {
                        // plugin in config file but not installed
                        if !ins_pack.is_installed() {
                            ins_pack.set_category(toins_pack.category.as_str());
                            ins_pack.set_opt(toins_pack.opt);
                            ins_pack.set_types(toins_pack.for_types.clone());

                            ins_pack.load_command = toins_pack.load_command.clone();
                            ins_pack.build_command = toins_pack.build_command.clone();
                        } else {
                            toins_pack.set_category(ins_pack.category.as_str());
                            toins_pack.set_opt(ins_pack.opt);
                        }
                        true
                    }
                    None => false,
                };
                if !having {
                    // not yet installed, but add it anyway
                    installed_packs.push(toins_pack.clone());
                }
                manager.add(toins_pack.clone());
            }
        }

        for fail in manager.run(install_plugin) {
            installed_packs.retain(|e| e.idname != fail);
        }
    }

    installed_packs.sort_by(|a, b| a.idname.cmp(&b.idname));

    package::update_pac_plugin(&installed_packs)?;
    package::save(installed_packs)
}

fn install_plugin(pack: &Package) -> (Result<()>, bool) {
    let res = do_install(pack);
    let status = match res {
        Err(Error::PluginInstalled(_)) => true,
        Err(_) => false,
        _ => true,
    };
    (res, status)
}

fn do_install(pack: &Package) -> Result<()> {
    let path = pack.path();
    if path.is_dir() {
        Err(Error::plugin_installed(&path))
    } else {
        pack.git_clone()
    }
}
