use clap::{Arg, Command};
use std::fs;
use url_parser::ParsedURL;

fn main() {
    let matches = Command::new("URL Parser")
        .version("1.0")
        .author("Mariia Sychova")
        .about("A command-line tool to parse URLs into components.")
        .help_template(
            "\
{about}

USAGE:
    cargo run -- 

OPTIONS:
{options}

COMMANDS:
{subcommands}
",
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse")
                .about("Parses a URL provided as a command-line argument.")
                .long_about(
                    "\
Parses a URL directly from the command-line argument into its components.

EXAMPLE USAGE:
    cargo run -- parse http://example.com/products/electronics?query=1#reviews",
                )
                .arg(
                    Arg::new("url")
                        .help("The URL to parse.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("parse-file")
                .about("Parses a URL from a file.")
                .long_about(
                    "\
Reads a URL from a specified file and parses it into components.

EXAMPLE USAGE:
    cargo run -- parse-file test.txt",
                )
                .arg(
                    Arg::new("file")
                        .help("Path to the file containing the URL.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("credits")
                .about("Displays credits for the application.")
                .long_about(
                    "\
Shows information about the program, including the author and acknowledgments.

EXAMPLE USAGE:
    cargo run -- credits",
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            if let Some(url) = sub_m.get_one::<String>("url") {
                parse_url(url);
            }
        }
        Some(("parse-file", sub_m)) => {
            if let Some(file_path) = sub_m.get_one::<String>("file") {
                match fs::read_to_string(file_path) {
                    Ok(contents) => parse_url(contents.trim()),
                    Err(e) => eprintln!("Error reading file: {}", e),
                }
            }
        }
        Some(("credits", _)) => {
            display_credits();
        }
        _ => unreachable!("Clap ensures that subcommand is required"),
    }
}

fn parse_url(url: &str) {
    match ParsedURL::parse(url) {
        Ok(parsed) => println!("{:?}", parsed),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn display_credits() {
    println!("URL Parser v1.0");
    println!("Author: Mariia Sychova");
}
