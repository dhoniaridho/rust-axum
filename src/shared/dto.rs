use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub page: u32,
    pub page_size: u32,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

impl<T> Paginated<T> {
    pub fn new(data: Vec<T>, meta: Meta) -> Self {
        Paginated { data, meta }
    }
}
