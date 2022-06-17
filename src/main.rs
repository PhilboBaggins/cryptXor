#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate exitcode;

use std::process;

use crypt_xor::{crypt_double_xor_in_place, crypt_triple_double_xor_in_place, read_and_crypt};

const FLAG_DOUBLE_XOR: &str = "double-xor";
const FLAG_TRIPLE_DOUBLE_XOR: &str = "triple-double-xor";

fn main() {
    let matches = App::new("CryptXor")
        .version(crate_version!())
        .about("A joke encryption utility")
        .author("Phil B.")
        .arg(Arg::with_name("input-file")
            .help("Input file")
            .takes_value(true)
            .required(true)
            .multiple(false))
        .arg(Arg::with_name("output-file")
            .help("Output file")
            .takes_value(true)
            .required(true)
            .multiple(false))
        .arg(Arg::with_name("block-size")
            .help("Block size to read file")
            .short('b')
            .long("block-size")
            .takes_value(true)
            .default_value("64")
            .multiple(false))
        .arg(Arg::with_name("algorithm")
            .help("\"Encryption\" algorithm to use")
            .short('a')
            .long("algorithm")
            .takes_value(true)
            .multiple(false)
            .possible_values(&[FLAG_DOUBLE_XOR, FLAG_TRIPLE_DOUBLE_XOR])
            .conflicts_with_all(&[FLAG_DOUBLE_XOR, FLAG_TRIPLE_DOUBLE_XOR]))
        .arg(Arg::with_name(FLAG_DOUBLE_XOR)
            .help("Use double-xor algorithm")
            .long(FLAG_DOUBLE_XOR)
            .multiple(false)
            .conflicts_with_all(&["algorithm", FLAG_TRIPLE_DOUBLE_XOR]))
        .arg(Arg::with_name(FLAG_TRIPLE_DOUBLE_XOR)
            .help("Use triple double-xor algorithm")
            .long(FLAG_TRIPLE_DOUBLE_XOR)
            .multiple(false)
            .conflicts_with_all(&["algorithm", FLAG_DOUBLE_XOR]))
        .get_matches();

    // Calling unwrap on these command line options is OK because they are marked as required above
    let input_path = matches.value_of("input-file").unwrap();
    let output_path = matches.value_of("output-file").unwrap();

    let block_size = value_t!(matches, "block-size", usize).unwrap_or_else(|_| {
        eprintln!(
            "Invalid block size \"{}\"",
            matches.value_of("block-size").unwrap()
        );
        process::exit(exitcode::USAGE);
    });

    let algorithm = if matches.is_present(FLAG_DOUBLE_XOR) {
        FLAG_DOUBLE_XOR
    } else if matches.is_present(FLAG_TRIPLE_DOUBLE_XOR) {
        FLAG_TRIPLE_DOUBLE_XOR
    } else {
        matches.value_of("algorithm").unwrap_or(FLAG_DOUBLE_XOR)
    };

    let mut crypt_func = match algorithm {
        FLAG_DOUBLE_XOR => crypt_double_xor_in_place,
        FLAG_TRIPLE_DOUBLE_XOR => crypt_triple_double_xor_in_place,
        _ => {
            eprintln!("Unknown algorithm \"{}\"", algorithm);
            process::exit(exitcode::USAGE);
        }
    };

    match read_and_crypt(input_path, output_path, block_size, &mut crypt_func) {
        Ok(_) => process::exit(exitcode::OK),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(exitcode::IOERR);
        }
    }
}
