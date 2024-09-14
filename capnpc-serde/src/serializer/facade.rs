use capnp::schema_capnp::node;
use capnp::serialize;
use capnpc::codegen::GeneratorContext;
use log::{debug, error};
use std::fs;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::annotation_decl_node::serialize_annotation_decl;
use crate::serializer::const_node::serialize_const;
use crate::serializer::enum_node::serialize_enum;
use crate::serializer::file_node::serialize_file;
use crate::serializer::interface_node::serialize_interface;
use crate::serializer::struct_node::serialize_struct;
use crate::serializer::util::*;

// TODO
static mut g_no_standard_import: bool = false;
static mut g_import_paths: Vec<PathBuf> = Vec::new();
static mut g_src_prefixes: Vec<PathBuf> = Vec::new();


pub fn serialize(
    cache: &mut cache::NodeCache,
    no_standard_import: bool,
    import_paths: &Vec<PathBuf>,
    src_prefixes: &Vec<PathBuf>,
    file: &PathBuf,
) -> CapSerResult<serde_json::Value> {

    unsafe {
        g_no_standard_import = no_standard_import;
        g_import_paths = import_paths.clone();
        g_src_prefixes = src_prefixes.clone();
    }

    debug!("{}:{} serialize called", file!(), line!());
    let abs_file_path = fs::canonicalize(file)?;
    let stdout = run_capnp(no_standard_import, import_paths, src_prefixes, file);
    let message = serialize::read_message(
        ReadWrapper { inner: stdout },
        capnp::message::ReaderOptions::new(),
    )?;
    let ctx: GeneratorContext = GeneratorContext::new(&message)?;

    // This function only receives one file to parse, so get_requested_files() returns only one element.
    let requested_files = ctx.request.get_requested_files()?;
    if requested_files.is_empty() {
        error!("{}:{} there are no requested files", file!(), line!());
        return Err(CapSerError::new("failed to load requested files"));
    }
    let requested_file = requested_files.get(0);

    let file = serialize_node(cache, &ctx, requested_file.get_id(), &abs_file_path)?;
    Ok(serde_json::to_value(file)?)
}

pub fn serialize_node(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    node_id: u64,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    if let Some(value) = cache.get_node(&node_id) {
        return Ok(value);
    }

    debug!("{}:{} serialize_node called", file!(), line!());
    let node = ctx.node_map[&node_id];
    unsafe {
        let result = match node.which()? {
            node::File(_) => serialize_file(g_no_standard_import, &g_import_paths, &g_src_prefixes, cache, ctx, node.get_id(), abs_file_path),
            node::Struct(_) => serialize_struct(cache, ctx, node.get_id(), abs_file_path),
            node::Interface(_) => serialize_interface(cache, ctx, node.get_id(), abs_file_path),
            node::Const(_) => serialize_const(cache, ctx, node.get_id(), abs_file_path),
            node::Enum(_) => serialize_enum(cache, ctx, node.get_id(), abs_file_path),
            node::Annotation(_) => serialize_annotation_decl(cache, ctx, node.get_id(), abs_file_path),
        }?;
        cache.register_node(&result);
        Ok(result)
    }
}

fn run_capnp(
    no_standard_import: bool,
    import_paths: &Vec<PathBuf>,
    src_prefixes: &Vec<PathBuf>,
    file: &PathBuf,
) -> std::process::ChildStdout {
    debug!("{}:{} run_capnp called", file!(), line!());
    let mut command = ::std::process::Command::new("capnp");
    command.env_remove("PWD");
    command.arg("compile").arg("-o").arg("-");
    if no_standard_import {
        command.arg("--no-standard-import");
    }

    for import_path in import_paths {
        command.arg(&format!("--import-path={}", import_path.display()));
    }

    for src_prefix in src_prefixes {
        command.arg(&format!("--src-prefix={}", src_prefix.display()));
    }

    command.arg(file);
    command.stdout(::std::process::Stdio::piped());
    command.stderr(::std::process::Stdio::inherit());

    let mut p = command.spawn().unwrap();
    p.stdout.take().unwrap()
}
