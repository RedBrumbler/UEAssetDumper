use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct UmapItem {
    pub unknown: Vec<u8>
}