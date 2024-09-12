use capnp::schema_capnp::node;
use capnpc::codegen::GeneratorContext;
use log::{debug, warn};
use std::fmt;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::method::*;
use crate::serializer::util::*;

use serde::Serialize;
use serde_json;

pub fn serialize_interface(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_interface called", file!(), line!());
    if !cache.start_parse_node(ctx, id)? {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = InterfaceNode::new(cache, ctx, id, abs_file_path)?;
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct InterfaceNode {
    #[serde(flatten)]
    common_node: CommonNode,
    brands: Vec<serde_json::Value>,
    methods: Vec<serde_json::Value>,
}

impl InterfaceNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let common_node = CommonNode::new(cache, ctx, String::from("Interface"), id, abs_file_path);

        let node = ctx.node_map[&id];
        let node::Interface(interface) = node.which()? else {
            warn!("{}:{} failed to read as interface", file!(), line!());
            return Err(CapSerError::new("This is not interface node."));
        };

        let mut brands = Vec::new(); 
        if let Some(resolved_brands) = cache.resolve_brands(id){
            for serialized_brand in resolved_brands.iter() {
                brands.push(serialized_brand.clone());
            }
        }

        let methods: Vec<serde_json::Value> = interface
            .get_methods()?
            .into_iter()
            .filter_map(|x| serialize_method(cache, ctx, x, abs_file_path).ok())
            .collect();

        Ok(InterfaceNode {
            common_node,
            brands,
            methods,
        })
    }
}

impl fmt::Display for InterfaceNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_nodes_fmt = String::from("");
        for nested_node_id in &self.common_node.nested_nodes {
            nested_nodes_fmt += format!("{}, ", nested_node_id).as_str();
        }
        write!(
            f,
            "InterfaceNode(id: {}, parent_id: {}, nested_nodes: {})",
            self.common_node.id, self.common_node.parent_id, nested_nodes_fmt
        )
    }
}
