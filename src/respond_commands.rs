use serenity::{
    model::{interactions::Interaction},
    prelude::*,
    utils::MessageBuilder,
};

use serde_json::json;
// use std::io; use std::io::Write;
use colored::*;

pub async fn interaction_create(ctx: Context, interaction: Interaction) {
    //println!("inter {:#?}", interaction);
    let int = interaction.clone();
    let interaction_data = interaction.data.expect("Interaction had no data");
    let interaction_name = interaction_data.name.as_str();
    let interaction_user = interaction.member.user.name;
    let interaction = int;
    println!("Got interaction {} from user {}.",
        interaction_name.yellow(), interaction_user.yellow());
    match interaction_name {
        "help" => respond_help(ctx, interaction).await,
        _ => println!("{}... {:#?}", "UNEXPECTED INTERACTION".red().bold(), interaction)
    }
}

async fn respond_help(ctx: Context, interaction: Interaction) {
    let help = MessageBuilder::new()
        .push_bold("MDLChef Bot\n")
        .push("This bot generates memes using MDL, the Meme Description Language.")
        .build();
    ctx.http.create_interaction_response(
        *interaction.id.as_u64(),
        &interaction.token,
        &json!({
            "type": 4,
            "data": {
                "content": help
            }
        })
    ).await.unwrap();
}