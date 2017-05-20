extern crate clap;

use clap::{App, Arg};
use std::io;
use std::io::{Write};

fn main() {

    let matches = App::new("xor-genkeys")
        .version("0.1.0")
        .author("Gavyn Riebau")
        .about("Generates sets of ascii values that can be used as guessed keys when decrypting xor encrypted content")
        .arg(Arg::with_name("LENGTH")
             .help("The assumed key length")
             .default_value("1")
             .takes_value(true))
        .get_matches();

    let length_str = matches.value_of("LENGTH").unwrap();
    let length = length_str.parse::<u32>().expect("Failed to parse LENGTH into u32. LENGTH must be numeric");
    let max = 128u32.pow(length);

    for i in 0..max {
        let mut value = i;

        for j in (0..length).rev() {
            let digit = value / 128u32.pow(j);
            value = value - digit * 128u32.pow(j);
            print!("{}", (digit as u8) as char);
        }

        println!();
    }

    io::stdout().flush().expect("Failed to flush stdout.");
}
