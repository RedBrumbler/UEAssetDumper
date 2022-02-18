use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SaveObject {
    pub index: i32,
    pub property_name: i32,
    pub property_type: i32,
    pub property_desc: String,
    pub value: String,
    pub value_2: String,
    pub value_3: String,
    pub value_4: String,
    pub the_type: i32,
    pub random_num: i32
}