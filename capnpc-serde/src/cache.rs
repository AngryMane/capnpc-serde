use serde::Serialize;
use std::collections::HashMap;

use capnpc::codegen::GeneratorContext;
use capnp::schema_capnp::type_;
use log::debug;

use crate::error_handler::CapSerResult;

#[derive(Clone, Serialize)]
pub struct BrandRecord {
    node_id: u64,
    serialized_brands: Option<Vec<serde_json::Value>>
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

    pub fn resolve_brands(&self, id: u64) -> Option<Vec<serde_json::Value>> {
        for i in  self.brand_replace_stack.iter().rev(){
            if i.node_id != id {
                continue
            }
            return i.serialized_brands.clone();
        }
        None
    }

    pub fn resolve_brand(&mut self, id: u64, index: usize) -> Option<serde_json::Value> {
        for i in  self.brand_replace_stack.iter().rev(){
            if i.node_id != id {
                continue
            }
            if let Some(parameter_node_ids) = &i.serialized_brands {
                return parameter_node_ids.get(index).cloned();
            }
        }
        None
    }

    pub fn push_interface_brand_record(&mut self, interface_type: &type_::interface::Reader, serialized_brands: Option<Vec<serde_json::Value>>) {
        self.brand_replace_stack.push(BrandRecord  { node_id: interface_type.get_type_id(), serialized_brands });
    }

    pub fn push_struct_brand_record(&mut self, struct_type: &type_::struct_::Reader, serialized_brands: Option<Vec<serde_json::Value>>) {
        self.brand_replace_stack.push(BrandRecord  { node_id: struct_type.get_type_id(), serialized_brands });
    }

    pub fn pop_brand_record(&mut self) {
        self.brand_replace_stack.pop();
    }

}
