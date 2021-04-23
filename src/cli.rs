use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("pac")
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List installed packages")
                .arg(
                    Arg::with_name("start")
                        .long("start")
                        .short("s")
                        .conflicts_with("opt")
                        .help("List start packages"),
                )
                .arg(
                    Arg::with_name("opt")
                        .long("opt")
                        .short("o")
                        .conflicts_with("start")
                        .help("List optional packages"),
                )
                .arg(
                    Arg::with_name("detached")
                        .long("detached")
                        .short("d")
                        .help("List detached(untracked) packages"),
                )
                .arg(
                    Arg::with_name("category")
                        .long("category")
                        .short("c")
                        .help("List packages under this category")
                        .value_name("CATEGORY"),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install new packages/plugins")
                .arg(
                    Arg::with_name("opt")
                        .short("o")
                        .long("opt")
                        .help("Install plugins as opt(ional)"),
                )
                .arg(
                    Arg::with_name("category")
                        .long("category")
                        .short("c")
                        .help("Install package under provided category")
                        .default_value("default")
                        .value_name("CATEGORY"),
                )
                .arg(
                    Arg::with_name("branch")
                        .long("branch")
                        .help("Checkout this branch by default")
                        .value_name("BRANCH")
                        .conflicts_with_all(&["tag", "commit"]),
                )
                .arg(
                    Arg::with_name("tag")
                        .long("tag")
                        .help("Checkout this tag by default")
                        .value_name("TAG"),
                )
                .arg(
                    Arg::with_name("commit")
                        .long("commit")
                        .help("Checkout this commit by default")
                        .value_name("COMMIT_HASH")
                        .validator(|commit| match commit.len() {
                            40 => Ok(()),
                            _ => Err("Invalid commit hash length, please provide the full hash"
                                .to_string()),
                        }),
                )
                .arg(
                    Arg::with_name("as")
                        .long("as")
                        .help("Install plugin under this name")
                        .value_name("NAME"),
                )
                .arg(
                    Arg::with_name("on")
                        .long("on")
                        .help("Command for loading the plugins")
                        .value_name("LOAD_CMD"),
                )
                .arg(
                    Arg::with_name("for")
                        .long("for")
                        .help("Load this plugins for specific types")
                        .value_name("TYPES"),
                )
                .arg(
                    Arg::with_name("build")
                        .long("build")
                        .help("Build command for build package")
                        .value_name("BUILD_CMD"),
                )
                .arg(
                    Arg::with_name("threads")
                        .short("j")
                        .long("threads")
                        .help("Installing packages concurrently")
                        .value_name("THREADS"),
                )
                .arg(Arg::with_name("package").multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstall packages/plugins")
                .arg(Arg::with_name("package").required(true).multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Move a package to a different category or make it optional.")
                .arg(
                    Arg::with_name("opt")
                        .conflicts_with("category")
                        .long("opt")
                        .short("o")
                        .help("Make package optional"),
                )
                .arg(
                    Arg::with_name("package")
                        .help("Package to move")
                        .required(true),
                )
                .arg(
                    Arg::with_name("category")
                        .conflicts_with("opt")
                        .help("Category to move the package to"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update packages")
                .arg(
                    Arg::with_name("skip")
                        .short("s")
                        .long("skip")
                        .multiple(true)
                        .help("Skip packages"),
                )
                .arg(
                    Arg::with_name("threads")
                        .short("j")
                        .long("threads")
                        .help("Updating packages concurrently"),
                )
                .arg(
                    Arg::with_name("package")
                        .help("Packages to update, default all")
                        .multiple(true),
                ),
        )
        // TODO: remove generate subcommand (package config no longer managed by pac)
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate the pack package file")
                .help("Generate _pac.vim file which combines all package configurations"),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generates completion scripts for your shell")
                .setting(AppSettings::Hidden)
                .arg(
                    Arg::with_name("SHELL")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh"])
                        .help("The shell to generate the script for"),
                ),
        )
}
