use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(1),
        }
    }
}
