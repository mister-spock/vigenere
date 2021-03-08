use std::io::{BufReader, BufRead, BufWriter, Read, Write};

const CAP_A: u8 = b'A';
const CAP_Z: u8 = b'Z';
const NUM_LETTERS: u8 = 26;

pub struct Config<A: Read, B: Write> {
    pub keyword: String,
    pub source: A,
    pub sink: B,
    pub decipher: bool,
}

pub fn run(config: Config<impl Read, impl Write>) -> Result<(), String> {
    // Unpack the struct
    let Config { keyword, source, sink, decipher } = config;

    let key_uppercase = keyword.to_ascii_uppercase();
    let input = BufReader::new(source);
    let mut output = BufWriter::new(sink);
    let mut key_iter = key_uppercase.as_bytes().into_iter().cycle();

    for line in input.lines() {
        let line = match line {
            Ok(l) => l.to_ascii_uppercase(),
            Err(e) => return Err(format!("Failed to read lines from input file: {}", e.to_string())),
        };

        let line_bytes = line.as_bytes().into_iter();

        let mut crypted = line_bytes.map(|byte| {
            if *byte < CAP_A || *byte > CAP_Z {
                return *byte;
            }

            let key_letter = key_iter.next().expect("Unexpected end of cyclic keyword iterator?");

            if decipher {
                (byte + NUM_LETTERS - key_letter) % NUM_LETTERS + CAP_A
            } else {
                (byte + key_letter) % NUM_LETTERS + CAP_A
            }
        }).collect::<Vec<u8>>();

        // Push new line
        crypted.push(0x0A);

        match output.write(&crypted[..]) {
            Ok(_) => 0,
            Err(e) => return Err(format!("Failed to write lines to output file: {}", e.to_string())),
        };
    }

    Ok(())
}
