use anitable::Tabletype;
use async_trait::async_trait;
use chrono::{Datelike, Local};
use std::convert::TryFrom;
use telegram_bot::*;

use crate::command::CommandKind;
use crate::command::CommandWithCallback;
use crate::Context;

pub struct Anitable {
    week_data: Vec<&'static str>,
    client: anitable::Anitable,
}

impl Anitable {
    pub fn new() -> CommandKind {
        CommandKind::CommandWithCallback(Box::new(Self {
            week_data: vec!["일", "월", "화", "수", "목", "금", "토", "신작", "기타"],
            client: anitable::Anitable::new(),
        }))
    }

    fn make_keyboard(&self, selected: usize) -> InlineKeyboardMarkup {
        let mut keyboard = InlineKeyboardMarkup::new();
        for i in 0..3 {
            let row: Vec<InlineKeyboardButton> = self.week_data[(i * 3)..(i * 3 + 3)]
                .iter()
                .enumerate()
                .map(|(id, &item)| {
                    let mut item = String::from(item);
                    if selected == (i * 3 + id) {
                        item.insert(0, '*');
                    }
                    InlineKeyboardButton::callback(
                        item,
                        format!("{}/{}", self.name().unwrap(), i * 3 + id),
                    )
                })
                .collect();
            keyboard.add_row(row);
        }
        keyboard
    }

    async fn get_data(&self, week: usize) -> Result<String, Box<dyn std::error::Error>> {
        let tabletype = Tabletype::try_from(week as u8).unwrap();
        let data = self.client.list(tabletype).await?;
        let data = data.iter().fold(
            format!(
                "           *{}요일 애니 편성표*\n",
                self.week_data.get(week).unwrap()
            ),
            |mut acc, x| {
                let to_str = format!(
                    "\n`{}:{}` `│` `{:.16}` ",
                    &x.time[..2],
                    &x.time[2..],
                    x.subject
                );
                acc.push_str(&to_str);
                acc
            },
        );
        Ok(data)
    }
}

#[async_trait]
impl CommandWithCallback for Anitable {
    fn name(&self) -> Option<&'static str> {
        Some("anitable")
    }

    async fn on_command(
        &self,
        ctx: &Context,
        message: &Message,
        _arg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let local = Local::now();
        let week = (local.weekday() as usize + 1) % 7;
        let data = self.get_data(week).await?;
        let mut msg = message.chat.text(format!("{}", data));
        msg.reply_markup(self.make_keyboard(week));
        ctx.api.send(msg.parse_mode(ParseMode::Markdown)).await?;
        Ok(())
    }

    async fn on_callback(
        &self,
        ctx: &Context,
        callback_query: &CallbackQuery,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let week = *callback_query
            .data
            .as_ref()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap();
        let week: usize = week.parse().unwrap();
        let data = self.get_data(week).await?;
        let mut msg = callback_query.message.as_ref().unwrap().edit_text(data);
        msg.reply_markup(self.make_keyboard(week as usize));
        ctx.api.send(msg.parse_mode(ParseMode::Markdown)).await?;
        Ok(())
    }
}
