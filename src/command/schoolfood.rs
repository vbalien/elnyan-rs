use telegram_bot::*;
use async_trait::async_trait;
use regex::Regex;
use crate::command::Command;
use chrono::Local;
use std::collections::HashMap;


pub struct Schoolfood {}

#[derive(Debug, Clone)]
struct FoodData {
    kind: String,
    foods: Vec<String>,
}

impl FoodData {
    pub fn to_string(self) -> String {
        let data: String = self.foods.iter().fold(String::from(""), |mut acc, x| {
            acc.push_str(&format!("- {}", x));
            acc.push('\n');
            acc
        });
        format!("━━━━━{}━━━━━\n{}", self.kind, data)
    }
}

#[async_trait]
impl Command for Schoolfood {
    async fn execute(&self, api: Api, message: &Message, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sections = vec!["중식1", "중식2", "중식3", "석식1"];
        let local = Local::now();
        let url = "http://soongguri.com/menu/m_menujson.php";
        let re = Regex::new(r#"<[^>]*>"#).unwrap();
        let re_eng = Regex::new(r#"[A-Za-z]"#).unwrap();
        let data: HashMap<String, HashMap<String, String>> = reqwest::get(url)
            .await?
            .json()
            .await?;
        let data = data.get("학생식당").unwrap();
        let data: Vec<_> = sections.iter().map(|section| {
            let foods: Vec<String> = re.split(data.get(*section).unwrap()).filter(|x| {
                !x.trim().is_empty() && !re_eng.is_match(x)
            }).map(|s| {String::from(s)}).collect();
            FoodData {
                kind: section.to_string(),
                foods,
            }
        }).collect();
        let data: String = data.iter().fold(String::from(""), |mut acc, x| {
            let tmp = x.clone().to_string();
            acc.push_str(&tmp);
            acc.push('\n');
            acc
        });
        
        api.send(message.chat.text(format!("{}\n{}", local.format("%Y년 %m월 %d일 %A 학식"), data))).await?;
        Ok(())
    }
}
