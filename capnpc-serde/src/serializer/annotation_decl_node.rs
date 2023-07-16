use capnpc::codegen::GeneratorContext;
use std::fmt;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::util::*;
use serde::Serialize;

use log::debug;

pub fn serialize_annotation_decl(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_annotation_decl called", file!(), line!());
    if !cache.start_parse_node(id) {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = AnnotationDeclNode::new(cache, ctx, id, abs_file_path);
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct AnnotationDeclNode {
    #[serde(flatten)]
    common_node: CommonNode,
}

impl AnnotationDeclNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> Self {
        let common_node = CommonNode::new(
            cache,
            ctx,
            String::from("AnnotationDecl"),
            id,
            abs_file_path,
        );

        AnnotationDeclNode { common_node }
    }
}

impl fmt::Display for AnnotationDeclNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_nodes_fmt = String::from("");
        for nested_node_id in &self.common_node.nested_nodes {
            nested_nodes_fmt += format!("{}, ", nested_node_id).as_str();
        }
        write!(
            f,
            "AnnotationNode(id: {}, parent_id: {}, nested_nodes: {})",
            self.common_node.id, self.common_node.parent_id, nested_nodes_fmt
        )
    }
}
