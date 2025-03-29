
fn main() {
    let _matches = clap::Command::new("BES-512")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!(", "))
        .disable_colored_help(true)
        .disable_version_flag(true)
        .arg(clap::Arg::new("version")
            .long("version")
            .short('v')
            .action(clap::ArgAction::Version)
            .help("Print version"))
        .get_matches();
    println!("Hello, world! 1+1={}", bes_512_core::add(1, 1));
}
