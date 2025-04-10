// use clap::{Command,Arg,ArgAction,ValueEnum};
// use strum::IntoStaticStr;
// use enum_assoc::Assoc;
// use anyhow::{Error,Context as _};

// use std::io;

// #[derive(Clone, ValueEnum, IntoStaticStr, Assoc, Default)]
// #[value(rename_all = "kebab-case")]
// #[strum(serialize_all = "snake_case")]
// #[allow(non_camel_case_types)]
// #[non_exhaustive]
// enum CipherMode {
//     #[default]
//     Block,
//     ECB,
//     CBC,
// }

// #[derive(Clone, ValueEnum, IntoStaticStr, Assoc, Default)]
// #[value(rename_all = "kebab-case")]
// #[strum(serialize_all = "kebab-case")]
// #[allow(non_camel_case_types)]
// #[non_exhaustive]
// #[func(pub const fn key_size(&self) -> u16)]
// #[func(pub const fn block_size(&self) -> u16)]
// enum Cipher {
//     #[default]
// 	#[assoc(key_size = 512, block_size = 128)]
//     BES_512,
	
// 	#[assoc(key_size = 256, block_size = 128)]
//     AES_256,
	
// 	#[assoc(key_size = 192, block_size = 128)]
//     AES_192,
	
// 	#[assoc(key_size = 128, block_size = 128)]
//     AES_128
// }


// fn get_key(text: impl AsRef<str>) -> Result<Box<[u8]>, Error> {
//     match text.as_ref().trim().strip_prefix("0x") {
//         Some(key) => {
//             key.as_bytes().chunks(2).map(|x| u8::from_str_radix(str::from_utf8(x).unwrap(),16).map_err(Error::new)).collect::<Result<Vec<u8>,Error>>().with_context(|| format!("Invalid hexadecimal digit in key '{}'", key))
//         },
//         _ => std::fs::read(text.as_ref()).with_context(|| format!("Failed to read key from {}", text.as_ref()))
//     }.map(Vec::into_boxed_slice)
// }

// fn main() -> Result<(), Error> {
//     let matches = Command::new("BES-512")
//         .version(clap::crate_version!())
//         .about(clap::crate_description!())
//         .author(clap::crate_authors!(", "))
//         .disable_colored_help(true)
// 		.disable_help_subcommand(true)
// 		.arg_required_else_help(true)
//         .disable_version_flag(true)
// 	.arg(Arg::new("version")
// 		.long("version")
// 		.short('v')
// 		.action(ArgAction::Version)
// 		.help("Print version")
// 	)
// 	.arg(Arg::new("action")
// 		.value_name("ACTION")
// 		.value_parser(["enc", "dec"])
// 		.required(true)
// 	)
// 	.arg(Arg::new("cipher")
// 		.long("cipher")
// 		.short('c')
// 		.value_name("CIPHER")
// 		.value_parser(clap::value_parser!(Cipher))
// 		.default_value(Into::<&str>::into(Cipher::default()))
// 		.help("Set cipher")
// 	)
// 	.arg(Arg::new("mode")
// 		.long("mode")
// 		.short('m')
// 		.value_name("MODE")
// 		.value_parser(clap::value_parser!(CipherMode))
// 		.default_value(Into::<&str>::into(CipherMode::default()))
// 		.help("Set cipher mode of operation")
// 	)
// 	.arg(Arg::new("key")
// 		.long("key")
// 		.short('k')
// 		.value_name("HEX_KEY | PATH")
// 		.help("Set key")
// 		.required(true)
// 	)
// 	.get_matches();

//     match matches.get_one::<String>("action").unwrap().as_str() {
// 		"enc" => println!("Encrypting"),
// 		"dec" => println!("Decrypting"),
// 		_ => ()
//     }
    
//     println!("Hello, world! 1+1={}", bes_512_core::add(1, 1));

// 	return Ok(());
// }


fn main() {
	bes_512_core::encrypt();
}