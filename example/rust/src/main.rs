use capnpc_serde::*;
use std::path::PathBuf;
use serde_json;
use clap::Parser;

use tera::Tera;
use tera::Context;
use std::collections::HashMap;
use std::collections::HashSet;
use tera::Value;
use std::sync::Arc;

#[derive(Parser, Debug)]
struct Args {
    /// the path to the target capn'proto schema file
    target_file_path: String,
    /// whether to output to file. The default value is None, and does not output as a file.
    #[arg(short, long, default_value=None)]
    output_file_path: Option<String>,
    /// whether to import the standard path("/usr/local/include" and "/usr/include") or not
    #[arg(short, long, default_value_t = false)]
    no_standard_import: bool,
    /// paths to the capn' proto schema files you want to import from the target schema
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    import_paths: Vec<String>,
    /// prefixes of the schema file
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    src_prefixes: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    plantuml: bool,
}

static mut G_PLANTUML: bool = false;

fn main() {
    let args = Args::parse();
    //let args: Vec<String> = env::args().collect();
    //if args.len() == 1 {
    //    println!("This sample needs target schema file path as an argument.");
    //    return;
    //}
    unsafe {
        G_PLANTUML = args.plantuml;
    }
    let target_file_path = &args.target_file_path;
    let import_pathes: Vec<PathBuf> = args.import_paths.iter().map(|x| PathBuf::from(x)).collect();
    let _a: Vec<PathBuf> = args.src_prefixes.iter().map(|x| PathBuf::from(x)).collect();
    let serialized: serde_json::Value = serialize_cap(target_file_path, args.no_standard_import, &import_pathes, &_a);
    render_file(serialized);
}

fn load_tera() -> Option<Tera> {
    let mut system_tmp_path = "/usr/local/bin/templates/**";
    unsafe {
        if G_PLANTUML {
            system_tmp_path = "/usr/local/bin/templates-pu/**";
        }
    }
    let mut local_tmp_path = "./templates/**";
    unsafe {
        if G_PLANTUML {
            local_tmp_path = "./templates-pu/**";
        }
    }

    let renderer: Option<Tera> = match Tera::new(local_tmp_path) {
        Ok(t) => Some(t),
        Err(_) => None,
    };

    if let Some(a) = renderer {
        if a.get_template_names().count() > 0{
            return Some(a);
        }
    }

    let renderer: Option<Tera> = match Tera::new(system_tmp_path) {
        Ok(t) => Some(t),
        Err(_) => None,
    };

    if let Some(a) = renderer {
        if a.get_template_names().count() > 0{
            return Some(a);
        }
    }

    println!("templates not found.");
    return None;

}

pub fn render_file(serialized: serde_json::Value) {
    let renderer = load_tera();
    if renderer.is_none() {
        println!("templates not found.");
        return;
    }
    let mut renderer = renderer.unwrap();

    let root_id = serialized.as_object().unwrap().get("root_id").unwrap().as_str().unwrap();
    let id_map = serialized.as_object().unwrap().get("id_map").unwrap(); 
    let root = id_map.get(root_id).unwrap();

    let mut context: Context = Context::new();
    context.insert("file_node", &root);
    context.insert("id_map", &id_map);
    let arc_cache = Arc::new(id_map.clone());
    let arc_root = Arc::new(root.clone());
    renderer.register_function("render_node", render_node(Arc::clone(&arc_cache), Arc::clone(&arc_root)));
    renderer.register_function("render_relationship", render_relationship(Arc::clone(&arc_cache), Arc::clone(&arc_root)));

    //let result = renderer.render("file.tmpl", &context).unwrap();
    let result = renderer.render("all.tmpl", &context).unwrap();

    println!("{}", result);
}

fn render_relationship(cache: Arc<serde_json::Value>, root_value: Arc<serde_json::Value>) -> impl tera::Function{
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let renderer = load_tera();
        let mut renderer = renderer.unwrap();

        let mut context: Context = Context::new();
        renderer.register_function("get_name_by_id", get_name_by_id(Arc::clone(&cache), Arc::clone(&root_value)));
        renderer.register_function("get_complex_type_name", get_complex_type_name(Arc::clone(&cache)));

        let id = &args["id"].as_str().unwrap().to_string();
        let arc_cache = Arc::clone(&cache);
        let node = if let Some(node_) = arc_cache.as_object().unwrap().get(id) {node_.as_object().unwrap()} else { return Ok(tera::to_value("").unwrap());};
        let ids = get_object_related_ids_internal(cache.clone(), node);
        let uniq_ids: HashSet<String> = ids.into_iter().collect();

        context.insert("node", &node);
        context.insert("ids", &uniq_ids);
        let ret = renderer.render("relationship.tmpl", &context).unwrap();
        return Ok(tera::to_value(ret).unwrap());
    })
}

fn render_node(cache: Arc<serde_json::Value>, root_value: Arc<serde_json::Value>) -> impl tera::Function{
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let renderer = load_tera();
        let mut renderer = renderer.unwrap();
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
        let target_id = &args["id"].as_str().unwrap().to_string().replace("\"", "");
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
        let resolve_generics = if let Some(a) = args.get("resolve_generics") { a.as_bool() } else { Some(true) } ;
        let resolve_generics  = if let Some(a)= resolve_generics { a } else { true };
        let name = get_complex_type_name_internal(cache, complex_type, resolve_generics);
        return Ok(tera::to_value(name).unwrap());
    })
}

fn get_complex_type_name_internal(cache: Arc<serde_json::Value>, complex_type: &serde_json::Map<String, Value>, resolve_generics: bool) -> String {
    let mut generics_start = "~";
    let mut generics_end = "~";
    let mut plantuml = false;
    unsafe {
        if G_PLANTUML {
            generics_start = "<";
            generics_end = ">";
            plantuml = true;
        }
    }
    if let Some(list_value) = complex_type.get("List"){
        if let Some(list_obj) = list_value.as_object() {
            let nested_type_name = get_complex_type_name_internal(cache, &list_obj, resolve_generics);
            return format!("List{}{}{}", generics_start, nested_type_name, generics_end);
        } else {
            return format!("List{}{}{}", generics_start, list_value.as_str().unwrap(), generics_end);
        }
    }
    if let Some(enum_value) = complex_type.get("Enum"){
        let name = if let Some(enum_obj) = enum_value.as_object() { 
            let enum_id  = enum_obj.get("id").unwrap().to_string();
            let enum_ = cache.as_object().unwrap().get(&enum_id).unwrap().as_object().unwrap();
            enum_.get("base_name").unwrap().to_string().replace("\"", "")
        } else {
            return String::from("Any");
        };
        return String::from(name);
    }
    if let Some(struct_type) = complex_type.get("Struct"){
        let struct_node = if let Some(struct_obj) = struct_type.as_object() { 
            let struct_id  = struct_obj.get("id").unwrap().to_string();
            cache.as_object().unwrap().get(&struct_id).unwrap().as_object().unwrap()
        } else {
            return String::from("Any");
        };
        let mut name = struct_node.get("base_name").unwrap().to_string().replace("\"", "");
        let brands = if resolve_generics {
            struct_type
                .as_object()
                .unwrap()
                .get("generics")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(
                    |x| 
                    if x.is_object() { get_complex_type_name_internal(cache.clone(), x.as_object().unwrap(), resolve_generics) } else { x.to_string() }
                )
                .collect::<Vec<String>>()
        } else {
            struct_node.get("brands").unwrap().as_array().unwrap().iter().map(|x| x.to_string().replace("\"", "")).collect::<Vec<String>>()
        };
        if brands.is_empty() {} else {
            let mut brands_str_v  = 
                brands
                .iter()
                .fold(String::from(generics_start), |x, y| x + y.as_str() + ",");
            brands_str_v.pop();
            brands_str_v  += generics_end;
            name += brands_str_v.as_str();
        } 

        let name = if plantuml { name } else { format!("`{}`", name) };
        return String::from(name);
    }
    if let Some(interface_type) = complex_type.get("Interface"){
        let interface_node = if let Some(interface_obj) = interface_type.as_object() { 
            let interface_id  = interface_obj.get("id").unwrap().to_string();
            cache.as_object().unwrap().get(&interface_id).unwrap().as_object().unwrap()
        } else {
            return String::from("Any");
        };
        let mut name = interface_node.get("base_name").unwrap().to_string().replace("\"", "");
        let brands = if resolve_generics {
            interface_type
                .as_object()
                .unwrap()
                .get("generics")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(
                    |x| 
                    if x.is_object() { get_complex_type_name_internal(cache.clone(), x.as_object().unwrap(), resolve_generics) } else { x.to_string() }
                )
                .collect::<Vec<String>>()
        } else {
            interface_node.get("brands").unwrap().as_array().unwrap().iter().map(|x| x.to_string().replace("\"", "")).collect::<Vec<String>>()
        };
        if brands.is_empty() {} else {
            let mut brands_str_v  = 
                brands
                .iter()
                .fold(String::from(generics_start), |x, y| x + y.as_str() + ",");
            brands_str_v.pop();
            brands_str_v  += generics_end;
            name += brands_str_v.as_str();
        } 

        return String::from(name);
    }
    return String::from("");
}

fn get_object_related_ids_internal(cache: Arc<serde_json::Value>, node: &serde_json::Map<String, Value>) -> Vec<String> {
    let mut ids = Vec::<String>::new();
    match node.get("node_type").unwrap().as_str().unwrap() {
        "Struct" => {
            for field in node.get("fields").unwrap().as_array().unwrap() {
                ids.append(&mut get_field_related_ids(cache.clone(), field));
            }
            for field in node.get("union_fields").unwrap().as_array().unwrap() {
                ids.append(&mut get_field_related_ids(cache.clone(), field));
            }
        }
        "Interface" => {
            for method in node.get("methods").unwrap().as_array().unwrap() {
                let parameters = method.get("parameters").unwrap().as_object().unwrap();
                for field in parameters.get("fields").unwrap().as_array().unwrap() {
                    ids.append(&mut get_field_related_ids(cache.clone(), field))
                }

                let results = method.get("results").unwrap().as_object().unwrap();
                for field in results.get("fields").unwrap().as_array().unwrap() {
                    ids.append(&mut get_field_related_ids(cache.clone(), field))
                }
            }
        }
        _=>{}
    }
    ids
}

fn get_field_related_ids(cache: Arc<serde_json::Value>, field: &Value) -> Vec<String> {
    let field = field.as_object().unwrap();
    if field.get("field_type").unwrap() == "Group" {
        return vec![field.get("id").unwrap().to_string()];
    }

    if !field.get("type_").unwrap().is_object() {
        return vec![]
    }
    get_type_related_ids(cache, field.get("type_").unwrap())
}

fn get_type_related_ids(cache: Arc<serde_json::Value>, type_: &Value) -> Vec<String> {
    if ! type_.is_object() {
        return vec![];
    }

    let type_ = type_.as_object().unwrap();
    if let Some(a) = type_.get("List"){
        return if a.is_object()  {
            get_type_related_ids(cache.clone(), a)
        } else {
            vec![]
        }
    }

    if let Some(_) = type_.get("AnyPointer"){
        return vec![];
    }

    let mut result = Vec::<String>::new();
    for value in type_.values(){
        for brand in value.get("generics").unwrap().as_array().unwrap() {
            result.append(&mut get_type_related_ids(cache.clone(), brand));
        }
        let id = value.get("id").unwrap().to_string();
        result.push(id);
        break;
    }

    result 
}