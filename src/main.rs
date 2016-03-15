extern crate itertools;
extern crate clap;
use itertools::Itertools;
use clap::{App, Arg};

const DEFAULT_CROSSOVER_POINTS: usize = 3;
const DEFAULT_MUTATION_RATE: f64 = 0.001;
const DEFAULT_UNIT: usize = 1;
const DEFAULT_STRIDE: usize = 1;

fn main() {
    let matches = App::new("matef")
        .version("0.0")
        .author("Geordon Worley <vadixidav@gmail.com>")
        .about("Mates two files")
        .arg(Arg::with_name("output")
            .help("The output location")
            .required(true)
            .index(1))
        .arg(Arg::with_name("file1")
            .help("Input file 1")
            .required(true)
            .index(2))
        .arg(Arg::with_name("file2")
            .help("Input file 2")
            .required(true)
            .index(3))
        .arg(Arg::with_name("mutation-rate")
            .short("m")
            .multiple(false)
            .long("mutation-rate")
            .value_name("RATE")
            .help("Takes a RATE of mutation that randomizes UNIT bytes at a time")
            .takes_value(true))
        .arg(Arg::with_name("crossover-points")
            .short("c")
            .multiple(false)
            .long("crossover-points")
            .value_name("NUMBER")
            .help("Takes a NUMBER of crossover points of 1 or greater")
            .takes_value(true))
        .arg(Arg::with_name("unit")
            .short("u")
            .multiple(false)
            .long("unit")
            .value_name("BYTES")
            .help("Takes an amount of BYTES that are always mutated as a group")
            .takes_value(true))
        .arg(Arg::with_name("stride")
            .short("s")
            .multiple(false)
            .long("stride")
            .value_name("BYTES")
            .help("Takes an amount of BYTES that define the alignment of mutated units")
            .takes_value(true))
        .get_matches();

    let crossover_points = match matches.value_of("crossover-points") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(0) => {
                    panic!("Error: Cannot accept 0 crossover-points.");
                },
                Ok(n) => n,
                Err(e) => {
                    panic!("Error: Failed to parse crossover-points: {}", e);
                },
            }
        },
        None => DEFAULT_CROSSOVER_POINTS,
    };

    let mutation_rate = match matches.value_of("mutation-rate") {
        Some(c) => {
            match c.parse::<f64>() {
                Ok(n) => n,
                Err(e) => {
                    panic!("Error: Failed to parse mutation-rate: {}", e);
                },
            }
        },
        None => DEFAULT_MUTATION_RATE,
    };

    let mutation_size = match matches.value_of("unit") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(0) => {
                    panic!("Error: Cannot accept 0 bytes as the unit.");
                },
                Ok(n) => n,
                Err(e) => {
                    panic!("Error: Failed to parse unit: {}", e);
                },
            }
        },
        None => DEFAULT_UNIT,
    };

    let stride = match matches.value_of("stride") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(0) => {
                    panic!("Error: Cannot accept 0 bytes as the stride.");
                },
                Ok(n) => n,
                Err(e) => {
                    panic!("Error: Failed to parse stride: {}", e);
                },
            }
        },
        None => DEFAULT_STRIDE,
    };

    let output = matches.value_of("output").unwrap();
    let files = (matches.value_of("file1").unwrap(), matches.value_of("file2").unwrap());
}
