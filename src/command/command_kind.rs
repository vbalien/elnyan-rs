use crate::command::*;

pub enum CommandKind {
    Command(Box<dyn Command + Sync + Send>),
    CommandWithCallback(Box<dyn CommandWithCallback + Sync + Send>),
}

impl CommandKind {
    pub async fn do_command(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandKind::Command(cmd) => cmd.on_command(ctx, message, arg).await,
            CommandKind::CommandWithCallback(cmd) => cmd.on_command(ctx, message, arg).await,
        }
    }

    pub async fn do_callback(&self, ctx: &Context, cbq: &CallbackQuery) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandKind::CommandWithCallback(cmd) => cmd.on_callback(ctx, cbq).await,
            _ => Ok(()),
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        match self {
            CommandKind::Command(cmd) => cmd.name(),
            CommandKind::CommandWithCallback(cmd) => cmd.name(),
        }
    }
}
