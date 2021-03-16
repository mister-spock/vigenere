use std::io::{BufReader, BufRead, BufWriter, Read, Write};

const CAP_A: u8 = b'A';
const CAP_Z: u8 = b'Z';
const LF: u8 = b'\n';
const NUM_LETTERS: u8 = 26;

/// Configuration object that has to be passed to the `run` function
pub struct Config<'a> {
    /// Cipher keyword
    pub keyword: String,
    /// Source of the ciphertext (or plaintext). A file, an input stream, etc.
    pub source: &'a mut dyn Read,
    /// Sink for the ciphered or deciphered text. A file, an output stream, etc.
    pub sink: &'a mut dyn Write,
    /// If `true` will attempt a decipher, applies cipher otherwise
    pub decipher: bool,
}

/// Implements ciphering and deciphering logic. Depending on the given `Config` struct will perform either,
/// reading from the `source` and writing to the `sink`. Returns unit in case of success, or error string
/// in case of an error.
pub fn run(config: Config) -> Result<(), String> {
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
        crypted.push(LF);

        match output.write(&crypted[..]) {
            Ok(_) => 0,
            Err(e) => return Err(format!("Failed to write lines to output file: {}", e.to_string())),
        };
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ciphering() {
        let string = b"Hello, world!";
        let mut input = BufReader::new(&string[..]);
        let mut output: Vec<u8> = vec![];

        let config = Config {
            keyword: "foo".to_owned(),
            source: &mut input,
            sink: &mut output,
            decipher: false,
        };

        run(config).expect("Should be OK");

        assert_eq!(output, b"MSZQC, KTFZI!\n");
    }

    #[test]
    fn test_deciphering() {
        let string = b"WLW YAVGB TUSOV LBB AMPTK WBRV KZH PSHE QSX";
        let mut input = BufReader::new(&string[..]);
        let mut output: Vec<u8> = vec![];

        let config = Config {
            keyword: "designers".to_owned(),
            source: &mut input,
            sink: &mut output,
            decipher: true,
        };

        run(config).expect("Should be OK");

        assert_eq!(output, b"THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG\n");
    }
}
