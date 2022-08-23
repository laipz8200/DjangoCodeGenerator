mod fields;
mod model;
mod utils;

use clap::{Parser, ValueEnum};
use model::{DjangoModel, Model};

/// Run Django Code Generator
pub fn run_generator() -> Result<String, String> {
    let args = Args::parse();
    match args.component {
        Component::Model => {
            let model = DjangoModel::new(args.name, args.fields);
            Ok(model.code())
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
}

#[derive(Clone, ValueEnum)]
enum Component {
    Model,
}
