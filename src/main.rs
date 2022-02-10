// MIT License
//
// Copyright (c) 2021 Theo M. Bulut, Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod status;
use avro_rs::Schema;
use degauss::compat::{DegaussCheck, DegaussCompatMode};
use degauss::errors::DegaussError;
use degauss::prelude::{Auth, SchemaRegistryClient, SchemaSubjectType, SerdeExt};
use degauss::schema::FromFile;
use degauss::table;
use status::Status;
use std::{panic, path::PathBuf};
use structopt::StructOpt;
use strum::VariantNames;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(StructOpt, Debug)]
#[structopt(name = "degauss",  version = VERSION, author = AUTHORS)]
/// Kafka schema compatibility checker
struct Degauss {
    /// Print the exit status
    #[structopt(short, long, global = true)]
    exit_status: bool,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
/// Options to set during the interaction with compatibility
struct ValidateOpts {
    /// All schemas in chronological order. From oldest to newest.
    #[structopt(short, long, parse(from_os_str))]
    schemas: Vec<PathBuf>,

    /// Compat Mode to check against
    #[structopt(short, long, possible_values = DegaussCompatMode::VARIANTS, case_insensitive = true,)]
    compat: DegaussCompatMode,
}

#[derive(StructOpt, Debug, Clone)]
enum SRCommand {
    /// Register a given schema to kafka schema registry
    Register(RegisterOpts),

    /// Set or Get compatibility for a given topic/subject
    Compatibility(Compatibility),
}

#[derive(StructOpt, Debug, Clone)]
/// Interact with Kafka Schema Registry
struct SchemaRegistry {
    #[structopt(long, env = "DEGAUSS_SCHEMA_REGISTRY_URL")]
    schema_registry_url: String,

    /// Schema registry username
    #[structopt(long, env = "DEGAUSS_SCHEMA_REGISTRY_USER")]
    schema_registry_user: Option<String>,

    /// Schema registry password
    #[structopt(long, env = "DEGAUSS_SCHEMA_REGISTRY_PASS")]
    schema_registry_pass: Option<String>,

    #[structopt(subcommand)]
    cmd: SRCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Validate the compatibility
    Validate(ValidateOpts),

    /// Interact with Schema Registry
    SchemaRegistry(SchemaRegistry),
}

#[derive(StructOpt, Debug, Clone)]
/// Interact with Kafka Schema Registry
struct Compatibility {
    #[structopt(subcommand)]
    cmd: CompatibilityCommand,
}

#[derive(StructOpt, Debug, Clone)]
/// Interact with Kafka Schema Registry
enum CompatibilityCommand {
    /// Get compatibility mode for a given topic/subject on schema registry
    Get(CompatibilityOpts),

    /// Set compatibility for a given topic/subject on schema registry
    Set(CompatibilityOpts),

    /// Check compatibility for a given topic/subject on schema registry
    Check(CheckOpts),
}

#[derive(StructOpt, Debug, Clone)]
/// Interact with Kafka Schema Registry
struct RegisterOpts {
    /// Schema registry username
    #[structopt(long, env = "DEGAUSS_TOPIC")]
    topic: String,

    /// Schema registry password
    #[structopt(long, possible_values = SchemaSubjectType::VARIANTS, env = "DEGAUSS_SUBJECT_TYPE", case_insensitive = true)]
    subject_type: SchemaSubjectType,

    /// Absolute path to the schema file to register
    #[structopt(short, long, parse(from_os_str), env = "DEGAUSS_SCHEMA_PATH")]
    schema_path: PathBuf,
}

/// Check Schema Registry schema compatibility
type CheckOpts = RegisterOpts;

#[derive(StructOpt, Debug, Clone)]
/// Options to set during the interaction with compatibility
struct CompatibilityOpts {
    /// Schema registry topic
    #[structopt(long, env = "DEGAUSS_TOPIC")]
    topic: String,

    /// Schema registry subject type
    #[structopt(long, possible_values = SchemaSubjectType::VARIANTS, env = "DEGAUSS_SUBJECT_TYPE", case_insensitive = true)]
    subject_type: SchemaSubjectType,

    /// Compatibility to set, not needed with Get commands
    #[structopt(short, long, possible_values = DegaussCompatMode::VARIANTS,env = "DEGAUSS_COMPATIBILITY", case_insensitive = true,)]
    compatibility: Option<DegaussCompatMode>,
}

fn process_validate(schemas: Vec<PathBuf>, compatibility: DegaussCompatMode) -> bool {
    let schemas = schemas
        .iter()
        .map(|e| Schema::parse_file(e).unwrap_or_else(|op| panic!("Failed to find file {:#?}", op)))
        .collect::<Vec<Schema>>();

    match (schemas.len(), compatibility) {
        (1, _) => panic!("There is nothing to compare against. Exiting."),
        (2, DegaussCompatMode::Backward | DegaussCompatMode::Forward | DegaussCompatMode::Full) => {
            let dc = DegaussCheck(compatibility);
            let compatibility = dc.tabular_validate(&schemas);
            table::render(&compatibility);
            return compatibility.values().all(|x| *x) ;
        },
        (sl, DegaussCompatMode::BackwardTransitive | DegaussCompatMode::ForwardTransitive | DegaussCompatMode::FullTransitive) if sl >= 2 => {
            let dc = DegaussCheck(compatibility);
            let compatibility = dc.tabular_validate(&schemas);
            table::render(&compatibility);
            return compatibility.values().all(|x| *x) ;
        }
        (a, e) => panic!("Schema count and compatibility check failure. {} compatibility and {} schemas are not comparable.", e, a)
    }
}

fn process_check(client: SchemaRegistryClient, opts: CheckOpts) -> Status {
    let schema = Schema::parse_file(&opts.schema_path).expect("Failed to find path");
    match client.check_compatibility(&schema, &opts.topic, opts.subject_type, true) {
        Ok(compat) => {
            println!("{}", compat.pretty_string());
            if !compat.is_compatible {
                Status::Failure
            } else {
                Status::Success
            }
        }
        Err(e) => {
            println!("{}", e);
            Status::Failure
        }
    }
}

fn process_register(client: SchemaRegistryClient, opts: RegisterOpts) -> Status {
    let schema = Schema::parse_file(opts.schema_path).expect("Error in parsing the schema file");
    match client.register_schema(&schema, &opts.topic, opts.subject_type) {
        Ok(resp) => {
            println!("{}", resp.pretty_string());
            Status::Success
        }
        Err(e) => {
            println!("{}", e);
            Status::Failure
        }
    }
}

fn process_set(client: SchemaRegistryClient, opts: CompatibilityOpts) -> Status {
    match client.set_compatibility(&opts.topic, opts.subject_type, opts.compatibility.unwrap()) {
        Ok(compat) => {
            println!("{}", compat.pretty_string());
            Status::Success
        }
        Err(e) => {
            println!("{}", e);
            Status::Failure
        }
    }
}

fn process_get(client: SchemaRegistryClient, opts: CompatibilityOpts) -> Status {
    match client.get_compatibility(&opts.topic, opts.subject_type) {
        Ok(compat) => {
            println!("{}", compat.pretty_string());
            Status::Success
        }
        Err(e) => {
            let er = match e {
                DegaussError::SrHttp {
                    error_code,
                    message,
                } => serde_json::to_string(&serde_json::json!({
                    "error_code": error_code,
                    "message": message,
                }))
                .unwrap(),
                _ => format!("{}", e),
            };
            println!("{}", er);
            Status::Failure
        }
    }
}

fn create_schema_registry_client(sr: SchemaRegistry) -> SchemaRegistryClient {
    let auth = match (sr.schema_registry_user, sr.schema_registry_pass) {
        (Some(user), Some(pass)) => Auth::Basic {
            username: user,
            password: pass,
        },
        (None, None) => Auth::Skip,
        _ => panic!("Please set both user/pass, not just one"),
    };
    SchemaRegistryClient::new(sr.schema_registry_url, auth)
        .expect("Failed to create a Schema Registry client")
}

fn main() {
    let degauss_cli: Degauss = Degauss::from_args();

    let status = match degauss_cli.cmd {
        SubCommand::Validate(opts) => {
            let valid = process_validate(opts.schemas, opts.compat);
            if !valid {
                Status::Failure
            } else {
                Status::Success
            }
        }

        SubCommand::SchemaRegistry(sr) => {
            let client = create_schema_registry_client(sr.clone());
            match sr.cmd {
                SRCommand::Compatibility(comp) => match comp.cmd {
                    CompatibilityCommand::Get(opts) => process_get(client, opts),
                    CompatibilityCommand::Set(opts) => process_set(client, opts),
                    CompatibilityCommand::Check(opts) => process_check(client, opts),
                },
                SRCommand::Register(opts) => process_register(client, opts),
            }
        }
    };
    if degauss_cli.exit_status {
        std::process::exit(status.to_i32())
    }
}
