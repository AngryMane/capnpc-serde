use capnp::schema_capnp::method;
use capnpc::codegen::GeneratorContext;
use log::debug;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::annotaion::*;
use crate::serializer::facade::*;

use serde::Serialize;

pub fn serialize_method(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    method: method::Reader,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_method called", file!(), line!());
    let ret = Method::new(cache, ctx, method, abs_file_path)?;
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct Method {
    name: String,
    parameters: serde_json::Value,
    results: serde_json::Value,
    annotations: serde_json::Value,
}

impl Method {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        method: method::Reader,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let name = String::from(method.get_name()?);
        let param_id = method.get_param_struct_type();
        let parameters = serialize_node(cache, ctx, param_id, abs_file_path)?;
        let result_id = method.get_result_struct_type();
        let results = serialize_node(cache, ctx, result_id, abs_file_path)?;
        let annotations: Vec<serde_json::Value> = method
            .get_annotations()?
            .into_iter()
            .filter_map(|x| serialize_annotation(ctx, x).ok())
            .collect();
        let annotations =
            serde_json::to_value(annotations).unwrap_or_else(|_| serde_json::to_value("").unwrap());
        Ok(Method {
            name,
            parameters,
            results,
            annotations,
        })
    }
}
