use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
