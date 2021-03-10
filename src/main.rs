use teloxide::utils::command::BotCommand;

#[derive(BotCommand, Debug, PartialEq)]
#[command(rename = "lowercase", description = "Food Bot commands.")]
enum Command {
    #[command(description = "creates room.")]
    CreateRoom,
    #[command(description = "subscribe room.", parse_with = "split")]
    Subscribe { id: u64 },
    #[command(description = "unsubscribe room.", parse_with = "split")]
    Unsubscribe { id: u64 },

    #[command(description = "wallet status.")]
    Wallet,
    #[command(description = "send request to subscribers.")]
    WalletCry,

    #[command(
        description = "open GB. Enter name of the store and expiration estimate.",
        parse_with = "split"
    )]
    OpenGroupBye { store: String, expiration: String },
    #[command(description = "close GB. Enter GB identifier.", parse_with = "split")]
    CloseGroupBye { id: u64 },
    #[command(description = "send GB bills.", parse_with = "split")]
    SendBills { id: u64, info: String },

    #[command(
        description = "start groupbuy. Enter room identifier and groupbuy name.",
        parse_with = "split"
    )]
    Start { id: u64, name: String },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Food bot starting...");
}
