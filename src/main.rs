extern crate dotenv;
use dotenv::dotenv;

use std::collections::HashSet;
use std::env;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::client::{Client, Context, EventHandler};

use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{help_commands, StandardFramework, CommandResult, Args, HelpOptions, CommandGroup};

#[group]
#[commands(ping)]

struct General;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("ready!");
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        ctx.set_presence(Some(Activity::streaming(format!("!help | {} server(s)", guilds.len()), "https://www.twitch.tv/moby25k")), OnlineStatus::Online).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|cfg| {
            cfg.
                prefix("!")
        }) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command("ping")]
#[description = "Pings the bot!"]
#[usage = "!ping"]
#[example = "!ping"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
