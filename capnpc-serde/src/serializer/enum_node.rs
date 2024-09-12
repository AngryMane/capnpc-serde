use capnp::schema_capnp::node;
use capnpc::codegen::GeneratorContext;
use log::debug;
use std::fmt;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::util::*;
use serde::Serialize;
use serde_json;

pub fn serialize_enum(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_enum called", file!(), line!());
    if !cache.start_parse_node(ctx, id)? {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = EnumNode::new(cache, ctx, id, abs_file_path)?;
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct EnumNode {
    #[serde(flatten)]
    common_node: CommonNode,
    enum_names: Vec<String>,
}

impl EnumNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let common_node = CommonNode::new(cache, ctx, String::from("Enum"), id, abs_file_path);
        let node: node::Reader = ctx.node_map[&id];
        let node::Enum(enum_node_unit) = node.which()? else {
            return Err(CapSerError::new("This is not enum node"));
        };
        let enum_names: Vec<String> = enum_node_unit
            .get_enumerants()
            .unwrap()
            .into_iter()
            .map(|x| String::from(x.get_name().unwrap()))
            .collect();

        Ok(EnumNode {
            common_node,
            enum_names,
        })
    }
}

impl fmt::Display for EnumNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_nodes_fmt = String::from("");
        for nested_node_id in &self.common_node.nested_nodes {
            nested_nodes_fmt += format!("{}, ", nested_node_id).as_str();
        }
        write!(
            f,
            "EnumNode(id: {}, parent_id: {}, nested_nodes: {})",
            self.common_node.id, self.common_node.parent_id, nested_nodes_fmt
        )
    }
}
