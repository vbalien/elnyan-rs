use telegram_bot::*;
use async_trait::async_trait;
use crate::Context;

mod count;
mod select;
mod schoolfood;
mod memo;
mod anitable;
mod command_kind;

pub use count::Count;
pub use select::Select;
pub use schoolfood::Schoolfood;
pub use memo::Memo;
pub use self::anitable::Anitable;
pub use command_kind::CommandKind;

#[async_trait]
pub trait Command {
    fn name(&self) -> Option<&'static str> { None }
    async fn on_command(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait CommandWithCallback {
    fn name(&self) -> Option<&'static str> { None }
    async fn on_command(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn on_callback(&self, ctx: &Context, callback_query: &CallbackQuery) -> Result<(), Box<dyn std::error::Error>>;
}
