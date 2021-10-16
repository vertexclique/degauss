mod errors;
mod compat;

use crate::errors::*;
use crate::compat::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Degauss")
        .version("1.0")
        .author("Theo B. <vertexclique@gmail.com>")
        .about("Avro schema compatibility tool")
        .arg(Arg::new("compat")
            .short('c')
            .long("compatibility")
            .about("Given compatibility type to check")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("schemas")
            .short('s')
            .about("List of schemas to check against the compatibility type")
            .required(true)
            .min_values(2))// At least 2 schemas needed to be checked
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .takes_value(true)
            .about("Sets the level of verbosity"))
        .get_matches();

    println!("Hello, world!");
}
