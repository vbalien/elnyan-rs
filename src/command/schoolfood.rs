use telegram_bot::*;
use async_trait::async_trait;
use regex::Regex;
use chrono::*;

use crate::command::Command;

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
        let local: DateTime<Local> = Local::now();
        let url = format!("http://m.soongguri.com/m_req/m_menu.php?rcd=1&sdt={}", local.format("%Y%m%d").to_string());
        let body = reqwest::get(&url)
            .await?
            .text()
            .await?;
        let re = Regex::new(r#"<td class="menu_nm">(.*)</td>\s*<td class="menu_list"><div>(.*)</div>"#).unwrap();
        let data: Vec<_> = re.captures_iter(&body).map(|cap| {
            FoodData {
                kind: String::from(&cap[1]),
                foods: String::from(&cap[2])
                    .split("</div><div>").filter(|x| {
                        let tmp = String::from(*x);
                        !tmp.contains("<div") && !tmp.contains("<span")
                    })
                    .map(|s| {String::from(s)})
                    .collect(),
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

