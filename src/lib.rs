mod fields;
mod model;
mod model_serializer;
mod model_viewset;
mod serializer;
mod utils;

use clap::{Parser, ValueEnum};

/// Run Django Code Generator
pub fn run_generator() -> String {
    let args = Args::parse();
    match args.component {
        Component::Model => {
            let model = model::DjangoModel::new(args.name, args.fields);
            model.code()
        }
        Component::Serializer => {
            let serializer = serializer::DRFSerializer::new(args.name, args.fields);
            serializer.code()
        }
        Component::ModelViewset => {
            let viewset = model_viewset::DRFModelViewSet::new(args.name);
            viewset.code()
        }
        Component::ModelSerializer => {
            let serializer = model_serializer::DRFModelSerializer::new(args.name);
            serializer.code()
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
    ModelViewset,
    ModelSerializer,
}
