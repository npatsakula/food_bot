use std::sync::Arc;
use tokio_postgres::Client;

pub mod create_room;
pub use create_room::create_room;

pub mod list_rooms;
pub use list_rooms::list_rooms;

pub mod subscribe_unsubscribe;
pub use subscribe_unsubscribe::{subscribe, unsubscribe};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn initialize(client: Arc<Client>) -> Result<(), tokio_postgres::Error> {
    create_rooms_table(client.clone()).await?;
    create_subscribe_table(client.clone()).await?;

    Ok(())
}

pub async fn create_rooms_table(client: Arc<Client>) -> Result<(), tokio_postgres::Error> {
    client
        .query(
            "CREATE TABLE
             IF NOT EXISTS 
             rooms (
               id bigserial PRIMARY KEY,
               name TEXT NULL,
               owner Int8 NOT NULL,
               UNIQUE (name, owner)
            );",
            &[],
        )
        .await
        .map(|_| ())
}

pub async fn create_subscribe_table(client: Arc<Client>) -> Result<(), tokio_postgres::Error> {
    client
        .query(
            "CREATE TABLE
             IF NOT EXISTS
             subscribers (
               room Int8 references rooms(id),
               subscriber Int8 NOT NULL,
               UNIQUE (room, subscriber)
             );",
            &[],
        )
        .await
        .map(|_| ())
}
