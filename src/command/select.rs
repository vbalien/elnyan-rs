use async_trait::async_trait;
use telegram_bot::*;

use crate::command::Command;
use crate::command::CommandKind;
use crate::Context;

pub struct Select {}

impl Select {
    pub fn new() -> CommandKind {
        CommandKind::Command(Box::new(Self {}))
    }
}

#[async_trait]
impl Command for Select {
    async fn on_command(
        &self,
        ctx: &Context,
        message: &Message,
        arg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<&str> = arg.trim().split(' ').collect();
        if args.len() == 1 && args[0] == "" {
            return Ok(());
        }
        let selected = rand::random::<usize>() % args.len();
        ctx.api
            .send(message.chat.text(format!("{}", args[selected])))
            .await?;
        Ok(())
    }
}
