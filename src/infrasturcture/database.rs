use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DB {
    pub pool: Arc<DbPool>, // Use Arc to make it shareable in Axum
}

impl DB {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        println!("Connecting to database: {}", database_url);

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create database pool");

        DB {
            pool: Arc::new(pool), // Wrap the pool in Arc for thread safety
        }
    }
}
