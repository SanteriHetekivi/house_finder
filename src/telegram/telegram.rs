/// Telegram Bot for sending messages.
#[derive(Clone)]
pub(crate) struct Telegram {
    bot: teloxide::prelude::Bot,
    user_id: teloxide::prelude::UserId,
}

impl Telegram {
    /// Create new Telegram Bot.
    ///
    /// # Arguments
    /// * `token` - Telegram bot token.
    /// * `user_id` - Telegram user id to send messages to.
    pub(crate) fn new(token: &std::primitive::str, user_id: std::primitive::u64) -> Self {
        Telegram {
            bot: teloxide::prelude::Bot::new(token),
            user_id: teloxide::prelude::UserId(user_id),
        }
    }

    /// Sends a message to user who's id was used in initialization.
    ///
    /// # Arguments
    /// * `message` - Message to send.
    pub(crate) async fn send_message(
        &self,
        message: &std::primitive::str,
    ) -> std::result::Result<teloxide::prelude::Message, teloxide::RequestError> {
        teloxide::requests::Requester::send_message(&self.bot, self.user_id, message).await
    }
}
