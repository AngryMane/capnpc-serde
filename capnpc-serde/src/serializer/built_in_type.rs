use capnp::schema_capnp::type_;
use capnpc::codegen::GeneratorContext;
use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::facade::*;

pub fn serialize_type(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    type_: &type_::WhichReader,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_type called", file!(), line!());
    serialize_type_internal(cache, ctx, type_, abs_file_path)
}

fn serialize_type_internal(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    type_: &type_::WhichReader,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    // TODO: use macro
    match type_ {
        type_::Which::Void(_) => Ok(serde_json::to_value("Void")?),
        type_::Which::Bool(_) => Ok(serde_json::to_value("Bool")?),
        type_::Which::Int8(_) => Ok(serde_json::to_value("Int8")?),
        type_::Which::Int16(_) => Ok(serde_json::to_value("Int16")?),
        type_::Which::Int32(_) => Ok(serde_json::to_value("Int32")?),
        type_::Which::Int64(_) => Ok(serde_json::to_value("Int64")?),
        type_::Which::Uint8(_) => Ok(serde_json::to_value("Uint8")?),
        type_::Which::Uint16(_) => Ok(serde_json::to_value("Uint16")?),
        type_::Which::Uint32(_) => Ok(serde_json::to_value("Uint32")?),
        type_::Which::Uint64(_) => Ok(serde_json::to_value("Uint64")?),
        type_::Which::Float32(_) => Ok(serde_json::to_value("Float32")?),
        type_::Which::Float64(_) => Ok(serde_json::to_value("Float64")?),
        type_::Which::Text(_) => Ok(serde_json::to_value("Text")?),
        type_::Which::Data(_) => Ok(serde_json::to_value("Data")?),
        type_::Which::List(list_var) => {
            let mut ret = HashMap::new();
            let nested_type = serialize_type(
                cache,
                ctx,
                &list_var.get_element_type()?.which()?,
                abs_file_path,
            )?;
            ret.insert("List", nested_type);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::Enum(enum_var) => {
            let mut ret = HashMap::new();
            let value = serialize_node(cache, ctx, enum_var.get_type_id(), abs_file_path)?;
            ret.insert("Enum", value);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::Struct(struct_type) => {
            let mut ret = HashMap::new();
            cache.push_brand_record(struct_type)?;
            let value = serialize_node(cache, ctx, struct_type.get_type_id(), abs_file_path)?;
            cache.pop_brand_record();
            ret.insert("Struct", value);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::Interface(interface_var) => {
            let mut ret = HashMap::new();
            let value: serde_json::Value = serialize_node(cache, ctx, interface_var.get_type_id(), abs_file_path)?;
            ret.insert("Interface", value);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::AnyPointer(a) => {
            match a.which()? {
                type_::any_pointer::Which::Unconstrained(_) => {},
                type_::any_pointer::Which::Parameter(a1) => {
                    if let Some(node_id) = cache.resolve_brand(a1.get_scope_id(), a1.get_parameter_index() as usize){
                        let mut ret = HashMap::new();
                        let value = serialize_node(cache, ctx, node_id, abs_file_path)?;
                        ret.insert("Struct", value);
                        return Ok(serde_json::to_value(ret)?);
                    } 
                },
                type_::any_pointer::Which::ImplicitMethodParameter(_) => {},
            }
            Ok(serde_json::to_value("AnyPointer")?)
        },
    }

}
