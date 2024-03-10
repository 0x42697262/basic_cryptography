mod caesar;
mod vigenere;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cipher: Cipher,

    #[arg(short = 'd', long, help = "Decode message")]
    decrypt: bool,

    #[arg(required = true, help = "Message")]
    text: String,
}

#[derive(Parser)]
enum Cipher {
    Caesar(CaesarArgs),
    Vigenere(VigenereArgs),
}

#[derive(Parser, Debug)]
struct CaesarArgs {
    #[arg(short = 's', long, help = "Shift value for Caesar cipher")]
    shift: u32,
}

#[derive(Parser)]
struct VigenereArgs {
    #[arg(short = 'k', long, help = "Key for Vigenere cipher")]
    key: String,
}

fn main() {
    let args = Cli::parse();

    let cipher: Cipher = args.cipher;
    let decrypt: bool = args.decrypt;
    let text: String = args.text;

    let output: String;

    match cipher {
        Cipher::Caesar(caesar_args) => {
            let shift: u32 = caesar_args.shift;

            if !decrypt {
                output = caesar::encrypt(&text, shift);
            } else {
                output = caesar::decrypt(&text, shift);
            }
        }
        Cipher::Vigenere(vigenere_args) => {
            let key: String = vigenere_args.key;

            if !decrypt {
                output = vigenere::encrypt(&text, &key);
            } else {
                output = vigenere::decrypt(&text, &key);
            }
        }
    }

    println!("{output}");
}
