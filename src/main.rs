#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::prelude::*;

const FLAG_DOUBLE_XOR: &str = "double-xor";
const FLAG_TRIPLE_DOUBLE_XOR: &str = "triple-double-xor";

fn main() {
    let matches = App::new("CryptXor")
        .version(crate_version!())
        .about("????? TODO: Add description")
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
            .short("b")
            .long("block-size")
            .takes_value(true)
            .default_value("64")
            .multiple(false))
        .arg(Arg::with_name("algorithm")
            .help("\"Encryption\" algorithm to use")
            .short("a")
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

    let input_path = matches.value_of("input-file").unwrap();
    let output_path = matches.value_of("output-file").unwrap();

    let block_size = value_t!(matches, "block-size", usize).unwrap();

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
            return;
        },
    };

    let ret = read_and_crypt(input_path, output_path, block_size, &mut crypt_func);
    if let Err(e) = ret {
        eprintln!("Error: {}", e);
    }
}

fn crypt_double_xor_in_place(plaintext: &mut Vec<u8>, key: &Vec<u8>, count: usize) {
    assert!(plaintext.len() <= key.len());
    assert!(plaintext.len() >= count);

    // Encrypt once by xor'ing plaintext with key
    for (p, k) in plaintext.iter_mut().take(count).zip(key.iter()) {
        *p ^= *k;
    }

    // Xor again for extra security
    for (p, k) in plaintext.iter_mut().take(count).zip(key.iter()) {
        *p ^= *k;
    }
}

fn crypt_triple_double_xor_in_place(plaintext: &mut Vec<u8>, key: &Vec<u8>, count: usize) {
    crypt_double_xor_in_place(plaintext, key, count);
    crypt_double_xor_in_place(plaintext, key, count);
    crypt_double_xor_in_place(plaintext, key, count);
}

fn read_and_crypt(input_path: &str, output_path: &str, block_size: usize, crypt_func: &mut FnMut(&mut Vec<u8>, &Vec<u8>, usize)) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    let mut buf = vec![0u8; block_size];
    let key = vec![0x55u8; block_size]; // TODO: Use better key

    loop {
        let count = input_file.read(&mut buf)?;
        if count == 0 {
            break;
        }
        crypt_func(&mut buf, &key, count);
        output_file.write_all(&buf[..count])?;
    }

    Ok(())
}
