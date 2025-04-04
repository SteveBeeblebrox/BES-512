use clap::{Command,Arg,ArgAction,ValueEnum};
use strum::IntoStaticStr;

use std::io;

#[derive(Clone, ValueEnum, IntoStaticStr, Default)]
#[value(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
#[allow(non_camel_case_types)]
#[non_exhaustive]
enum CipherMode {
    #[default]
    Block,
    ECB,
    CBC
}

#[derive(Clone, ValueEnum, IntoStaticStr, Default)]
#[value(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[allow(non_camel_case_types)]
#[non_exhaustive]
enum Cipher {
    #[default]
    BES_512,
    AES_256,
    AES_192,
    AES_128
}

fn main() {
    let matches = Command::new("BES-512")
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
	.arg(Arg::new("cipher")
		.long("cipher")
		.short('c')
		.action(ArgAction::Set)
		.value_name("CIPHER")
		.num_args(1)
		.value_parser(clap::value_parser!(Cipher))
		.default_value(Into::<&str>::into(Cipher::default()))
		.help("Set cipher")
		.global(true)	
	)
	.arg(Arg::new("mode")
		.long("mode")
		.short('m')
		.action(ArgAction::Set)
		.value_name("MODE")
		.num_args(1)
		.value_parser(clap::value_parser!(CipherMode))
		.default_value(Into::<&str>::into(CipherMode::default()))
		.help("Set cipher mode of operation")
		.global(true)
	)
	.subcommand(Command::new("enc")
		.about("Encrypt")
	)
	.subcommand(Command::new("dec")
		.about("Decrypt")
	)
        .get_matches();

    match matches.subcommand() {
	Some(("enc", _)) => println!("Encrypting"),
	Some(("dec", _)) => println!("Decrypting"),
	_ => ()
    }
    
    println!("Hello, world! 1+1={}", bes_512_core::add(1, 1));
}
