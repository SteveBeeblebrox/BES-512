use bes_512::{aes_128,aes_192,aes_256};
use clap::{Command,Arg,ArgAction,ValueEnum};
use strum::IntoStaticStr;
use anyhow::{Error,Context as _};
use std::io::{Read,Write};

mod experiments;

#[derive(Clone, Default, Debug, ValueEnum, IntoStaticStr)]
#[allow(non_camel_case_types)]
#[strum(serialize_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
enum Cipher {
	AES_128,
	AES_192,
	AES_256,
	#[default]
	BES_512 
}

#[derive(Clone, Default, Debug, ValueEnum, IntoStaticStr)]
#[allow(non_camel_case_types)]
#[strum(serialize_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
#[non_exhaustive]
enum CipherMode {
	Block,
	#[default] CBC
}

fn get_bytes<const KEY_SIZE: usize>(text: impl AsRef<str>) -> Result<[u8; KEY_SIZE], Error> {
	let mut bytes= match text.as_ref().trim().strip_prefix("0x") {
        Some(key) => {
            key.as_bytes().chunks(2).map(|x| u8::from_str_radix(str::from_utf8(x).unwrap(),16).map_err(Error::new)).collect::<Result<Vec<u8>,Error>>().with_context(|| format!("Invalid hexadecimal digit in key '{}'", key))
        },
        _ => std::fs::read(text.as_ref()).with_context(|| format!("Failed to read bytes from {}", text.as_ref()))
    }?;

	let len = bytes.len();

	if len < KEY_SIZE {
		bytes.resize(KEY_SIZE, 0);
	}
	
	return bytes.try_into().map_err(|_| Error::msg("Failed to read bytes")).with_context(|| format!("Expected {KEY_SIZE} bytes but got {len}"));
}

fn main() -> Result<(), Error> {
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

		.arg(Arg::new("verbose")
			.long("verbose")
			.short('V')
			.help("Enable verbose output")
			.action(ArgAction::Count)
		)

		.arg(Arg::new("action")
			.value_name("ACTION")
			.value_parser(["enc", "dec"])
			.ignore_case(true)
			.required(true)
			.help("Encrypt or decrypt")
		)

		.arg(Arg::new("key")
			.long("key")
			.short('k')
			.value_name("HEX_KEY | PATH")
			.required(true)
			.help("Set key")
		)

		.arg(Arg::new("cipher")
			.long("cipher")
			.short('c')
			.value_name("CIPHER")
			.value_parser(clap::value_parser!(Cipher))
			.ignore_case(true)
			.default_value(Into::<&str>::into(Cipher::default()))
			.help("Set cipher")
		)

		.arg(Arg::new("mode")
			.long("mode")
			.short('m')
			.value_name("MODE")
			.value_parser(clap::value_parser!(CipherMode))
			.ignore_case(true)
			.default_value(Into::<&str>::into(CipherMode::default()))
			.help("Set mode of operation")
		)

		.arg(Arg::new("iv")
			.long("iv")
			.short('I')
			.value_name("HEX | PATH")
			.default_value("0x0")
			.help("Set IV")
		)

		.arg(Arg::new("input")
			.long("in")
			.short('i')
			.value_name("PATH")
			.default_value("-")
			.help("Set input (\"-\" for stdin)")
		)

		.arg(Arg::new("output")
			.long("out")
			.short('o')
			.value_name("PATH")
			.default_value("-")
			.help("Set output (\"-\" for stdout)")
		)

		.arg(Arg::new("bes-rounds")
			.long("bes-rounds")
			.value_parser(clap::value_parser!(u8).range(1..51))
			.hide(true)
			.help("Override BES-512 rounds")
		)

		.get_matches();

	let _verbose: u8 = matches.get_count("verbose");
	
	let key: &String = matches.get_one::<String>("key").unwrap();

	let input: &mut Box<dyn Read> = &mut match matches.get_one::<String>("input").map(|s| s.as_str()).unwrap_or("-") {
		"-" => Box::new(std::io::stdin()),
		s => Box::new(std::fs::File::open(s)?)
	};

	let output: &mut Box<dyn Write> = &mut match matches.get_one::<String>("output").map(|s| s.as_str()).unwrap_or("-") {
		"-" => Box::new(std::io::stdout()),
		s => Box::new(std::fs::File::open(s)?)
	};

	// if verbose > 0 {
	// 	println!("Verbose: {}", verbose > 0);
	// 	println!("Action:  {}", matches.get_one::<String>("action").unwrap());
	// 	println!("Cipher:  {}", Into::<&str>::into(matches.get_one::<Cipher>("cipher").unwrap()));
	// 	println!("Mode:    {}", Into::<&str>::into(matches.get_one::<CipherMode>("mode").unwrap()));
	// 	println!("Input:   {}", matches.get_one::<String>("input").unwrap());
	// 	println!("Output:  {}", matches.get_one::<String>("output").unwrap());
	// }

	match (
		matches.get_one::<String>("action").map(|s| s.as_str()).unwrap(),
		matches.get_one::<Cipher>("cipher").unwrap(),
		matches.get_one::<CipherMode>("mode").unwrap()
	) {
		("enc", Cipher::AES_128, CipherMode::Block) => block_cipher(input, output, key, aes_128::encrypt)?,
		("dec", Cipher::AES_128, CipherMode::Block) => block_cipher(input, output, key, aes_128::decrypt)?,
		("enc", Cipher::AES_192, CipherMode::Block) => block_cipher(input, output, key, aes_192::encrypt)?,
		("dec", Cipher::AES_192, CipherMode::Block) => block_cipher(input, output, key, aes_192::decrypt)?,
		("enc", Cipher::AES_256, CipherMode::Block) => block_cipher(input, output, key, aes_256::encrypt)?,
		("dec", Cipher::AES_256, CipherMode::Block) => block_cipher(input, output, key, aes_256::decrypt)?,

		("enc", Cipher::BES_512, CipherMode::Block) => block_cipher(input, output, key, experiments::encrypt_with_variable_rounds(matches.get_one::<u8>("override-bes-512-rounds").map(|r| *r).unwrap_or(bes_512::ROUNDS as u8)))?,
		("dec", Cipher::BES_512, CipherMode::Block) => block_cipher(input, output, key, experiments::decrypt_with_variable_rounds(matches.get_one::<u8>("override-bes-512-rounds").map(|r| *r).unwrap_or(bes_512::ROUNDS as u8)))?,

		_ => Err(Error::msg("Unsupported argument configuration!"))?
	}

	return Ok(());
}


fn block_cipher<const BLOCK_SIZE: usize, const KEY_SIZE: usize>(input: &mut Box<dyn Read>, output: &mut Box<dyn Write>, key: &String, cipher: bes_512::CipherFunciton<BLOCK_SIZE, KEY_SIZE>) -> Result<(), Error> {
	let mut buf: Vec<u8> = vec![];
	input.read_to_end(&mut buf)?;
	let len = buf.len();
	output.write_all(cipher(&mut buf.try_into().map_err(|_| Error::msg("Failed to read bytes")).with_context(|| format!("Expected {} bytes but got {}", BLOCK_SIZE, len))?, &get_bytes(key)?))?;
	return Ok(());
}