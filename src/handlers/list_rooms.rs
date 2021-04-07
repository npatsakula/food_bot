use std::sync::Arc;
use teloxide::{
    prelude::{AutoSend, Bot, UpdateWithCx},
    types::Message,
};

use tokio_postgres::Client;

pub async fn list_rooms(
    client: Arc<Client>,
    message: UpdateWithCx<AutoSend<Bot>, Message>,
    owner: i64,
) -> Result<(), super::Error> {
    match client
        .query(
            "SELECT id, name FROM rooms WHERE owner = $1 :: Int8;",
            &[&owner],
        )
        .await
    {
        Ok(rows) => {
            let list = rows
                .iter()
                .map(|row| {
                    format!(
                        "ID: {}. Name: {:?}.\n",
                        row.get::<usize, i64>(0),
                        row.get::<usize, Option<&str>>(1)
                    )
                })
                .collect::<String>();

            message.answer(&list).await?;
        }
        Err(err) => {
            log::warn!("Failed to request room list. Reason: '{}'", err);
        }
    }

    Ok(())
}
