use crate::Context;
use async_trait::async_trait;
use telegram_bot::*;

mod anitable;
mod command_kind;
mod count;
mod memo;
mod schoolfood;
mod select;

pub use self::anitable::Anitable;
pub use command_kind::CommandKind;
pub use count::Count;
pub use memo::Memo;
pub use schoolfood::Schoolfood;
pub use select::Select;

#[async_trait]
pub trait Command {
    fn name(&self) -> Option<&'static str> {
        None
    }
    async fn on_command(
        &self,
        ctx: &Context,
        message: &Message,
        arg: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait CommandWithCallback {
    fn name(&self) -> Option<&'static str> {
        None
    }
    async fn on_command(
        &self,
        ctx: &Context,
        message: &Message,
        arg: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn on_callback(
        &self,
        ctx: &Context,
        callback_query: &CallbackQuery,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
