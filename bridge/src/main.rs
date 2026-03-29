use std::env;
use dotenv::dotenv;
use teloxide::prelude::*;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌐 V.O.I.D. Omni-Bridge: Scanning for active keys...");

    // --- 1. TELEGRAM BRIDGE ---
    if let Ok(token) = env::var("TELEGRAM_TOKEN") {
        println!("🤖 TELEGRAM: Key detected. Activating...");
        tokio::spawn(async move {
            let bot = Bot::new(token);
            teloxide::repl(bot, |bot: Bot, msg: Message| async move {
                bot.send_message(msg.chat.id, "🌌 V.O.I.D. Linked. Commands: /status /shards").await?;
                respond(())
            }).await;
        });
    }

    // --- 2. DISCORD BRIDGE ---
    if let Ok(token) = env::var("DISCORD_TOKEN") {
        println!("🎮 DISCORD: Key detected. Activating...");
        tokio::spawn(async move {
            let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
                .await.expect("Err creating Discord client");
            if let Err(why) = client.start().await {
                println!("❌ Discord error: {:?}", why);
            }
        });
    }

    println!("🚀 Bridge is holding. Waiting for messages...");
    
    // Keep the process alive
    loop { tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; }
}
