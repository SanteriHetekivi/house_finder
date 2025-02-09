mod app;
mod cache;
mod client;
mod elisa;
mod etuovi;
mod open_route_service;
mod telegram;

#[tokio::main]
async fn main() -> std::result::Result<(), self::app::Error> {
    let args: self::app::Args = <self::app::Args as clap::Parser>::parse();
    let telegram: std::option::Option<telegram::Telegram> = match args.telegram_bot_token.clone() {
        Some(telegram_bot_token) => match args.telegram_user_id.clone() {
            Some(telegram_user_id) => Some(telegram::Telegram::new(
                &telegram_bot_token,
                telegram_user_id,
            )),
            // Should not happen if clap is configured and working correctly.
            None => std::panic!("--telegram-bot-token was given but not --telegram-user-id."),
        },
        None => {
            if args.telegram_user_id.is_some() {
                // Should not happen if clap is configured and working correctly.
                std::panic!("--telegram-user-id was given but not --telegram-bot-token.");
            }
            None
        }
    };
    match self::app::run(args, telegram.clone()).await {
        Ok(count) => {
            let message: std::string::String = format!("Found {}!", count);
            println!("{}", message);
            if let Some(telegram) = &telegram {
                telegram.send_message(&message).await?;
            }
            Ok(())
        }
        Err(error) => {
            let message: std::string::String = format!("Got error: {:?}", error);
            eprintln!("{}", message);
            eprintln!("Backtrace: {:?}", std::backtrace::Backtrace::capture());
            if let Some(telegram) = &telegram {
                telegram.send_message(&message).await?;
            }
            Err(error)
        }
    }
}
