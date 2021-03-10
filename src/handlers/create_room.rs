use std::sync::Arc;
use teloxide::{
    prelude::{AutoSend, Bot, UpdateWithCx},
    types::Message,
};
use tokio_postgres::Client;

pub async fn create_room(
    name: &str,
    client: Arc<Client>,
    message: UpdateWithCx<AutoSend<Bot>, Message>,
    owner: i64,
) -> Result<(), super::Error> {
    let query = if name.is_empty() {
        client
            .query(
                "INSERT INTO rooms (name, owner) VALUES (NULL, $1 :: Int8) RETURNING id;",
                &[&owner],
            )
            .await
    } else {
        client
            .query(
                "INSERT INTO rooms (name, owner) VALUES ($1 :: TEXT, $2 :: Int8) RETURNING id;",
                &[&name, &owner],
            )
            .await
    };

    match query {
        Ok(rows) => {
            let room_id: i64 = rows[0].get(0);

            log::info!("Room '{}' with owner '{}' registered.", room_id, owner);

            let answer = format!("Room created. ID: {}", room_id.to_string());
            message.answer(&answer).await?;
        }
        Err(err) => {
            log::warn!("Failed to register room: '{}'", err);

            let answer = format!("Failed to create room. Reason: '{}'", err);
            message.answer(&answer).await?;
        }
    }

    Ok(())
}
