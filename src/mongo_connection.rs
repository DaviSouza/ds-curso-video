use dotenv::dotenv;
use mongodb::sync::{Client, Database};
use std::env;

use mongodb::bson;
use thiserror::Error;

pub struct DbConn {
    pub db: Database,
}

impl DbConn {
    pub fn new() -> Self {
        dotenv().ok();
        let mongo_addr = env::var("MONGO_ADDR").expect("MONGO_ADDR must be set");
        let db_name = &env::var("DB_NAME").expect("DB_NAME env var must be set");
        let client = Client::with_uri_str(mongo_addr).unwrap();
        let db = client.database(db_name);
        DbConn { db }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not serialize data: {0}")]
    MongoSerializeBsonError(bson::ser::Error),
}