extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("CryptXor")
        .version("1.0")
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
        .get_matches();

    let (input_file, output_file) =
        (matches.value_of("input-file").unwrap(),
         matches.value_of("output-file").unwrap());

    println!("Input file  = {}", input_file);
    println!("Output file = {}", output_file);
}
