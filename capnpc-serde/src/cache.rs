use serde::Serialize;
use std::collections::HashMap;

use capnpc::codegen::GeneratorContext;
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
}
