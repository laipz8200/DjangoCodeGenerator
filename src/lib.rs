pub mod model;
pub mod serializer;
mod utils;
pub mod viewset;

use clap::{Parser, ValueEnum};

/// Run Django Code Generator
pub fn run_generator() -> Result<String, String> {
    let args = Args::parse();
    match args.component {
        Component::Model => {
            let generator = model::Generator::new();
            generator.generate_model_code(&args.name, &args.fields)
        }
        Component::Serializer => {
            let generator = serializer::Generator::new();
            generator.generate_serializer_code(&args.name, &args.fields)
        }
        Component::ModelSerializer => {
            let generator = serializer::Generator::new();
            generator.generate_model_serializer_code(&args.name, &args.fields)
        }
        Component::Viewset => Err(String::from("not support yet")),
        Component::ModelViewset => {
            let generator = viewset::Generator::new();
            generator.generate_model_viewset_code(&args.name)
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
    Serializer,
    ModelSerializer,
    Viewset,
    ModelViewset,
}
