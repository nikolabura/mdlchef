use once_cell::sync::OnceCell;
use colored::*;
use regex::RegexBuilder;
use std::collections::HashMap;
use std::path::PathBuf;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        interactions::Interaction,
    },
    prelude::*,
};

//mod create_commands;
mod mdl;
mod meme_generator;
mod meme_repository;
mod respond_commands;
mod respond_mdl;

pub static SETTINGS: OnceCell<HashMap<String, String>> = OnceCell::new();

struct Handler {
    pub meme_format_repo: meme_repository::FormatRepo,
    pub settings: HashMap<String, String>,
}

#[async_trait]
impl EventHandler for Handler {
    // Executes upon receiving message in DM or chat
    async fn message(&self, ctx: Context, msg: Message) {
        // Scan the message for MDL signature
        if msg.content.contains("MDL/1.") {
            // Ensure it's not MDIR
            if msg.content.contains("// MDLChef MDIR") {
                return;
            }
            // Message might be a valid MDL snippet. Regex it.
            let re = RegexBuilder::new(r"\{.*MDL/1\..*\}")
                .dot_matches_new_line(true)
                .build()
                .unwrap();
            if let Some(cap) = re.captures(msg.content.as_str()) {
                // Found possible MDL region.
                let mdlstr = cap.get(0).unwrap().as_str();
                ctx.http.broadcast_typing(msg.channel_id.0).await.unwrap();
                // NOTE: this ^^^ breaks interaction response
                // since iteration response cannot occur while typing :(
                respond_mdl::respond_mdl(&self.meme_format_repo, ctx, &msg, mdlstr)
                    .await;
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name.blue().bold());
        ctx.set_activity(Activity::playing("DM me and say /help"))
            .await;
    }

    // Triggered when receiving interaction.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        //println!("{:#?}", interaction);
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
    let settings: HashMap<String, String> = settings.try_into::<HashMap<String, String>>().unwrap();
    SETTINGS.set(settings.clone()).unwrap();

    // Get bot token, app ID, and meme repo directory location from the settings
    let token: String = settings
        .get("token")
        .expect("Error: token not found in Settings.toml")
        .clone();
    let application_id: u64 = settings
        .get("application_id")
        .expect("Error: application_id not found in Settings.toml")
        .parse()
        .expect("Error: application_id in Settings.toml is not a numeric");
    let meme_repo_folder = settings
        .get("meme_repo_folder")
        .expect("Error: meme_repo_folder not found in Settings.toml");

    // Initialize the meme format repository and put it in the Handler
    let meme_format_repo =
        meme_repository::FormatRepo::new(PathBuf::from(meme_repo_folder), "Meme".to_string())
            .expect("Died: Failed to create format repo.");
    let handler = Handler {
        meme_format_repo,
        settings,
    };

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .application_id(application_id)
        .event_handler(handler)
        .await
        .expect("Error creating client.");

    // Create the slash commands.
    // VVVV Set to true to refresh slash commands.
    if true {
        /*serenity::model::interactions::ApplicationCommand::create_global_application_command(&(client.cache_and_http.http), |a| {
            a.name("credits").description("View credits and learn about the technology behind this bot.")
        }).await.unwrap();*/
        /*println!("{:#?}",
        serenity::model::interactions::ApplicationCommand::get_global_application_commands(&(client.cache_and_http.http)).await.unwrap());*/
        /*serenity::model::interactions::ApplicationCommand::delete_global_application_command(&(client.cache_and_http.http),
        serenity::model::id::CommandId(816539740425814037)).await.unwrap();*/
        //create_commands::issue_command_creation(&client, application_id).await
    };

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
