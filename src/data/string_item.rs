use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct StringItem {
    pub string: String
}
impl StringItem {
    pub fn new(string: String) -> StringItem {
        StringItem { 
            string
        }
    }
}