use telegram_bot::*;
use async_trait::async_trait;

use crate::command::Command;

pub struct Select {}

#[async_trait]
impl Command for Select {
    async fn execute(&self, api: Api, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<&str> = arg.trim().split(' ').collect();
        if args.len() == 1 && args[0] == "" { return Ok(()) }
        let selected = rand::random::<usize>() % args.len();
        api.send(message.chat.text(format!("{}", args[selected]))).await?;
        Ok(())
    }
}
