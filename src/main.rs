use anyhow::Context as _;
use colored::*;
use serenity::{
    all::colours::branding::RED, async_trait, builder::{CreateEmbed, CreateMessage}, model::{channel::Message, gateway::Ready}, prelude::*
};
use shuttle_runtime::SecretStore;
use tracing::info;



const BOT_NAMES: [&str; 2] = ["Vibr", "FredBoat♪♪"];
const MUSIC_CHANNEL_ID: u64 = 961445348935610449;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if BOT_NAMES.contains(&msg.author.name.as_str()) {
            info!("Music bot being used | Checking Channel ID");

            if msg.channel_id != MUSIC_CHANNEL_ID {
                info!("{}", "MUSIC BOT NOT IN MUSIC CHANNEL | NOT GOOD".red());

                //This attempts to take user name and saves it so it can be used in let embed
                // let embed = &msg.embeds[0];
                // let author_name = if let Some(author) = &embed.author {
                //     author.name.clone()
                // } else {
                //     "Unknown User".to_string()
                // };
                // let mut user_mention_str = String::new(); 
                // if let Some(description) = &embed.description {
                //     let re = Regex::new(r"(<@\d+>)").unwrap();
                //     if let Some(captures) = re.captures(description) {
                //         if let Some(user_mention) = captures.get(1) {
                //             user_mention_str = user_mention.as_str().to_string();
                //             println!("Extracted user mention: {}", user_mention_str);
                //         }
                //     }
                // }

                //create our own embed to be sent to channel
                let embed = CreateEmbed::default()
                    .title(format!("DUMMY DETECTED!!"))
                    .description("🚨🚨🚨\n\nPlease only use music bot in **music-bot-only** \n\n🚨🚨🚨")
                    .color(RED);

                let message = CreateMessage::new()
                    .embed(embed);


                //we only want to send message if a new song is being played
                //don't send message for songs ending just delete
                if !msg.embeds.is_empty() {
                    let embed = &msg.embeds[0];
                    if let Some(description) = &embed.description {
                        info!("Embed description: {}", description);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                            info!("Error sending message: {:?}", why);
                        }
                    }
    
                }
               
                if let Err(why) = msg.delete(&ctx.http).await {
                    info!("Error deleting message: {:?}", why);
                } else {
                    info!("{}", "Message deleted successfully".green());
                }

            } else {
                info!("{}", "Music Bot in Music Channel | all good".blue());
            }

        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
