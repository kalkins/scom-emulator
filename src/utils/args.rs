extern crate clap;
use clap::{Arg, App};

use std::str::FromStr;

use super::settings::Settings;

pub fn parse_arguments() -> Settings {
    let matches = App::new("SCOM Emulator")
        .version("0.1.0")
        .author("Sindre Stephansen <sindre@sindrestephansen.com>")
        .about("An emulator for the SCOM 8-bit homemade computer")
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("log-file")
             .short("l")
             .long("log-file")
             .value_name("LOG-FILE")
             .help("Print log events to the given file")
             .takes_value(true))
        .arg(Arg::with_name("log-to-stdout")
             .short("s")
             .long("log-to-stdout")
             .help("Print log events to stdout"))
        .arg(Arg::with_name("input")
             .value_name("INPUT")
             .help("Program to execute")
             .required(true)
             .index(1))
        .arg(Arg::with_name("memory-size")
             .value_name("SIZE")
             .help("The size of the machines memory (default 32K)")
             .short("m")
             .long("memory-size"))
        .get_matches();


    Settings {
        verbose: matches.occurrences_of("verbose"),
        log_to_stdout: matches.is_present("log-to-stdout"),
        log_file: matches.value_of("log-file").map(|s| s.to_string()),
        in_file: matches.value_of("input").unwrap().to_string(),
        memory_size: match matches.value_of("memory-size").map(parse_memory_size) {
            Some(Ok(val)) => val,
            Some(Err(err)) => panic!(err),
            _ => 32768,
        }
    }
}

fn parse_memory_size(input: &str) -> Result<usize, <usize as FromStr>::Err> {
    let factor = 1024_usize.pow(match input.chars().last() {
        Some('K') | Some('k') => 1,
        Some('M') | Some('m') => 2,
        Some('G') | Some('g') => 3,
        _ => 0,
    });

    input.parse::<usize>().map(|x| x * factor)
}
