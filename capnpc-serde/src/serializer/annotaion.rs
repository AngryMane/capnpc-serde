use capnp::schema_capnp::annotation;
use capnpc::codegen::GeneratorContext;

use crate::error_handler::*;
use crate::serializer::literal_value::*;
use crate::serializer::util::*;

use log::debug;
use serde::Serialize;

pub fn serialize_annotation(
    ctx: &GeneratorContext,
    annotation: annotation::Reader,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_annotation called", file!(), line!());
    let node = ctx.node_map[&annotation.get_id()];
    let value = annotation.get_value()?;
    let value = load_value(&value);
    let (base_name, paths, file_name) = parse_name(&node);
    Ok(serde_json::to_value(Annotation {
        base_name,
        paths,
        file_name,
        value,
    })?)
}

#[derive(Serialize)]
struct Annotation {
    base_name: String,
    paths: Vec<String>,
    file_name: String,
    value: serde_json::Value,
}
