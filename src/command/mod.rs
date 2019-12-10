use telegram_bot::*;
use async_trait::async_trait;

pub mod count;
pub mod select;
pub mod schoolfood;

#[async_trait]
pub trait Command {
    async fn execute(&self, api: Api, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
}
