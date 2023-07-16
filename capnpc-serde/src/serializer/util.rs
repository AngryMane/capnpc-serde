use crate::cache;
use crate::serializer::annotaion::*;
use crate::serializer::facade::*;
use capnp::schema_capnp::node;
use capnpc::codegen::GeneratorContext;
use std::path::PathBuf;

use serde::Serialize;

pub fn parse_name(node: &node::Reader) -> (String, Vec<String>, String) {
    let name_with_prefix = node.get_display_name().unwrap();
    if !name_with_prefix.contains(':') {
        return (String::from(""), vec![], String::from(name_with_prefix));
    }
    let prefix_length = node.get_display_name_prefix_length();
    let base_name: String = name_with_prefix
        .chars()
        .enumerate()
        .filter(|&(i, _)| i >= prefix_length as usize)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
    let left_name = name_with_prefix
        .chars()
        .enumerate()
        .filter(|&(i, _)| i < prefix_length as usize)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));

    let left_names: Vec<&str> = left_name.split(':').collect();
    let file_name = String::from(left_names[0]);

    let paths__: Vec<&str> = left_names[1].split('.').collect();
    let paths_: Vec<String> = paths__.into_iter().map(String::from).collect();
    let paths: Vec<String> = paths_.into_iter().filter(|x| !x.is_empty()).collect();

    (base_name, paths, file_name)
}

pub struct ReadWrapper<R>
where
    R: std::io::Read,
{
    pub inner: R,
}

impl<R> capnp::io::Read for ReadWrapper<R>
where
    R: std::io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> capnp::Result<usize> {
        loop {
            match std::io::Read::read(&mut self.inner, buf) {
                Ok(n) => return Ok(n),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    return Err(capnp::Error {
                        description: format!("{e}"),
                        kind: capnp::ErrorKind::Failed,
                    })
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct CommonNode {
    pub node_type: String,
    pub id: String,
    pub parent_id: String,
    pub base_name: String,
    pub paths: Vec<String>,
    pub abs_file_path: PathBuf,
    pub nested_nodes: Vec<String>,
    pub annotations: serde_json::Value,
}

impl CommonNode {
    pub fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        node_type: String,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> Self {
        let node: node::Reader = ctx.node_map[&id];
        let parent_id = node.get_scope_id().to_string();
        let (base_name, paths, _) = parse_name(&node);

        let mut nested_nodes: Vec<String> = vec![];
        for nested_node in node.get_nested_nodes().unwrap() {
            let nested_node_id = nested_node.get_id();
            let _ = serialize_node(cache, ctx, nested_node_id, abs_file_path);
            nested_nodes.push(nested_node_id.to_string())
        }

        let annotations: Vec<serde_json::Value> = if let Ok(a) = node.get_annotations() {
            a.iter()
                .filter_map(|x| serialize_annotation(ctx, x).ok())
                .collect()
        } else {
            vec![]
        };
        let annotations =
            serde_json::to_value(annotations).unwrap_or_else(|_| serde_json::to_value("").unwrap());
        CommonNode {
            node_type,
            id: id.to_string(),
            parent_id,
            base_name,
            paths,
            abs_file_path: abs_file_path.clone(),
            nested_nodes,
            annotations,
        }
    }
}
