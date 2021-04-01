use crate::git::GitRepo;
use crate::package::{self, Package};
use crate::task::{TaskManager, TaskType};
use crate::{Error, Result};
use clap::{value_t, ArgMatches};

#[derive(Debug)]
struct UpdateArgs {
    plugins: Vec<String>,
    skip: Vec<String>,
    threads: Option<usize>,
    paconfig: bool,
}

impl UpdateArgs {
    fn from_matches(m: &ArgMatches) -> UpdateArgs {
        UpdateArgs {
            plugins: m.values_of_lossy("package").unwrap_or_else(|| vec![]),
            skip: m.values_of_lossy("skip").unwrap_or_else(|| vec![]),
            threads: value_t!(m, "threads", usize).ok(),
            // TODO: remove this opt (already removed from cli)
            paconfig: m.is_present("paconfig"),
        }
    }
}

pub fn exec(matches: &ArgMatches) {
    let args = UpdateArgs::from_matches(matches);

    if args.paconfig {
        if let Err(e) = update_paconfig() {
            die!("Err: {}", e);
        }
        return;
    }

    let threads = args.threads.unwrap_or_else(num_cpus::get);
    if threads < 1 {
        die!("Threads should be greater than 0");
    }

    if let Err(e) = update_plugins(&args.plugins, threads, &args.skip) {
        die!("Err: {}", e);
    }
}

fn update_paconfig() -> Result<()> {
    println!("Update _pack file for all plugins.");
    let mut packs = package::fetch()?;

    packs.sort_by(|a, b| a.idname.cmp(&b.idname));
    package::update_pac_plugin(&packs)?;

    Ok(())
}

fn update_plugins(plugins: &[String], threads: usize, skip: &[String]) -> Result<()> {
    let mut packs = package::fetch()?;

    let mut manager = TaskManager::new(TaskType::Update, threads);
    if plugins.is_empty() {
        for pack in &packs {
            if skip.iter().any(|x| pack.idname.contains(x)) {
                println!("Skip {}", pack.idname);
                continue;
            }
            manager.add(pack.clone());
        }
    } else {
        for pack in packs.iter().filter(|x| plugins.contains(&x.idname)) {
            manager.add(pack.clone());
        }
    }

    for fail in manager.run(update_plugin) {
        packs.retain(|e| e.idname != fail);
    }

    packs.sort_by(|a, b| a.idname.cmp(&b.idname));

    package::update_pac_plugin(&packs)?;

    Ok(())
}

fn update_plugin(pack: &Package) -> (Result<()>, bool) {
    let res = do_update(pack);
    let status = match res {
        Err(Error::SkipLocal) | Err(Error::Git(_)) => true,
        Err(_) => false,
        _ => true,
    };
    (res, status)
}

fn do_update(pack: &Package) -> Result<()> {
    let path = pack.path();
    if !path.is_dir() {
        Err(Error::plugin_not_installed(&pack.idname))
    } else {
        pack.git_pull()
    }
}
