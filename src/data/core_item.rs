use super::save_object::SaveObject;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CoreItem {
    pub index: i32,
    pub stats: Vec<SaveObject>,
    pub subs: Vec<CoreItem>,
    pub property_name: i32,
    pub name_override: String,
    pub property_type: i32,
    pub array_type: i32,
    pub sub_struct: i32,
    pub terminate: bool,
    pub trail: bool,
    pub break_: bool,
}

pub fn new_simple(i: i32, name: i32) -> CoreItem {
    CoreItem { 
        index: i,
        property_name: name,
        ..Default::default()
    }
}