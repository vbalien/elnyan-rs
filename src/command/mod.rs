use telegram_bot::*;
use async_trait::async_trait;

mod count;
mod select;
mod schoolfood;

pub use count::Count;
pub use select::Select;
pub use schoolfood::Schoolfood;

#[async_trait]
pub trait Command {
    async fn execute(&self, api: Api, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
}
