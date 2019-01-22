use std::io::{BufReader, BufRead, BufWriter, Read, Write};

pub struct Config<A: Read, B: Write> {
    pub keyword: String,
    pub source: A,
    pub sink: B,
    pub decipher: bool,
}

pub fn run(config: Config<impl Read, impl Write>) -> Result<(), String> {
    // Create cyclic iterator over keyword characters
    let key = config.keyword.to_ascii_uppercase();
    let input = BufReader::new(config.source);
    let do_decipher = config.decipher;

    let mut output = BufWriter::new(config.sink);
    let mut key_iter = key.as_bytes().into_iter().cycle();

    for line in input.lines() {
        let line = match line {
            Ok(l) => l.to_ascii_uppercase(),
            Err(e) => return Err(format!("Failed to read lines from input file: {}", e.to_string())),
        };

        let line_bytes = line.as_bytes().into_iter();

        let mut crypted = if do_decipher {
            line_bytes.map(|byte| {
                if *byte < 65u8 || *byte > 90u8 {
                    return *byte;
                }

                let key = key_iter.next().unwrap();
                (byte + 26u8 - key) % 26u8 + 65u8
            }).collect::<Vec<u8>>()
        }
        else {
            line_bytes.map(|byte| {
                if *byte < 65u8 || *byte > 90u8 {
                    return *byte;
                }

                let key = key_iter.next().unwrap();
                (byte + key) % 26u8 + 65u8
            }).collect::<Vec<u8>>()
        };

        // Push new line
        crypted.push(0x0A);

        match output.write(&crypted[..]) {
            Ok(_) => 0,
            Err(e) => return Err(format!("Failed to write lines to output file: {}", e.to_string())),
        };
    }

    Ok(())
}
