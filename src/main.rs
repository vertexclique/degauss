mod compat;
mod errors;
mod schema;
mod table;

use avro_rs::Schema;
use compat::DegaussCompatMode;
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

    /// All schemas in chronological order
    #[structopt(short, long, parse(from_os_str))]
    schemas: Vec<PathBuf>,

    /// Compat Mode to check against
    #[structopt(short, long, parse(from_os_str))]
    compat: DegaussCompatMode,

    /// Print the exit status
    #[structopt(short, long)]
    exit_status: bool,
}

fn main() {
    let matches: Degauss = Degauss::from_args();

    let schemas = matches.schemas
        .iter()
        .map(|e| Schema::parse_file(e)
            .unwrap_or_else(|op| panic!("Failed to find file {:#?}", op)))
        .collect::<Vec<Schema>>();

    
    // let old = Schema::parse_file(&matches.schemas)
    //     .unwrap_or_else(|_| panic!("Failed to find file {:?}", &matches.old));

    // let new = Schema::parse_file(&matches.new)
    //     .unwrap_or_else(|_| panic!("Failed to find file {:?}", &matches.new));

    // let compatibility = DegaussCheck::validate_all(&new, &old);
    // table::render(&compatibility);

    // if matches.exit_status {
    //     if !compatibility.values().all(|x| *x) {
    //         std::process::exit(1);
    //     } else {
    //         std::process::exit(0);
    //     }
    // }
}
