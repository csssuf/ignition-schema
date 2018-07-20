extern crate schemafy;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let schema = "schema/ignition.json";
    println!("cargo:rerun-if-changed={}", schema);
    let src = Path::new(schema);

    let mut file = File::open(src).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let output = schemafy::generate(Some("Config"), &input).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("schema.rs");
    let mut out_file = File::create(out_path).unwrap();
    out_file.write_all(output.as_bytes()).unwrap();
}
