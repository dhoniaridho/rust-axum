use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct GetUserListRequest {
    #[validate(range(min = 1, message = "page must be greater than 0"))]
    pub page: Option<u32>,

    #[validate( range(min = 1, message = "page_size must be greater than 0"))]
    pub page_size: Option<u32>,
}
