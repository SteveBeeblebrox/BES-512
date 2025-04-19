#![feature(exit_status_error)]
use clap::{Command,Arg,ArgAction};
use anyhow::Error;

fn main() -> Result<(), Error> {
    let matches = Command::new("xtask")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!(", "))
        .disable_colored_help(true)
		.disable_help_subcommand(true)
		.arg_required_else_help(true)
        .disable_version_flag(true)

        .arg(Arg::new("version")
            .long("version")
            .short('v')
            .action(ArgAction::Version)
            .help("Print version")
        )

        .subcommand(Command::new("doc").about("Custom `cargo doc` with Katex support"))
        .get_matches();

    match matches.subcommand() {
        Some(("doc",_)) => {
            let katex: String = format!("{}/katex.html", std::env::var("CARGO_WORKSPACE_DIR").unwrap_or(String::from("..")));
            std::process::Command::new("cargo")
                .arg("doc")
                .env("RUSTDOCFLAGS", format!(
                    "--html-in-header={}", std::fs::canonicalize(&katex).ok().and_then(|p| p.into_os_string().into_string().ok()).unwrap()))
                .status().map(|s| s.exit_ok().ok())
                .expect("Failed to run `cargo doc`");
        }
        _ => ()
    };

    return Ok(());
}
