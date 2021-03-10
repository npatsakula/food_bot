fn main() {
    pretty_env_logger::formatted_timed_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Food bot starting...");
}
