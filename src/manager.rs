// Copyright (c) 2021 bb8-mongodb developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `bb8-mongodb` connection manager

use crate::error::Error;
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

impl ManageConnection for Mongodb {
    type Connection = Database;
    type Error = Error;

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::with_options(self.client_options.clone())?;
        Ok(client.database(&self.db_name))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _doc = conn.run_command(doc! { "ping": 1 }).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Mongodb;
    use anyhow::Result;
    use bb8::Pool;
    use futures::pin_mut;
    use mongodb::{
        bson::doc,
        options::{ClientOptions, Credential},
    };
    use std::{env, time::Duration};
    use tokio::time::timeout;

    #[tokio::test]
    #[allow(clippy::needless_return)]
    async fn new_works() -> Result<()> {
        let mut client_options = ClientOptions::parse(env::var("BB8_MONGODB_URL")?).await?;
        client_options.credential = Some(
            Credential::builder()
                .username(env::var("BB8_MONGODB_USER").ok())
                .password(env::var("BB8_MONGODB_PASSWORD").ok())
                .build(),
        );

        // Setup the `bb8-mongodb` connection manager
        let connection_manager = Mongodb::new(client_options, "admin");
        // Setup the `bb8` connection pool
        let pool = Pool::builder().build(connection_manager).await?;
        // Connect
        let conn = pool.get().await?;
        assert_eq!(conn.name(), "admin");
        // Run a command
        let doc = conn.run_command(doc! { "ping": 1 }).await?;
        // Check the result
        assert_eq!(doc! { "ok": 1 }, doc);
        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::needless_return)]
    async fn bad_connect_errors() -> Result<()> {
        let mut client_options = ClientOptions::parse(env::var("BB8_MONGODB_URL")?).await?;
        client_options.credential = Some(
            Credential::builder()
                .username(env::var("BB8_MONGODB_USER").ok())
                .password(Some("not a password".to_string()))
                .build(),
        );
        client_options.connect_timeout = Some(Duration::from_secs(3));
        client_options.server_selection_timeout = Some(Duration::from_secs(3));

        // Setup the `bb8-mongodb` connection manager
        let connection_manager = Mongodb::new(client_options, "admin");
        // Setup the `bb8` connection pool
        let pool = Pool::builder().build(connection_manager).await?;
        // Connect
        let conn_fut = pool.get();
        pin_mut!(conn_fut);
        assert!(timeout(Duration::from_secs(5), &mut conn_fut)
            .await
            .is_err());
        Ok(())
    }
}
