mod cache;
mod common;
mod error_handler;
mod serializer;
use log::debug;

use crate::serializer::facade::*;
use log::error;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn serialize_cap(
    file_path: &str,
    no_standard_import: bool,
    import_paths: &Vec<PathBuf>,
    src_prefixes: &Vec<PathBuf>,
) -> serde_json::Value {
    debug!("{}:{} serialize_cap called", file!(), line!());
    let file = PathBuf::from(file_path);
    let mut cache = cache::NodeCache {
        node_id_stack: Vec::new(),
        node_map: HashMap::new(),
    };
    let root = serialize(
        &mut cache,
        no_standard_import,
        import_paths,
        src_prefixes,
        &file,
    );
    let mut ret = HashMap::new();
    let Ok(root) = root else {
        error!("{}:{} failed to serialize the target file", file!(), line!());
        return serde_json::to_value("").unwrap();
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
    serde_json::to_value(ret).unwrap()
}
