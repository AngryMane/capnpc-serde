use capnp::schema_capnp::value;
use log::debug;
use serde::Serialize;
use std::{collections::HashMap, hash::Hash};

pub fn load_value(value: &value::Reader) -> serde_json::Value {
    debug!("{}:{} load_value called", file!(), line!());
    let mut ret = HashMap::new();
    match value.which().unwrap() {
        value::WhichReader::Void(()) => None,
        value::WhichReader::Bool(value) => {
            ret.insert(LiteralValueEnum::Bool, serde_json::to_value(value).unwrap())
        }
        value::WhichReader::Int8(value) => {
            ret.insert(LiteralValueEnum::Int8, serde_json::to_value(value).unwrap())
        }
        value::WhichReader::Int16(value) => ret.insert(
            LiteralValueEnum::Int16,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Int32(value) => ret.insert(
            LiteralValueEnum::Int32,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Int64(value) => ret.insert(
            LiteralValueEnum::Int64,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Uint8(value) => ret.insert(
            LiteralValueEnum::Uint8,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Uint16(value) => ret.insert(
            LiteralValueEnum::Uint16,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Uint32(value) => ret.insert(
            LiteralValueEnum::Uint32,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Uint64(value) => ret.insert(
            LiteralValueEnum::Uint64,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Float32(value) => ret.insert(
            LiteralValueEnum::Float32,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Float64(value) => ret.insert(
            LiteralValueEnum::Float64,
            serde_json::to_value(value).unwrap(),
        ),
        value::WhichReader::Text(value) => ret.insert(
            LiteralValueEnum::Text,
            serde_json::to_value(value.unwrap()).unwrap(),
        ),
        value::WhichReader::Data(value) => ret.insert(
            LiteralValueEnum::Data,
            serde_json::to_value(value.unwrap()).unwrap(),
        ),
        value::WhichReader::List(_) => {
            // TODO
            //let aaa = value.get_as::<capnp::primitive_list::Reader<bool>>().unwrap();
            //for b in aaa.iter(){
            //    //println!("{}, \n", b);
            //}
            None
        }
        value::WhichReader::Enum(value) => {
            ret.insert(LiteralValueEnum::Enum, serde_json::to_value(value).unwrap())
        }
        value::WhichReader::Struct(_) => {
            // TODO
            //value_unit.has_struct();
            //let aaa = value.get_as::<capnp::schema_capnp::node::struct_::Reader>().unwrap();
            ////let aaa = value.get_as::<capnp::struct_list::Reader<capnp::schema_capnp::field::Owned>>().unwrap();
            //for i in aaa.iter(){
            //    let x = i.get_name().unwrap();
            //    let y = i.get_discriminant_value();
            //}
            None
        }
        value::WhichReader::Interface(()) => ret.insert(
            LiteralValueEnum::Interface,
            serde_json::to_value(0).unwrap(),
        ), // Interface may not be value.
        value::WhichReader::AnyPointer(_) => None,
    };

    serde_json::to_value(ret).unwrap()
}

#[allow(dead_code)]
#[derive(Serialize, std::cmp::PartialEq, std::cmp::Eq, Hash)]
enum LiteralValueEnum {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Text,
    Data,
    List,
    Enum,
    Struct,
    Interface,
    AnyPointer,
}
