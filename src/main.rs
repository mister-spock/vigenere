use getopts::{Matches, Options};
use std::fs::File;
use std::io;
use std::{env, process};
use vigenere::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help manu");
    opts.optflag("d", "decipher", "decipher the message instead of ciphering");

    opts.optopt("k", "key", "keyword to be used for Vigenere cipher", "STRING",);
    opts.optopt("i", "input", "path to the input file", "FILE");
    opts.optopt("o", "output", "path to the output file", "FILE");

    // Get matches
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            println!("Failed to parse parameters with: {}", f.to_string());
            process::exit(1);
        }
    };

    // Display help and exit
    if matches.opt_present("h") {
        println!("{}", generate_usage(&opts));
        process::exit(0);
    }

    let keyword = match matches.opt_str("k") {
        Some(s) => s,
        None => {
            println!("ERROR: Ciphering keyword must be specified");
            process::exit(1);
        }
    };
    let input_file = match open_input_file(&matches) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            process::exit(1);
        }
    };
    let output_file = match create_output_file(&matches) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            process::exit(1);
        }
    };

    let config = Config {
        keyword,
        input_file,
        output_file,
        decipher: matches.opt_present("d"),
    };

    // Run processing
    match vigenere::run(config) {
        Ok(_) => process::exit(0),
        Err(f) => {
            println!("Failed to process cipher with: {}", f);
            process::exit(1);
        },
    };
}

/// Opens a file for input
fn open_input_file(matches: &Matches) -> io::Result<File> {
    match matches.opt_str("i") {
        None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file path must be specified")),
        Some(path) => File::open(path),
    }
}

/// Creates or truncates (if exists already) a file for output
fn create_output_file(matches: &Matches) -> io::Result<File> {
    match matches.opt_str("o") {
        None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Output file path must be specified")),
        Some(path) => File::create(path),
    }
}

/// Generates usage information string out of options object
fn generate_usage(opts: &Options) -> String {
    let brief = "USAGE: vigenere --key STRING --input FILE --output FILE [--decipher]";
    opts.usage(&brief)
}
