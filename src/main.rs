use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
#[commands(help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP).configure(|c| c.prefix("r$"));
    // framework; // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = "MTE3Njk5MzI4MTE1NTAyNzAyNA.G-L7dq.YYkocsQANrH6FiUKpJUxtWN8lsQPyKVDNGBLso";
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
