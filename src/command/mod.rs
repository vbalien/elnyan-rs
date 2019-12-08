use telegram_bot::*;
use async_trait::async_trait;

pub mod count;
pub mod select;

#[async_trait]
pub trait Command {
    async fn execute(&self, api: Api, message: &Message, arg: &str) -> Result<(), Error>;
}
