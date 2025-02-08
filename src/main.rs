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
    let telegram: telegram::Telegram =
        telegram::Telegram::new(&args.telegram_bot_token, args.telegram_user_id);
    match self::app::run(args).await {
        Ok(count) => {
            let message: std::string::String = format!("Found {}!", count);
            println!("{}", message);
            telegram.send_message(&message).await?;
            Ok(())
        }
        Err(error) => {
            let message: std::string::String = format!("Got error: {:?}", error);
            eprintln!("{}", message);
            eprintln!("Backtrace: {:?}", std::backtrace::Backtrace::capture());
            telegram.send_message(&message).await?;
            Err(error)
        }
    }
}
