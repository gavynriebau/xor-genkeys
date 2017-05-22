extern crate clap;
extern crate xor_genkeys;

use clap::{App, Arg};
use std::io;
use std::io::{Write};

fn main() {

    let matches = App::new("xor-genkeys")
        .version("0.2.0")
        .author("Gavyn Riebau")
        .about("Generates sets of ascii values that can be used as guessed keys when decrypting xor encrypted content")
        .arg(Arg::with_name("LENGTH")
             .help("The assumed key length")
             .default_value("1")
             .takes_value(true))
        .get_matches();

    let length_str = matches.value_of("LENGTH").unwrap();
    let length = length_str.parse::<u32>().expect("Failed to parse LENGTH into u32. LENGTH must be numeric");
    let keys = xor_genkeys::gen_ascii_keys(length);

    for key in keys {
        println!("{}", key);
    }

    io::stdout().flush().expect("Failed to flush stdout.");
}
