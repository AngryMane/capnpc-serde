use serde::Serialize;
use std::collections::HashMap;

use capnpc::codegen::GeneratorContext;
use capnp::schema_capnp::brand;
use capnp::schema_capnp::type_;
use log::debug;

use crate::error_handler::CapSerResult;

#[derive(Clone, Serialize)]
pub struct BrandRecord {
    struct_id: u64,
    parameter_node_ids: Vec<Option<u64>>
}

#[derive(Clone, Serialize)]
pub struct NodeCache {
    pub node_id_stack: Vec<u64>,
    pub node_map: HashMap<String, serde_json::Value>,
    pub brand_replace_stack: Vec<BrandRecord>
}

impl NodeCache {
    pub fn register_node(&mut self, value: &serde_json::Value) {
        let id = if let Some(value) = value.get("id") {
            value
        } else {
            return;
        };
        let id = if let Some(value) = id.as_str() {
            value.to_string()
        } else {
            return;
        };
        self.node_map.insert(id, value.clone());
    }

    pub fn get_node(&self, id: &u64) -> Option<serde_json::Value> {
        if let Some(value) = self.node_map.get(&id.to_string()) {
            debug!("specified id({}) is already serialized", id);
            return Some(value.clone());
        }
        None
    }
        
    pub fn start_parse_node(&mut self, _: &GeneratorContext, id: u64) -> CapSerResult<bool> {
        if self.node_id_stack.contains(&id) {
            return Ok(false);
        }
        self.node_id_stack.push(id);
        Ok(true)
    }

    pub fn end_parse_node(&mut self) {
        self.node_id_stack.pop();
    }

    pub fn resolve_brand(&mut self, id: u64, index: usize) -> Option<u64> {
        for i in  self.brand_replace_stack.iter().rev(){
            if i.struct_id != id {
                continue
            }
            if let Some(id) = i.parameter_node_ids.get(index){
                return id.clone();
            }
        }
        None
    }

    pub fn push_interface_brand_record(&mut self, interface_type: &type_::interface::Reader) -> CapSerResult<()>{
        let generics_struct= interface_type.get_brand()?.get_scopes()?.iter().filter(|x| x.get_scope_id() == interface_type.get_type_id()).next();
        let generics_struct_id = interface_type.get_type_id();
        let mut parameter_node_ids: Vec<Option<u64>> = Vec::new();
        if let Some(a) = generics_struct {
            match a.which()?{
                brand::scope::Which::Bind(binding_list) => {
                    for binding in binding_list? {
                        parameter_node_ids.push(self.get_brand_id(binding)?);
                    }
                }
                brand::scope::Which::Inherit(_) =>{ /* capnpc-serde must not fpollow this path*/ }
            }
        }

        self.brand_replace_stack.push(BrandRecord  { struct_id: generics_struct_id, parameter_node_ids });
        Ok(())
    }

    pub fn push_struct_brand_record(&mut self, struct_type: &type_::struct_::Reader) -> CapSerResult<()>{
        let generics_struct= struct_type.get_brand()?.get_scopes()?.iter().filter(|x| x.get_scope_id() == struct_type.get_type_id()).next();
        let generics_struct_id = struct_type.get_type_id();
        let mut parameter_node_ids: Vec<Option<u64>> = Vec::new();
        if let Some(a) = generics_struct {
            match a.which()?{
                brand::scope::Which::Bind(binding_list) => {
                    for binding in binding_list? {
                        parameter_node_ids.push(self.get_brand_id(binding)?);
                    }
                }
                brand::scope::Which::Inherit(_) =>{ /* capnpc-serde must not fpollow this path*/ }
            }
        }

        self.brand_replace_stack.push(BrandRecord  { struct_id: generics_struct_id, parameter_node_ids });
        Ok(())
    }

    pub fn pop_brand_record(&mut self) {
        self.brand_replace_stack.pop();
    }

    fn get_brand_id(&mut self, brand: brand::binding::Reader) -> CapSerResult<Option<u64>> {
        let result = match brand.which()? {
            brand::binding::Unbound(()) => { None }
            brand::binding::Type(t) => {
                match t?.which()? {
                    type_::Which::Void(_) |
                    type_::Which::Bool(_) |
                    type_::Which::Int8(_) |
                    type_::Which::Int16(_) |
                    type_::Which::Int32(_) |
                    type_::Which::Int64(_) |
                    type_::Which::Uint8(_) |
                    type_::Which::Uint16(_) |
                    type_::Which::Uint32(_) |
                    type_::Which::Uint64(_) |
                    type_::Which::Float32(_) |
                    type_::Which::Float64(_) |
                    type_::Which::Text(_) |
                    type_::Which::Data(_) | 
                    type_::Which::List(_) =>  None, /* capnpc-serde must not fpollow this path*/ 
                    type_::Which::Enum(enum_brand) => Some(enum_brand.get_type_id()),
                    type_::Which::Struct(struct_brand) => Some(struct_brand.get_type_id()),
                    type_::Which::Interface(interface_brand) => Some(interface_brand.get_type_id()),
                    type_::Which::AnyPointer(_) => None
                }
            }
        };

        Ok(result)
    }

}
