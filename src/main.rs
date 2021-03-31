use std::collections::HashMap;


use js_object_derive::JsObject;
use javascript_rs::runtime::ObjectType;


#[derive(JsObject)]
#[object_type(String)]
struct JsString {
    properties: HashMap<String, u32>
}

fn main() {}