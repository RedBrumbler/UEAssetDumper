use super::core_item::CoreItem;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ContentItem {
    pub subs: Vec<CoreItem>,
    pub bytes: Vec<u8>,
    pub cat_type: i32,
    pub cat_name: String
}
