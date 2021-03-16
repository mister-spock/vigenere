use getopts::{Matches, Options};
use std::fs::File;
use std::{env, io, process};
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
            eprintln!("Failed to parse parameters with: {}", f.to_string());
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
            eprintln!("ERROR: Ciphering keyword must be specified");
            process::exit(1);
        }
    };

    let mut source = get_source(&matches);
    let mut sink = get_sink(&matches);
    
    let config = Config {
        keyword,
        source: &mut source,
        sink: &mut sink,
        decipher: matches.opt_present("d"),
    };

    // Run processing
    match vigenere::run(config) {
        Ok(_) => process::exit(0),
        Err(f) => {
            eprintln!("Failed to process cipher with: {}", f);
            process::exit(1);
        },
    };
}

/// Opens a file for input, or connects to STDIN
fn get_source(matches: &Matches) -> Box<dyn io::Read> {
    match matches.opt_str("i") {
        None => Box::new(io::stdin()),
        Some(path) => Box::new(File::open(path).expect("Failed to open input file")),
    }
}

/// Creates or truncates (if exists already) a file for output,
/// or connects to STDOUT
fn get_sink(matches: &Matches) -> Box<dyn io::Write> {
    match matches.opt_str("o") {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(File::create(path).expect("Failed to open output file")),
    }
}

/// Generates usage information string out of options object
fn generate_usage(opts: &Options) -> String {
    let brief = "USAGE: vigenere --key STRING [--input FILE] [--output FILE] [--decipher]
\nOr just pipe text to the program to get (un)ciphered text to STDOUT.";
    opts.usage(&brief)
}
