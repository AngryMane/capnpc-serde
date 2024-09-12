use capnp::schema_capnp::type_;
use capnp::schema_capnp::brand;
use capnpc::codegen::GeneratorContext;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
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
            let serialized_brands = serialize_struct_brands(cache, ctx, struct_type, abs_file_path)?;
            cache.push_struct_brand_record(struct_type, serialized_brands);
            let value = serialize_node(cache, ctx, struct_type.get_type_id(), abs_file_path)?;
            cache.pop_brand_record();
            ret.insert("Struct", value);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::Interface(interface_type) => {
            let mut ret = HashMap::new();
            let serialized_brands = serialize_interface_brands(cache, ctx, interface_type, abs_file_path)?;
            cache.push_interface_brand_record(interface_type, serialized_brands);
            let value: serde_json::Value = serialize_node(cache, ctx, interface_type.get_type_id(), abs_file_path)?;
            cache.pop_brand_record();
            ret.insert("Interface", value);
            Ok(serde_json::to_value(ret)?)
        }
        type_::Which::AnyPointer(a) => {
            match a.which()? {
                type_::any_pointer::Which::Unconstrained(_) => {},
                type_::any_pointer::Which::Parameter(a1) => {
                    if let Some(serialized_brand) = cache.resolve_brand(a1.get_scope_id(), a1.get_parameter_index() as usize){
                        let mut ret = HashMap::new();
                        let node_type = serialized_brand["node_type"].to_string();
                        ret.insert(node_type, serialized_brand);
                        return Ok(serde_json::to_value(ret)?);
                    } 
                },
                type_::any_pointer::Which::ImplicitMethodParameter(_) => {},
            }
            Ok(serde_json::to_value("AnyPointer")?)
        },
    }
}

fn serialize_struct_brands(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    struct_type: &type_::struct_::Reader,
    abs_file_path: &PathBuf,
) -> CapSerResult<Option<Vec<serde_json::Value>>> {
    let generics_struct= struct_type.get_brand()?.get_scopes()?.iter().filter(|x| x.get_scope_id() == struct_type.get_type_id()).next();
    let mut serialized_brands = Vec::new();
    if let Some(a) = generics_struct {
        match a.which()?{
            brand::scope::Which::Bind(binding_list) => {
                for binding in binding_list? {
                    serialized_brands.push(serialize_brand(cache, ctx, binding, &abs_file_path)?);
                }
            }
            brand::scope::Which::Inherit(_) =>{ /* capnpc-serde must not fpollow this path*/ }
        }
    }

    if serialized_brands.is_empty() {
        return Ok(None);
    }
    Ok(Some(serialized_brands))
}

fn serialize_interface_brands(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    interface_type: &type_::interface::Reader,
    abs_file_path: &PathBuf,
) -> CapSerResult<Option<Vec<serde_json::Value>>> {
    let generics_struct= interface_type.get_brand()?.get_scopes()?.iter().filter(|x| x.get_scope_id() == interface_type.get_type_id()).next();
    let mut serialized_brands = Vec::new();
    if let Some(a) = generics_struct {
        match a.which()?{
            brand::scope::Which::Bind(binding_list) => {
                for binding in binding_list? {
                    serialized_brands.push(serialize_brand(cache, ctx, binding, &abs_file_path)?);
                }
            }
            brand::scope::Which::Inherit(_) =>{ /* capnpc-serde must not fpollow this path*/ }
        }
    }

    if serialized_brands.is_empty() {
        return Ok(None);
    }
    Ok(Some(serialized_brands))
}

fn serialize_brand(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    brand: brand::binding::Reader,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    let result = match brand.which()? {
        brand::binding::Unbound(()) => { None }
        brand::binding::Type(t) => {
            Some(serialize_type_internal(cache, ctx, &t?.which()?, abs_file_path)?)
        }
    };

    Ok(result.unwrap())
}