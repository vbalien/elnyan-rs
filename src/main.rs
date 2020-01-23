use futures::StreamExt;
use mongodb::{db::Database, ThreadedClient};
use std::env;
use std::sync::Arc;
use telegram_bot::*;

mod app;
mod command;
use crate::command::*;
use app::App;

#[derive(Clone)]
pub struct Context {
    api: Api,
    db: Database,
    botname: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let context = Context {
        api: Api::new(token),
        db: mongodb::Client::connect("localhost", 27017)
            .expect("Failed to initialize client.")
            .db("elnyan"),
        botname: env::var("BOT_NAME").expect("BOT_NAME not set"),
    };

    let mut app = App::new();
    app.add_command("cnt", Count::new());
    app.add_command("sel", Select::new());
    app.add_command("schoolfood", Schoolfood::new());
    app.add_command("anitable", Anitable::new());
    app.add_command("_", Memo::new());

    let app = Arc::new(app);
    let context = Arc::new(context);
    let mut stream = context.api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        let ctx = context.clone();
        let app = app.clone();

        match update.kind {
            UpdateKind::Message(message) => {
                tokio::spawn(async move {
                    app.message(&ctx, message).await.unwrap();
                });
            }
            UpdateKind::CallbackQuery(callback_query) => {
                tokio::spawn(async move {
                    app.callback(&ctx, callback_query).await.unwrap();
                });
            }
            _ => (),
        }
    }
    Ok(())
}
