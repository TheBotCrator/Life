use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::*;

use database::ConnectionPool;
use utils::time::now_utc;

pub fn on_message(_ctx: &Context, pool: &ConnectionPool, msg: &Message) {
    // skip empty messages, images / embeds / etc
    if msg.content.is_empty() {
        return;
    }

    if msg.author.bot {
        return;
    }

    let guild_id = match msg.guild_id() {
        Some(guild) => guild.0,
        None => return,
    };

    if let Some(notifications) = pool.get_notifications(&msg.content.to_lowercase(), guild_id) {
        for notification in notifications {
            // skip notifications for self
            if notification.user_id as u64 == msg.author.id.0 {
                continue;
            }

            // message user
            let user = UserId(notification.user_id as u64);

            if let Ok(channel) = user.create_dm_channel() {
                let desc = format!("Your notification `{}` was triggered in {}", notification.keyword, msg.channel_id.mention());

                // maybe switch to use Channel::messages() instead?
                let mut messages = pool.get_messages(msg.channel_id.0, 3);


                let _ = channel.id.send_message(|m| m
                    .embed(|e| {
                        let mut e = e.color(0xf58b28)
                        .description(desc)
                        .timestamp(now_utc().format("%Y-%m-%dT%H:%M:%S").to_string());

                        if let Some(ref mut messages) = messages {
                            messages.reverse();
                            for message in messages {
                                // bold the keyword
                                let content = message.content.replace(&notification.keyword, &format!("**{}**", notification.keyword));

                                e = e.field(format!("[{}] {}", message.created.format("%H:%M:%S UTC"), message.tag),
                                    format!("> {}", content),
                                    false);
                            }
                        } else {
                            let content = msg.content.replace(&notification.keyword, &format!("**{}**", notification.keyword));

                            e = e.field(format!("[{}] {}", msg.timestamp.format("%H:%M:%S UTC"), msg.author.tag()),
                                format!("> {}", content),
                                false);
                        }

                        e
                    })
                );
            } else {
                let s = format!(
                    "Failed sending notification message to: {}",
                    &notification.user_id.to_string()
                );
                error!("{}", s);
            }
        }
    }
}
