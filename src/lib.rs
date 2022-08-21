pub mod model;
pub mod serializer;
mod utils;
pub mod viewset;

use crate::model::generate_model_code;
use crate::serializer::{generate_model_serializer_code, generate_serializer_code};
use clap::{Parser, ValueEnum};

/// Run Django Code Generator
pub fn run_generator() -> Result<String, String> {
    let args = Args::parse();
    match args.component {
        Component::Model => generate_model_code(&args.name, &args.fields),
        Component::Serializer => {
            if args.model {
                generate_model_serializer_code(&args.name, &args.fields)
            } else {
                generate_serializer_code(&args.name, &args.fields)
            }
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(arg_enum, value_parser)]
    component: Component,
    #[clap(value_parser)]
    name: String,
    #[clap(value_parser)]
    fields: Vec<String>,
    #[clap(long, action)]
    model: bool,
}

#[derive(Clone, ValueEnum)]
enum Component {
    Model,
    Serializer,
}
