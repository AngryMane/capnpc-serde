use capnp::schema_capnp::field;
use capnp::schema_capnp::type_;
use capnpc::codegen::GeneratorContext;
use log::debug;
use std::fmt;
use std::path::PathBuf;

use crate::cache;
use crate::error_handler::*;
use crate::serializer::annotaion::*;
use crate::serializer::built_in_type::*;
use crate::serializer::facade::*;
use crate::serializer::literal_value::*;

use serde::Serialize;

pub fn serialize_field(
    cache: &mut cache::NodeCache,
    ctx: &GeneratorContext,
    field: &field::Reader,
    abs_file_path: &PathBuf,
) -> CapSerResult<serde_json::Value> {
    debug!("{}:{} serialize_field called", file!(), line!());
    let name = field.get_name()?.to_string();

    let annotations: Vec<serde_json::Value> = if let Ok(anns_) = field.get_annotations() {
        anns_
            .iter()
            .filter_map(|x| serialize_annotation(ctx, x).ok())
            .collect()
    } else {
        vec![]
    };
    let annotations =
        serde_json::to_value(annotations).unwrap_or_else(|_| serde_json::to_value("").unwrap());

    match field.which()? {
        field::Which::Group(group) => {
            let ret = Group::new(cache, ctx, name, &group, annotations, abs_file_path)?;
            Ok(serde_json::to_value(ret)?)
        }
        field::Which::Slot(slot) => {
            let ret = Slot::new(cache, ctx, name, &slot, annotations, abs_file_path)?;
            Ok(serde_json::to_value(ret)?)
        }
    }
}

#[derive(Serialize)]
struct Group {
    field_type: String,
    name: String,
    type_id: u64,
    #[serde(flatten)]
    fields: serde_json::Value,
}

#[derive(Serialize, Clone)]
struct Slot {
    field_type: String,
    name: String,
    is_interface: bool,
    has_default: bool,
    type_: serde_json::Value,
    default_value: Option<serde_json::Value>,
    annotations: serde_json::Value,
}

impl Slot {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        name: String,
        slot: &field::slot::Reader,
        annotations: serde_json::Value,
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let field_type = String::from("Slot");
        let type_: type_::WhichReader = slot.get_type()?.which()?;
        let type_ = serialize_type(cache, ctx, &type_, abs_file_path)?;

        let mut is_interface = false;
        if let Some(obj) = type_.as_object() {
            is_interface = obj.get_key_value("Interface").is_some();
        }
        let type_: serde_json::Value = serde_json::to_value(type_)?;
        let default_value = slot.get_default_value()?;
        let has_default = slot.has_default_value();
        let default_value = if slot.has_default_value() {
            Some(load_value(&default_value))
        } else {
            None
        };
        Ok(Slot {
            field_type,
            name,
            has_default,
            is_interface,
            type_,
            default_value,
            annotations,
        })
    }
}

impl Group {
    fn new(
        cache: &mut cache::NodeCache,
        ctx: &GeneratorContext,
        name: String,
        group: &field::group::Reader,
        _: serde_json::Value, // TODO
        abs_file_path: &PathBuf,
    ) -> CapSerResult<Self> {
        let field_type = String::from("Group");
        let type_id = group.get_type_id();
        let fields = serialize_node(cache, ctx, type_id, abs_file_path)?;

        Ok(Group {
            field_type,
            name,
            type_id: group.get_type_id(),
            fields,
        })
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group(name: {}, type_id: {})", self.name, self.type_id)
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slot(name: {}, type: TODO)", self.name)
    }
}
