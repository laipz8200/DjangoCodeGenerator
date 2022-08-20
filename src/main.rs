use clap::{Parser, ValueEnum};
use django_code_generator::model::generate_model_code;

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

fn run_generator() -> String {
    let args = Args::parse();
    match args.component {
        Component::Model => generate_model_code(&args.name, &args.fields),
    }
}

fn main() {
    let code = run_generator();
    print!("{}", code);
}
