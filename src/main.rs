use std::env;
use futures::StreamExt;
use telegram_bot::*;

mod app;
mod command;
use command::count::Count;
use command::select::Select;
use command::schoolfood::Schoolfood;
use app::App;
 
#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        let inner_api = api.clone();
        let mut app = App::new();

        app.add_command("cnt", Box::new(Count{}));
        app.add_command("sel", Box::new(Select{}));
        app.add_command("schoolfood", Box::new(Schoolfood{}));

        if let UpdateKind::Message(message) = update.kind {
            tokio::spawn(async move {
                app.run(inner_api, message).await.unwrap();
            });
        }
    }
    Ok(())
}
