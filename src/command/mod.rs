use telegram_bot::*;
use async_trait::async_trait;
use crate::Context;

mod count;
mod select;
mod schoolfood;

pub use count::Count;
pub use select::Select;
pub use schoolfood::Schoolfood;

#[async_trait]
pub trait Command {
    async fn execute(&self, api: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
}
