mod arguments;
mod yaml_to_json;

extern crate clap;
extern crate yaml_rust;

use arguments::*;
use yaml_to_json::*;

fn main() {
    let args = parse_arguments();

    println!("METHOD: {}", args.method);
    println!("URL: {}", args.url);

    if let Some(yaml) = args.yaml {
        println!("JSON: {}", yaml);

        let docs = yaml_rust::YamlLoader::load_from_str(&yaml).unwrap();
        let doc = &docs[0];

        let mut out = String::new();
        {
            yaml_to_json(doc, &mut out);
        }
        println!("Emit: {}", out);
    }
}
