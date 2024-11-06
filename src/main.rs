use clap::{Arg, Command};
use url_parser::ParsedURL;

fn main() {
    let matches = Command::new("url_parser")
        .version("1.0")
        .author("Mariia Sychova")
        .about("Parses URLs into components")
        .arg(
            Arg::new("url")
                .help("URL to parse")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(url) = matches.get_one::<String>("url") {
        match ParsedURL::parse(url) {
            Ok(parsed) => println!("{:?}", parsed),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}