use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        interactions::Interaction,
    },
    prelude::*,
};

use colored::*;
use regex::RegexBuilder;
use std::collections::HashMap;
use std::path::PathBuf;

use magick_rust::{magick_wand_genesis};
use std::sync::Once;

mod create_commands;
mod meme_repository;
mod respond_commands;
mod respond_mdl;

struct Handler {
    pub meme_format_repo: meme_repository::FormatRepo,
}

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.contains("MDL/1.") {
            // Message might be a valid MDL snippet. Regex it.
            let re = RegexBuilder::new(r"\{.*MDL/1\..*\}")
                .dot_matches_new_line(true)
                .build()
                .unwrap();
            if let Some(cap) = re.captures(msg.content.as_str()) {
                // Found possible MDL region.
                let mdlstr = cap.get(0).unwrap().as_str();
                respond_mdl::respond_mdl(&self.meme_format_repo, ctx, &msg, mdlstr).await;
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name.blue().bold());
        ctx.set_activity(Activity::playing("serving it up Gary's way"))
            .await;
    }

    // Triggered when receiving interaction.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        respond_commands::interaction_create(&self.meme_format_repo, ctx, interaction).await;
    }
}

#[tokio::main]
async fn main() {
    // Get settings file
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .expect("Expected Settings.toml file in top directory");
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();

    // Configure the client with your Discord bot token in the settings file.
    let token = settings
        .get("token")
        .expect("Error: token not found in Settings.toml");
    let application_id: u64 = settings
        .get("application_id")
        .expect("Error: application_id not found in Settings.toml")
        .parse()
        .expect("Error: application_id in Settings.toml is not a numeric");
    let meme_repo_folder = settings
        .get("meme_repo_folder")
        .expect("Error: meme_repo_folder not found in Settings.toml");

    // Launch ImageMagick
    START.call_once(|| {
        magick_wand_genesis();
    });

    // Initialize the meme format repository and put it in the Handler.
    let meme_format_repo = meme_repository::FormatRepo::new(
        PathBuf::from(meme_repo_folder),
        "Meme".to_string()).expect("Died: Failed to create format repo.");
    let handler = Handler {
        meme_format_repo
    };

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(handler)
        .await
        .expect("Error creating client.");

    // Create the slash commands.
    // VVVV Set to true to refresh slash commands.
    if false {
        create_commands::issue_command_creation(&client, application_id).await
    };

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
