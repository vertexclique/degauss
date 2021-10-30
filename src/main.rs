mod compat;
mod errors;
mod schema;
mod table;

use avro_rs::Schema;
use compat::DegaussCompatMode;
use schema::FromFile;
use std::{panic, path::PathBuf};
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

    /// All schemas in chronological order. From oldest to newest.
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

    let schemas = matches
        .schemas
        .iter()
        .map(|e| Schema::parse_file(e).unwrap_or_else(|op| panic!("Failed to find file {:#?}", op)))
        .collect::<Vec<Schema>>();

    match (schemas.len(), matches.compat.clone()) {
        (1, _) => panic!("There is nothing to compare against. Exiting."),
        (2, DegaussCompatMode::Backwards | DegaussCompatMode::Forwards | DegaussCompatMode::Full) => {
            let dc = DegaussCheck(matches.compat);
            let compatibility = dc.tabular_validate(&schemas);
            table::render(&compatibility);

            if matches.exit_status {
                if !compatibility.values().all(|x| *x) {
                    std::process::exit(1);
                } else {
                    std::process::exit(0);
                }
            }
        },
        (sl, DegaussCompatMode::BackwardsTransitive | DegaussCompatMode::ForwardsTransitive | DegaussCompatMode::FullTransitive) if sl >= 2 => {
            let dc = DegaussCheck(matches.compat);
            let compatibility = dc.tabular_validate(&schemas);
            table::render(&compatibility);

            if matches.exit_status {
                if !compatibility.values().all(|x| *x) {
                    std::process::exit(1);
                } else {
                    std::process::exit(0);
                }
            }
        }
        (a, e) => panic!("Schema count and compatibility check failure. {} compatibility and {} schemas are not comparable.", e, a)
    }
}
