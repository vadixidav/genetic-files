extern crate itertools;
extern crate clap;
extern crate rand;
use itertools::Itertools;
use clap::{App, Arg};
use std::fs::File;
use rand::Rng;
use std::io::{Read, Write};

const DEFAULT_CROSSOVER_POINTS: usize = 3;
const DEFAULT_MUTATION_RATE: f64 = 0.001;
const DEFAULT_UNIT: usize = 1;
const DEFAULT_STRIDE: usize = 1;

fn main() {
    let matches = App::new("matef")
        .version("1.0")
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
                    println!("Error: Cannot accept 0 crossover-points.");
                    return;
                },
                Ok(n) => n,
                Err(e) => {
                    println!("Error: Failed to parse crossover-points: {}", e);
                    return;
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
                    println!("Error: Failed to parse mutation-rate: {}", e);
                    return;
                },
            }
        },
        None => DEFAULT_MUTATION_RATE,
    };

    let mutation_size = match matches.value_of("unit") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(0) => {
                    println!("Error: Cannot accept 0 bytes as the unit.");
                    return;
                },
                Ok(n) => n,
                Err(e) => {
                    println!("Error: Failed to parse unit: {}", e);
                    return;
                },
            }
        },
        None => DEFAULT_UNIT,
    };

    let stride = match matches.value_of("stride") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(0) => {
                    println!("Error: Cannot accept 0 bytes as the stride.");
                    return;
                },
                Ok(n) => n,
                Err(e) => {
                    println!("Error: Failed to parse stride: {}", e);
                    return;
                },
            }
        },
        None => DEFAULT_STRIDE,
    };

    let output = matches.value_of("output").unwrap();
    let filenames = (matches.value_of("file1").unwrap(), matches.value_of("file2").unwrap());

    let files = (
        match File::open(filenames.0) {
            Ok(mut f) => {
                let mut v = Vec::new();
                match f.read_to_end(&mut v) {
                    Ok(_) => {},
                    Err(e) =>  {
                        println!("Could not read file \"{}\": {}", filenames.0, e);
                        return;
                    },
                }
                v
            },
            Err(e) => {
                println!("Could not open file \"{}\": {}", filenames.0, e);
                return;
            },
        }, match File::open(filenames.1) {
            Ok(mut f) => {
                let mut v = Vec::new();
                match f.read_to_end(&mut v) {
                    Ok(_) => {},
                    Err(e) =>  {
                        println!("Could not read file \"{}\": {}", filenames.1, e);
                        return;
                    },
                }
                v
            },
            Err(e) => {
                println!("Could not open file \"{}\": {}", filenames.1, e);
                return;
            },
        },
    );

    let len = std::cmp::min(files.0.len(), files.1.len());

    let mut rng = rand::os::OsRng::new().ok().unwrap();

    //Generate crossover file
    let mut result =
        (0..crossover_points)
        //Map these to random crossover points
        .map(|_| rng.gen_range(0, len))
        //Add total_instructions at the end so we can generate a range with it
        .chain(Some(len))
        //Sort them by value into BTree, which removes duplicates and orders them
        .fold(std::collections::BTreeSet::new(), |mut set, i| {set.insert(i); set})
        //Iterate over the sorted values
        .iter()
        //Turn every copy of two, prepending a 0, into a range
        .scan(0, |prev, x| {let out = Some(*prev..*x); *prev = *x; out})
        //Enumerate by index to differentiate odd and even values
        .enumerate()
        //Map even pairs to ranges in parent 0 and odd ones to ranges in parent 1 and expand the ranges
        .flat_map(|(index, range)| {
            {if index % 2 == 0 {files.0[range].iter()} else {files.1[range].iter()}}.cloned()
        })
        //Collect all the instruction ranges from each parent
        .collect_vec();

    //Mutate result file
    let strides =
        //We can only stride the beginning of a mutation group up to this actual len
        (result.len() - (mutation_size - 1))
        //Divide by stride
        / stride;

    for i in 0..strides {
        if rng.next_f64() < mutation_rate {
            for v in &mut result[(i * stride)..(i * stride + mutation_size)] {
                *v = rng.gen();
            }
        }
    }

    let mut outfile = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not create file \"{}\": {}", output, e);
            return;
        },
    };

    match outfile.write_all(&result[..]) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not write to \"{}\": {}", output, e);
            return;
        },
    }
}
