use log::warn;
use std::fmt;
use std::path::PathBuf;

use capnp::schema_capnp::node;
use capnpc::codegen::GeneratorContext;

use log::debug;
use serde::Serialize;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::built_in_type::*;
use crate::serializer::literal_value::*;
use crate::serializer::util::*;

pub fn serialize_const(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_const called", file!(), line!());
    if !cache.start_parse_node(id) {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = ConstNode::new(cache, ctx, id, abs_file_path)?;
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct ConstNode {
    #[serde(flatten)]
    common_node: CommonNode,
    type_: serde_json::Value,
    value: serde_json::Value,
}

impl ConstNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let common_node = CommonNode::new(cache, ctx, String::from("Const"), id, abs_file_path);
        let node: node::Reader = ctx.node_map[&id];
        let node::Const(const_) = node.which()? else {
            warn!("{}:{} failed to read as const", file!(), line!());
            return Err(capnp::Error {
                kind: capnp::ErrorKind::Failed,
                description: String::from("Unexpected node type"),
            })?;
        };
        let built_in_type = const_.get_type()?.which()?;
        let type_ = serialize_type(cache, ctx, &built_in_type, abs_file_path)?;
        let value = const_.get_value()?;
        let value = load_value(&value);

        Ok(ConstNode {
            common_node,
            type_,
            value,
        })
    }
}

impl fmt::Display for ConstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConstNode(id: {}, parent_id: {})",
            self.common_node.id, self.common_node.parent_id
        )
    }
}
