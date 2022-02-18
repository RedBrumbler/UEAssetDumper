use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct HeaderItem {
    pub name: String,
    pub code: u32
}

impl HeaderItem {
    pub fn new(name: String, code: u32) -> HeaderItem {
        HeaderItem {
            name,
            code,
        }
    }
}