use std::collections::HashMap;
use std::env;
use crate::command;
use crate::Context;
use telegram_bot::*;
use regex::Regex;

pub type Commands = HashMap<&'static str, Box<dyn command::Command + Sync + Send>>;

pub struct App {
    cmds: Commands,
}

impl App {
    pub fn new() -> Self {
        App {
            cmds: HashMap::new()
        }
    }

    async fn router(&self, ctx: &Context, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = message.kind {
            let re = Regex::new(r"/(?P<command>\w*)@?(?P<botname>\S*)\s?(?P<arg>.*)").unwrap();
            if let Some(cap) = re.captures(data.as_str()) {
                if cap["botname"].len() <= 0 || &cap["botname"] == env::var("BOT_NAME").expect("BOT_NAME not set") {
                    self.cmds[&cap["command"]].execute(ctx, &message, &cap["arg"]).await?
                }
            }
        };
        Ok(())
    }

    pub async fn run(&self, ctx: &Context, message: Message) -> Result<(), Error> {
        if let Err(e) = self.router(ctx, message).await {
            println!("{:?}", e);
        }
        Ok(())
    }

    pub fn add_command(&mut self, msg: &'static str, command: Box<dyn command::Command + Sync + Send>) {
        self.cmds.insert(msg, command);
    }
}
