use capnp::schema_capnp::field;
use capnp::schema_capnp::node;
use capnpc::codegen::GeneratorContext;
use log::debug;
use log::warn;
use std::fmt;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::field::*;
use crate::serializer::util::*;
use serde::Serialize;
use serde_json;

pub fn serialize_struct(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_struct called", file!(), line!());
    if !cache.start_parse_node(id) {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = StructNode::new(cache, ctx, id, abs_file_path)?;
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct StructNode {
    #[serde(flatten)]
    common_node: CommonNode,
    is_union: bool,
    fields: Vec<serde_json::Value>,
}

impl StructNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let common_node = CommonNode::new(cache, ctx, String::from("Struct"), id, abs_file_path);
        let node: node::Reader = ctx.node_map[&id];

        let node::Struct(struct_) = node.which()? else {
            warn!("{}:{} failed to read as struct", file!(), line!());
            return Err(CapSerError::new("This is not struct node."));
        };

        let fields: Vec<serde_json::Value> = struct_
            .get_fields()?
            .iter()
            .filter_map(|x| StructNode::create_field(cache, ctx, abs_file_path, &x).ok())
            .collect();
        let is_union = struct_
            .get_fields()?
            .iter()
            .find(|x| x.get_discriminant_value() != field::NO_DISCRIMINANT)
            .map_or_else(|| true, |_| false);

        Ok(StructNode {
            common_node,
            is_union,
            fields,
        })
    }

    fn create_field(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        abs_file_path: &PathBuf,
        field: &field::Reader,
    ) -> CapSerResult<serde_json::Value> {
        serialize_field(cache, ctx, field, abs_file_path)
    }
}

impl fmt::Display for StructNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_nodes_fmt = String::from("");
        for nested_node_id in &self.common_node.nested_nodes {
            nested_nodes_fmt += format!("{}, ", nested_node_id).as_str();
        }
        let mut fields_fmt = String::from("");
        for field in &self.fields {
            let a = format!("{}), ", field);
            fields_fmt += &a;
        }

        write!(
            f,
            "StructNode(id: {}, parent_id: {}, nested_nodes: {}, fields: {})",
            self.common_node.id, self.common_node.parent_id, nested_nodes_fmt, fields_fmt
        )
    }
}
