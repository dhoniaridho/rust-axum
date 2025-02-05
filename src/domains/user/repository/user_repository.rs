use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domains::user::{
        dto::request::GetUserListRequest,
        model::user::{PartialUser, User},
    },
    infrasturcture::database::DB,
    shared::dto::{Meta, Paginated},
};

pub struct UserRepository {
    db: Arc<DB>,
}

impl UserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        UserRepository { db }
    }

    pub fn paginate(&self, q: GetUserListRequest) -> Result<Paginated<User>, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let mut conn = Arc::clone(&self.db.pool).get().unwrap();
        let limit = q.page_size.unwrap_or(1).into();
        let page = q.page.unwrap_or(1);
        let page = if page > 1 { Some(page - 1) } else { None }; // Result handling
        let offset = i64::from(page.unwrap_or(0)) * limit;
        let data = users
            .filter(deleted_at.is_null())
            .offset(i64::from(offset))
            .limit(limit)
            .load::<User>(&mut conn)
            .unwrap();
        let total = users.count().get_result(&mut conn).unwrap_or(0);

        Ok(Paginated::new(
            data,
            Meta {
                page_size: q.page_size.unwrap_or(10),
                page: page.unwrap_or(1),
                total: total,
            },
        ))
    }

    pub fn get(&self, param: Uuid) -> Result<User, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let mut conn = Arc::clone(&self.db.pool).get().unwrap();
        let data = users.filter(id.eq(param)).first::<User>(&mut conn);
        match data {
            Ok(d) => Ok(d),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn delete(&self, param: Uuid) -> Result<Option<User>, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let i = id.eq(param);
        let mut conn = Arc::clone(&self.db.pool).get().unwrap();
        diesel::update(users.filter(i))
            .set(deleted_at.eq(chrono::Utc::now()))
            .execute(&mut conn)
            .unwrap();
        Ok(None)
    }

    pub fn update(&self, param: Uuid, data: PartialUser) -> Result<User, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let i = id.eq(param);

        let mut conn = Arc::clone(&self.db.pool).get().unwrap();
        match diesel::update(users.filter(i))
            .set(&data)
            .execute(&mut conn)
        {
            Ok(_) => Ok(users.filter(i).first::<User>(&mut conn).unwrap()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn create(&self, data: User) -> Result<User, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let mut conn = Arc::clone(&self.db.pool).get().unwrap();
        match diesel::insert_into(users).values(&data).execute(&mut conn) {
            Ok(_) => Ok(data),
            Err(e) => Err(e.to_string()),
        }
    }
}
