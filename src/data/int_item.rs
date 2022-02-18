use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct IntItem {
    pub member: i32,
    pub index: i32
}

impl IntItem {
    pub fn new(member: i32, index: i32) -> IntItem {
        IntItem {
            member, 
            index
        }
    }
}