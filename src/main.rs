use std::env;

use futures::StreamExt;
use telegram_bot::*;
use regex::Regex;
use std::{thread, time};

async fn count_command(api: Api, message: Message, arg: &str) -> Result<(), Error> {
    let args: Vec<&str> = arg.trim().split(' ').collect();
    let mut counter: u32 = match args[0].parse() {
        Ok(counter) => counter,
        Err(_e) => 5
    };
    let last_msg = if args.len() <= 1 { "ㄱㄱ" } else { &args[1] };
    loop {
        if counter == 0 {
            api.send(SendMessage::new(&message.chat, format!("{}", last_msg))).await?;
            break;
        }
        api.send(SendMessage::new(&message.chat, format!("{}", counter))).await?;
        thread::sleep(time::Duration::from_millis(1000));
        counter -= 1;
    };
    Ok(())
}

async fn router(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let re = Regex::new(r"/(?P<command>\w*)@?(?P<botname>\S*)\s?(?P<arg>.*)").unwrap();
            match re.captures(data.as_str()) {
                Some(cap) => {
                    if cap["botname"].len() <= 0 || &cap["botname"] == env::var("BOT_NAME").expect("BOT_NAME not set") {
                        match &cap["command"] {
                            "cnt" => count_command(api, message.clone(), &cap["arg"]).await?,
                            _ => (),
                        }
                    }
                },
                None => (),
            };
        },
        _ => (),
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
        if let UpdateKind::Message(message) = update.kind {
            router(api.clone(), message).await?
        }
    }
    Ok(())
}
