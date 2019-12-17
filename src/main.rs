extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("xdf-cli")
        .version("1.0")
        .about("CLI tools for reading and converting multiple data formats")
        .arg(
            Arg::with_name("INPUT_FILEPATH")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_FILEPATH")
                .help("Sets the output to converted")
                .short("o")
                .index(2),
        )
        .get_matches();

    let input_filepath = matches.value_of("INPUT_FILEPATH").unwrap();
    println!("Using input file: {}", input_filepath);
    if let Some(output_filepath) = matches.value_of("OUTPUT_FILEPATH") {
        println!("Using output file: {}", output_filepath);
    }
}
