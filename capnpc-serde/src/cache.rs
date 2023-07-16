use serde::Serialize;
use std::collections::HashMap;

use log::debug;

#[derive(Clone, Serialize)]
pub struct NodeCache {
    pub node_id_stack: Vec<u64>,
    pub node_map: HashMap<String, serde_json::Value>,
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

    pub fn start_parse_node(&mut self, id: u64) -> bool {
        if self.node_id_stack.contains(&id) {
            return false;
        }
        self.node_id_stack.push(id);
        true
    }

    pub fn end_parse_node(&mut self) {
        self.node_id_stack.pop();
    }
}
