use clap::Parser;
use django_code_generator::model::generate_model_code;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    component: String,
    #[clap(value_parser)]
    name: String,
    #[clap(value_parser)]
    fields: Vec<String>,
}

fn run_generator() -> String {
    let args = Args::parse();
    match args.component.as_str() {
        "model" => generate_model_code(&args.name, &args.fields),
        component => panic!("component not support: {}", component),
    }
}

fn main() {
    let code = run_generator();
    print!("{}", code);
}
