use telegram_bot::*;
use async_trait::async_trait;
use mongodb::{Bson, bson, doc};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::UpdateOptions;

use crate::command::CommandKind;
use crate::command::Command;
use crate::Context;

pub struct Memo {}

impl Memo {
    pub fn new() -> CommandKind {
        CommandKind::Command(Box::new(Self {}))
    }
}

impl Memo {
    async fn do_reply(
        &self,
        ctx: &Context,
        reply: &Box<MessageOrChannelPost>,
        message: &Message,
        arg: &str
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        let coll = ctx.db.collection("memos");
        let msg_id = reply.to_message_id().to_string().parse::<i64>().unwrap();
        let chat_id = message.to_source_chat().to_string().parse::<i64>().unwrap();
        let mut opts = UpdateOptions::new();
        opts.upsert = Some(true);
        coll.update_one(doc!{
            "name": arg,
            "chat_id": chat_id
        }, doc!{
            "$set": {
                "name": arg,
                "chat_id": chat_id,
                "message_id": msg_id
            }
        }, Some(opts)).unwrap();
        let mut message = message.text_reply("저장했습니다.");
        message.reply_markup(reply_markup!(inline_keyboard, ["지우기" callback "selfdel"]));
        ctx.api.send(message).await?;
        Ok(())
    }

    async fn do_msg(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let coll = ctx.db.collection("memos");
        let id = message.chat.id().to_string().parse::<i64>().unwrap();
        let doc = coll.find_one(Some(doc!{"name": arg, "chat_id": id}), None).unwrap().unwrap();
        let msg_id = match doc.get("message_id") {
            Some(&Bson::I64(msg_id)) => msg_id,
            Some(&Bson::I32(msg_id)) => msg_id as i64,
            _ => return Ok(()),
        };
        let req = ForwardMessage::new(MessageId::new(msg_id), &message.to_source_chat(), &message.to_source_chat());
        ctx.api.send(req).await?;
        Ok(())
    }
}

#[async_trait]
impl Command for Memo {
    async fn on_command(&self, ctx: &Context, message: &Message, arg: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(reply) = &message.reply_to_message {
            self.do_reply(ctx, reply, message, arg).await?;
        } else {
            self.do_msg(ctx, message, arg).await?;
        }
        Ok(())
    }
}
