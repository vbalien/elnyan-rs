use std::env;
use futures::StreamExt;
use telegram_bot::*;
use mongodb::{Client, ThreadedClient};

mod app;
mod command;
use command::Count;
use command::Select;
use command::Schoolfood;
use command::Memo;
use app::App;

#[derive(Clone)]
pub struct Context {
    api: Api,
    db: Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let context = Context {
        api: Api::new(token),
        db: mongodb::Client::connect("localhost", 27017)
            .expect("Failed to initialize client."),
    };

    let mut stream = context.api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        let mut app = App::new();
        let ctx = context.clone();

        app.add_command("cnt", Box::new(Count{}));
        app.add_command("sel", Box::new(Select{}));
        app.add_command("schoolfood", Box::new(Schoolfood{}));
        app.add_command("_", Box::new(Memo{}));

        if let UpdateKind::Message(message) = update.kind {
            tokio::spawn(async move {
                app.run(&ctx, message).await.unwrap();
            });
        }
    }
    Ok(())
}
