use std::collections::HashMap;
use std::ops::Index;
use crate::Context;
use telegram_bot::*;
use regex::Regex;
use crate::command::CommandKind;

pub type Commands = HashMap<&'static str, CommandKind>;

pub struct App {
    cmds: Commands,
}

impl App {
    pub fn new() -> Self {
        App {
            cmds: HashMap::new(),
        }
    }

    pub async fn message(&self, ctx: &Context, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = message.kind {
            let re = Regex::new(r"/(?P<command>[^@ ]*)@?(?P<botname>\S*)\s?(?P<arg>.*)").unwrap();
            if let Some(cap) = re.captures(data.as_str()) {
                let botname = cap.index("botname");
                let command = cap.index("command");
                let arg = cap.index("arg");

                if (botname.is_empty() || botname == ctx.botname)
                    && self.cmds.contains_key(command)
                {
                    self.cmds.get(command).unwrap().do_command(ctx, &message, arg).await?
                }
                else if self.cmds.contains_key("_")
                    && botname.is_empty()
                    && !self.cmds.contains_key(command)
                {
                    self.cmds.get("_").unwrap().do_command(ctx, &message, command).await?
                }
            }
        };
        Ok(())
    }

    pub async fn callback(&self, ctx: &Context, cbq: CallbackQuery) -> Result<(), Box<dyn std::error::Error>> {
        match &cbq.data {
            Some(data) => match data.as_str() {
                "selfdel" =>  ctx.api.send(&cbq.message.as_ref().unwrap().delete()).await?,
                _ => {
                    for (_, cmd) in self.cmds.iter() {
                        if let Some(name) = cmd.name() {
                            let re = Regex::new(&format!("^{}/", name)).unwrap();
                            if re.is_match(&data) {
                                cmd.do_callback(ctx, &cbq).await?;
                            }
                        }
                    }
                },
            },
            _ => (),
        };
        Ok(())
    }

    pub fn add_command(&mut self, msg: &'static str, command: CommandKind) {
        self.cmds.insert(msg, command);
    }
}
