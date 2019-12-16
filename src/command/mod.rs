use telegram_bot::*;
use async_trait::async_trait;
use crate::Context;

mod count;
mod select;
mod schoolfood;
mod memo;
mod anitable;

pub use count::Count;
pub use select::Select;
pub use schoolfood::Schoolfood;
pub use memo::Memo;
pub use self::anitable::Anitable;

#[async_trait]
pub trait Command {
    fn name(&self) -> Option<&'static str> {
        None
    }
    async fn execute(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn callback(&self, _ctx: &Context, _callback_query: &CallbackQuery) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
}
