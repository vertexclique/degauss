mod compat;
mod errors;
mod schema;
mod table;

use avro_rs::Schema;
use schema::FromFile;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::compat::DegaussCheck;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(StructOpt, Debug)]
#[structopt(name = "degauss",  version = VERSION, author = AUTHORS)]
/// Kafka schema compatibility checker
struct Degauss {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Old schema file
    #[structopt(short, long, parse(from_os_str))]
    old: PathBuf,

    /// New schema file
    #[structopt(short, long, parse(from_os_str))]
    new: PathBuf,

    /// Print the exit status
    #[structopt(short, long)]
    exit_status: bool,
}

fn main() {
    let matches: Degauss = Degauss::from_args();
    let old = Schema::parse_file(&matches.old)
        .unwrap_or_else(|_| panic!("Failed to find file {:?}", &matches.old));

    let new = Schema::parse_file(&matches.new)
        .unwrap_or_else(|_| panic!("Failed to find file {:?}", &matches.new));

    let compatibility = DegaussCheck::validate_all(&new, &old);
    table::render(&compatibility);

    if matches.exit_status {
        if !compatibility.values().all(|x| *x) {
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    }
}
