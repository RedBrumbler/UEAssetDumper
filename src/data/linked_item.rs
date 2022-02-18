use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct LinkedItem {
    pub base: i32,
    pub class: i32,
    pub link: i32,
    pub connection: i32,
    pub numeric: i32,
    pub b: i32,
    pub d: i32,
}
impl LinkedItem {
    pub fn new(base: i32, b: i32, class: i32, d: i32, link: i32, connection: i32, numeric: i32) -> LinkedItem {
        LinkedItem {
            base,
            b,
            class,
            d,
            link,
            connection,
            numeric
        }
    }
}