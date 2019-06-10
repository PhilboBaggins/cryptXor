#[macro_use]
extern crate clap;
use clap::{App, Arg};

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
        .get_matches();

    let input_path = matches.value_of("input-file").unwrap();
    let output_path = matches.value_of("output-file").unwrap();

    let block_size = value_t!(matches, "block-size", usize).unwrap();


    println!("Input file  = {}", input_file);
    println!("Output file = {}", output_file);
}
