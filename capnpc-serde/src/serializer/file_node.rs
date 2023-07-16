use log::debug;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use capnp::schema_capnp::code_generator_request;
use capnpc::codegen::GeneratorContext;

use crate::cache;
use crate::common;
use crate::error_handler::*;
use crate::serializer::facade::*;
use crate::serializer::util::*;
use serde::Serialize;
use serde_json;

pub fn serialize_file(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_field called", file!(), line!());
    if !cache.start_parse_node(id) {
        return Ok(serde_json::to_value(id.to_string())?);
    }
    let ret = FileNode::new(cache, ctx, id, abs_file_path)?;
    cache.end_parse_node();
    Ok(serde_json::to_value(ret)?)
}

#[derive(Serialize)]
struct FileNode {
    #[serde(flatten)]
    common_node: CommonNode,
    imported_file: Vec<serde_json::Value>,
}

impl FileNode {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        id: u64,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let common_node = CommonNode::new(cache, ctx, String::from("File"), id, abs_file_path);
        let target_file = ctx
            .request
            .get_requested_files()?
            .into_iter()
            .find(|x| x.get_id() == id)
            .ok_or_else(|| CapSerError::new("failed to find target file from the context."))?;
        let imported_file: Vec<serde_json::Value> = target_file
            .get_imports()?
            .into_iter()
            .filter_map(|x| FileNode::load_imported_file(cache, &x, abs_file_path).ok())
            .collect();
        Ok(FileNode {
            common_node,
            imported_file,
        })
    }

    fn load_imported_file(
        cache: &mut cache::NodeCache,
        imported: &code_generator_request::requested_file::import::Reader,
        abs_file_path: &Path,
    ) -> CapSerResult<serde_json::Value> {
        let relative_imported_file_path = String::from(imported.get_name()?);
        let abs_imported_file_path =
            FileNode::get_abs_imported_file_path(&relative_imported_file_path, abs_file_path)?;
        let import_paths: Vec<PathBuf> = vec![];
        let src_prefixes: Vec<PathBuf> = vec![];
        let serialized = serialize(
            cache,
            false,
            &import_paths,
            &src_prefixes,
            &abs_imported_file_path,
        )?;
        Ok(serialized)
    }

    fn get_abs_imported_file_path(
        relative_imported_file_path: &String,
        abs_file_path: &Path,
    ) -> CapSerResult<PathBuf> {
        // The path of the imported file is relative to one of the following search paths
        // * Execution path for this command
        // * Directory of the file to import from
        // * Standard import path. (/usr/local/include or /usr/include)
        if let Ok(abs_path) = fs::canonicalize(relative_imported_file_path) {
            return Ok(abs_path);
        }

        let mut search_paths: Vec<PathBuf> = common::CONFIG
            .standard_import_paths
            .iter()
            .map(PathBuf::from)
            .collect();
        let current_file_parent_dir = abs_file_path.parent().map(PathBuf::from);
        let current_file_parent_dir = current_file_parent_dir
            .ok_or_else(|| CapSerError::new("failed to get abs path of a imported file."))?;
        search_paths.push(current_file_parent_dir);

        //let relative_imported_file_path = if relative_imported_file_path.starts_with("/") {
        let relative_imported_file_path =
            if let Some(stripped_path) = relative_imported_file_path.strip_prefix('/') {
                stripped_path.to_string()
            } else {
                relative_imported_file_path.clone()
            };

        search_paths
            .into_iter()
            .map(|x| x.join(&relative_imported_file_path))
            .find(|x| x.is_file())
            .ok_or_else(|| CapSerError::new("failed to get abs path of a imported file."))
    }
}

impl fmt::Display for FileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_nodes_fmt = String::from("");
        for nested_node_id in &self.common_node.nested_nodes {
            nested_nodes_fmt += format!("{}, ", nested_node_id).as_str();
        }
        write!(
            f,
            "FileNode(id: {}, parent_id: {}, nested_nodes: {})",
            self.common_node.id, self.common_node.parent_id, nested_nodes_fmt
        )
    }
}
