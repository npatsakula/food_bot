use std::sync::Arc;
use teloxide::prelude::RequesterExt;
use teloxide::{
    prelude::{AutoSend, GetChatId, UpdateWithCx},
    types::Message,
    utils::command::BotCommand,
    Bot,
};
use tokio_postgres::Client;

use food_bot::handlers::{self, create_room, list_rooms, subscribe, unsubscribe};

#[derive(BotCommand, Debug, PartialEq)]
#[command(rename = "lowercase", description = "Food Bot commands.")]
enum Command {
    #[command(description = "creates room.")]
    CreateRoom { name: String },
    #[command(description = "list rooms.")]
    ListRooms,
    #[command(description = "subscribe room.", parse_with = "split")]
    Subscribe { id: i64 },
    #[command(description = "unsubscribe room.", parse_with = "split")]
    Unsubscribe { id: i64 },

    #[command(description = "wallet status.")]
    Wallet,
    #[command(description = "send request to subscribers.")]
    WalletCry,

    #[command(
        description = "open GB. Enter name of the store and expiration estimate.",
        parse_with = "split"
    )]
    OpenGroupBuy { store: String, expiration: String },
    #[command(description = "close GB. Enter GB identifier.", parse_with = "split")]
    CloseGroupBuy { id: i64 },
    #[command(description = "send GB bills.", parse_with = "split")]
    SendBills { id: i64, info: String },

    #[command(
        description = "start groupbuy. Enter room identifier and groupbuy name.",
        parse_with = "split"
    )]
    Start { id: u64, name: String },

    #[command(description = "Show this message.")]
    Help,
}

async fn message_handler(
    client: Arc<Client>,
    message: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), handlers::Error> {
    let chat_id = message.chat_id();
    log::info!("'{:?}' command received from user '{}'", command, chat_id,);

    match command {
        // TODO: Create meaningful usage instruction.
        Command::Help => message.answer(Command::descriptions()).await.map(|_| ())?,

        Command::CreateRoom { name } => create_room(&name, client, message, chat_id).await?,
        Command::ListRooms => list_rooms(client, message, chat_id).await?,
        Command::Subscribe { id } => subscribe(client, message, chat_id, id).await?,
        Command::Unsubscribe { id } => unsubscribe(client, message, chat_id, id).await?,

        _ => todo!(),
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    let host = std::env::var("HOST").unwrap();
    let user = std::env::var("USER").unwrap();
    let db_name = std::env::var("DBNAME").unwrap();

    let config = format!(
        "host={host} user={user} dbname={dbname}",
        host = host,
        user = user,
        dbname = db_name,
    );

    let (client, connection) = tokio_postgres::connect(&config, tokio_postgres::NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            log::error!("DB connection error: '{}'", err);
        }
    });

    log::info!("Connection to database established.");

    let client = Arc::new(client);
    handlers::initialize(client.clone()).await.unwrap();

    let answer = move |message, command| {
        let client = Arc::clone(&client);
        async move { message_handler(client, message, command).await }
    };

    let bot = Bot::from_env().auto_send();
    log::info!("Bot started.");

    teloxide::commands_repl(bot, "Food bod.", answer).await;
}
