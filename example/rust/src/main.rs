use capnpc_serde::*;
use std::path::PathBuf;
use std::env;
use serde_json;

use tera::Tera;
use tera::Context;
use std::collections::HashMap;
use tera::Value;
use std::sync::Arc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("This sample needs target schema file path as an argument.");
        return;
    }
    let target_file_path = args[1].as_str();

    let mut import_pathes: Vec<PathBuf> = vec![];
    if args.len() == 3 {
        let import_path = args[2].as_str();
        let import_path = PathBuf::from(import_path);
        import_pathes.push(import_path);
    }

    let _a: Vec<PathBuf> = vec![];
    let serialized: serde_json::Value = serialize_cap(target_file_path, false, &import_pathes, &_a);
    render_file(serialized);
}

pub fn render_file(serialized: serde_json::Value) {
    let mut renderer: Tera = match Tera::new("./templates/**") {
        Ok(t) => t,
        Err(e) => {println!{"{}", e};return;},
    };

    let root_id = serialized.as_object().unwrap().get("root_id").unwrap().as_str().unwrap();
    let id_map = serialized.as_object().unwrap().get("id_map").unwrap(); 
    let root = id_map.get(root_id).unwrap();

    let mut context: Context = Context::new();
    context.insert("file_node", &root);
    let arc_cache = Arc::new(id_map.clone());
    let arc_root = Arc::new(root.clone());
    renderer.register_function("render_node", render_node(Arc::clone(&arc_cache), Arc::clone(&arc_root)));
    let result = renderer.render("file.tmpl", &context).unwrap();
    println!("{}", result);
}

fn render_node(cache: Arc<serde_json::Value>, root_value: Arc<serde_json::Value>) -> impl tera::Function{
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let mut renderer: Tera = Tera::new("./templates/**").unwrap();
        let mut context: Context = Context::new();
        renderer.register_function("render_node", render_node(Arc::clone(&cache), Arc::clone(&root_value)));
        renderer.register_function("get_name_by_id", get_name_by_id(Arc::clone(&cache), Arc::clone(&root_value)));
        renderer.register_function("get_complex_type_name", get_complex_type_name(Arc::clone(&cache)));

        let id = &args["id"].as_str().unwrap().to_string();
        let arc_cache = Arc::clone(&cache);
        let node = if let Some(node_) = arc_cache.as_object().unwrap().get(id) {node_.as_object().unwrap()} else { return Ok(tera::to_value("").unwrap());};

        context.insert("struct", &node);
        context.insert("interface", &node);
        context.insert("enum", &node);
        let node_type = node.get("node_type").unwrap().as_str().unwrap();
        let ret = match node_type {
            "Struct" => renderer.render("struct.tmpl", &context).unwrap(),
            "Interface" => renderer.render("interface.tmpl", &context).unwrap(),
            "Enum" => renderer.render("enum.tmpl", &context).unwrap(),
            _ => String::from(""),
        };

        return Ok(tera::to_value(ret).unwrap());
    })
}

fn get_name_by_id(cache: Arc<serde_json::Value>, _: Arc<serde_json::Value>) -> impl tera::Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let target_id = &args["id"].as_str().unwrap().to_string();
        let target_obj = cache.as_object().unwrap().get(target_id);
        if let Some(a) = target_obj {
            let obj = a.as_object().unwrap();
            let result = obj.get("base_name").unwrap().as_str().unwrap();
            return Ok(tera::to_value(result).unwrap());
        } else {
            return Ok(tera::to_value("Not-Found").unwrap());
        }
    })
}

fn get_complex_type_name(cache: Arc<serde_json::Value>) -> impl tera::Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let cache = Arc::clone(&cache);
        let complex_type = args["complex_type"].as_object().unwrap();
        let name = get_complex_type_name_internal(cache, complex_type );
        return Ok(tera::to_value(name).unwrap());
    })
}

fn get_complex_type_name_internal(cache: Arc<serde_json::Value>, complex_type: &serde_json::Map<String, Value>) -> String {
    if let Some(list_value) = complex_type.get("List"){
        if let Some(list_obj) = list_value.as_object() {
            let nested_type_name = get_complex_type_name_internal(cache, &list_obj);
            return format!("List<{}>", nested_type_name);
        } else {
            return format!("List<{}>", list_value.as_str().unwrap());
        }
    }
    if let Some(enum_value) = complex_type.get("Enum"){
        let interface_obj = enum_value.as_object().unwrap();
        let name = interface_obj.get("base_name").unwrap().as_str().unwrap();
        return String::from(name);
    }
    if let Some(struct_value) = complex_type.get("Struct"){
        let struct_obj = if let Some(struct_obj) = struct_value.as_object() {struct_obj} else {
            let struct_id = struct_value.as_str().unwrap().to_string();
            cache.as_object().unwrap().get(&struct_id).unwrap().as_object().unwrap()
        };
        let name = struct_obj.get("base_name").unwrap().as_str().unwrap();
        return String::from(name);
    }
    if let Some(interface_value) = complex_type.get("Interface"){
        let interface_obj = interface_value.as_object().unwrap();
        let name = interface_obj.get("base_name").unwrap().as_str().unwrap();
        return String::from(name);
    }
    return String::from("");
}
