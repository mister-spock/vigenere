## Simple implementation of Vigenere cipher in Rust

*Works only with capital ASCII letters*

Build and run with `vigenere --help` to get help.
Can work with files or standard stream or a mix of both.
Errors are output to STDERR.

Usage:
* To work with files: `vigenere -d -i ciphertext.txt -o plaintext.txt --key <keyword>`
* To work with streams: `cat plaintext.txt | vigenere --key <keyword>`. Otputs to STDOUT.
* To work with mixed IO: `cat ciphertext.txt | vigenere --decipher --key <keywords> -o ./plaintext.txt` or `vigenere -i plaintext.txt --key <keyword>` (outputs to STDOUT) or `cat ./plaintext.txt | vigenere --key <keyword> > ./ciphertext.txt`

Will prioritize files if `--input` and/or `--output` parameters are given.
