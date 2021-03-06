use async_trait::async_trait;
use std::time::Duration;
use telegram_bot::*;
use tokio::timer::Interval;

use crate::command::Command;
use crate::command::CommandKind;
use crate::Context;

pub struct Count {}

impl Count {
    pub fn new() -> CommandKind {
        CommandKind::Command(Box::new(Self {}))
    }
}

#[async_trait]
impl Command for Count {
    async fn on_command(
        &self,
        ctx: &Context,
        message: &Message,
        arg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<&str> = arg.trim().split(' ').collect();
        let last_msg = if args.len() <= 1 { "ㄱㄱ" } else { &args[1] };
        let mut counter: u32 = args[0].parse().unwrap_or(5);
        let mut interval = Interval::new_interval(Duration::from_millis(1000));

        while counter != 0 {
            ctx.api
                .send(message.chat.text(format!("{}", counter)))
                .await?;
            interval.next().await;
            counter -= 1;
        }
        ctx.api
            .send(message.chat.text(format!("{}", last_msg)))
            .await?;
        Ok(())
    }
}
