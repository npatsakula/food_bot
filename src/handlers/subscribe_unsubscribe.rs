use std::sync::Arc;

use teloxide::{
    prelude::{AutoSend, UpdateWithCx},
    types::Message,
    Bot,
};
use tokio_postgres::Client;

pub async fn subscribe(
    client: Arc<Client>,
    message: UpdateWithCx<AutoSend<Bot>, Message>,
    user: i64,
    room: i64,
) -> Result<(), super::Error> {
    let result = client
        .query(
            "INSERT INTO subscribers (user, room) values ($1 :: Int8, $2 :: Int8);",
            &[&user, &room],
        )
        .await;

    match result {
        Ok(_) => {
            message.answer("Successfully subscribed.").await?;
        }
        Err(err) => {
            message
                .answer(format!("Subscription failed: '{}'.", err))
                .await?;
        }
    }

    Ok(())
}

pub async fn unsubscribe(
    client: Arc<Client>,
    message: UpdateWithCx<AutoSend<Bot>, Message>,
    user: i64,
    room: i64,
) -> Result<(), super::Error> {
    let result = client
        .query(
            "DELETE FROM subscribers WHERE user = $1 :: Int8 and room = $2 :: Int8;",
            &[&user, &room],
        )
        .await;

    match result {
        Ok(_) => {
            message.answer("Successfully unsubscription.").await?;
        }
        Err(err) => {
            message
                .answer(format!("Unsubscription failed: '{}'.", err))
                .await?;
        }
    }

    Ok(())
}
