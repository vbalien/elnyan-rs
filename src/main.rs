use std::env;

use futures::StreamExt;
use telegram_bot::*;
use regex::Regex;
use tokio::timer::Interval;
use std::time::Duration;

async fn count_command(api: Api, message: &Message, arg: &str) -> Result<(), Error> {
    let args: Vec<&str> = arg.trim().split(' ').collect();
    let last_msg = if args.len() <= 1 { "ㄱㄱ" } else { &args[1] };
    let mut counter: u32 = args[0].parse().unwrap_or(5);
    let mut interval = Interval::new_interval(Duration::from_millis(1000));

    while counter != 0 {
        api.send(message.chat.text(format!("{}", counter))).await?;
        interval.next().await;
        counter -= 1;
    };
    api.send(message.chat.text(format!("{}", last_msg))).await?;
    Ok(())
}

async fn router(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let re = Regex::new(r"/(?P<command>\w*)@?(?P<botname>\S*)\s?(?P<arg>.*)").unwrap();
        if let Some(cap) = re.captures(data.as_str()) {
            if cap["botname"].len() <= 0 || &cap["botname"] == env::var("BOT_NAME").expect("BOT_NAME not set") {
                match &cap["command"] {
                    "cnt" => count_command(api, &message, &cap["arg"]).await?,
                    _ => (),
                }
            }
        }
    };
    Ok(())
}
 
#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        let internal_api = api.clone();
        if let UpdateKind::Message(message) = update.kind {
            tokio::spawn(async move {
                if let Err(e) = router(internal_api, message).await {
                    println!("{:?}", e)
                }
            });
        }
    }
    Ok(())
}
