pub mod error;

use async_trait::async_trait;
use bb8::ManageConnection;
use error::Error;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub struct MongodbConnectionManager {
    client_options: ClientOptions,
    db_name: String,
}

impl MongodbConnectionManager {
    pub fn new<T>(client_options: ClientOptions, db_name: T) -> MongodbConnectionManager
    where
        T: Into<String>,
    {
        MongodbConnectionManager {
            client_options,
            db_name: db_name.into(),
        }
    }
}

#[async_trait]
impl ManageConnection for MongodbConnectionManager {
    type Connection = Database;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::with_options(self.client_options.clone())?;
        Ok(client.database(&self.db_name))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}
