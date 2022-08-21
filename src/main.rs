use clap::{Parser, ValueEnum};
use django_code_generator::model::generate_model_code;
use django_code_generator::serializer::{generate_model_serializer_code, generate_serializer_code};

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

fn run_generator() -> Result<String, String> {
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

fn main() {
    let res = run_generator();
    match res {
        Ok(code) => println!("{}", code),
        Err(err) => println!("failed with {}", err),
    }
}
