// Copyright (c) 2021 bb8-mongodb developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `bb8-mongodb` connection manager

use crate::error::Error;
use async_trait::async_trait;
use bb8::ManageConnection;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};

/// A `bb8` connection manager for the `MongoDB` database
#[derive(Clone, Debug)]
pub struct Mongodb {
    client_options: ClientOptions,
    db_name: String,
}

impl Mongodb {
    /// Create a new `MongodbConnectionManager` given [`mongodb::options::ClientOptions`] and a database name
    pub fn new<T>(client_options: ClientOptions, db_name: T) -> Mongodb
    where
        T: Into<String>,
    {
        Mongodb {
            client_options,
            db_name: db_name.into(),
        }
    }
}

#[async_trait]
impl ManageConnection for Mongodb {
    type Connection = Database;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::with_options(self.client_options.clone())?;
        Ok(client.database(&self.db_name))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _doc = conn.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::Mongodb;
    use anyhow::Result;
    use bb8::Pool;
    use mongodb::options::{ClientOptions, Credential};

    #[tokio::test]
    async fn new_works() -> Result<()> {
        let mut client_options = ClientOptions::parse("mongodb://somedburi:27017").await?;
        client_options.app_name = Some("app".to_string());
        client_options.credential = Some(
            Credential::builder()
                .username(Some("dbuser".to_string()))
                .password(Some("dbpass".to_string()))
                .source(Some("dbauthsource".to_string()))
                .build(),
        );

        // Setup the `bb8-mongodb` connection manager
        let connection_manager = Mongodb::new(client_options, "db");
        // Setup the `bb8` connection pool
        let _pool = Pool::builder().build(connection_manager).await?;
        Ok(())
    }
}
