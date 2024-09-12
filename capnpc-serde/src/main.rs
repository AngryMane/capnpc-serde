mod cache;
mod common;
mod error_handler;
mod serializer;

use clap::Parser;
use log::{debug, error};

use crate::serializer::facade::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// capnpc-serde serializes capn' proto schema file, and output as json to stdo.
#[derive(Parser, Debug)]
struct Args {
    /// the path to the target capn'proto schema file
    target_file_path: String,
    /// whether to output to file. The default value is None, and does not output as a file.
    #[arg(short, long, default_value=None)]
    output_file_path: Option<String>,
    /// whether to import the standard path("/usr/local/include" and "/usr/include") or not
    #[arg(short, long, default_value_t = false)]
    no_standard_import: bool,
    /// paths to the capn' proto schema files you want to import from the target schema
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    import_paths: Vec<String>,
    /// prefixes of the schema file
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    src_prefixes: Vec<String>,
}

pub fn main() {
    debug!("{}:{} main called", file!(), line!());
    env_logger::init();
    let args = Args::parse();
    let ret = __serialize(&args);

    writeln!(
        &mut std::io::stdout(),
        "{}",
        serde_json::to_value(&ret).unwrap()
    )
    .unwrap_or_else(|_| std::process::exit(0));

    let Some(output_file_path) = args.output_file_path else {
        return;
    };
    let output_file_path = PathBuf::from(output_file_path);
    let output_file = File::create(&output_file_path);
    let Ok(mut output_file) = output_file else{
        error!("{}:{} failed to create oupput file: {}", file!(), line!(), output_file_path.to_string_lossy());
        return;
    };
    let _ = write!(output_file, "{}", serde_json::to_value(&ret).unwrap());
}

fn __serialize(args: &Args) -> HashMap<String, serde_json::Value> {
    debug!("{}:{} __serialize called", file!(), line!());
    let file = PathBuf::from(&args.target_file_path);
    let mut cache = cache::NodeCache {
        node_id_stack: Vec::new(),
        node_map: HashMap::new(),
        brand_replace_stack: Vec::new()
    };
    let root = serialize(
        &mut cache,
        args.no_standard_import,
        &args.import_paths.iter().map(PathBuf::from).collect(),
        &args.src_prefixes.iter().map(PathBuf::from).collect(),
        &file,
    );
    let mut ret = HashMap::new();
    let Ok(root) = root else {
        error!("{}:{} failed to serialize the target file", file!(), line!());
        return ret;
    };
    let root_id = root
        .as_object()
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();
    ret.insert(
        String::from("root_id"),
        serde_json::to_value(root_id).unwrap(),
    );
    ret.insert(
        String::from("id_map"),
        serde_json::to_value(cache.node_map).unwrap(),
    );

    ret
}
