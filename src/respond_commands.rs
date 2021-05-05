use serenity::{model::interactions::Interaction, prelude::*, utils::MessageBuilder};

use colored::*;
use serde_json::json;

#[path = "./meme_repository.rs"]
mod meme_repository;
use super::meme_repository::FormatRepo;

pub async fn interaction_create(
    frepo: &FormatRepo,
    ctx: Context,
    interaction: Interaction,
) {
    //println!("inter {:#?}", interaction);
    let int = interaction.clone();
    let interaction_data = interaction.data.expect("Interaction had no data");
    let interaction_name = interaction_data.name.as_str();
    let interaction_user = interaction.user.unwrap().name;
    let interaction = int;
    println!(
        "Got interaction {} from user {}.",
        interaction_name.yellow(),
        interaction_user.yellow()
    );
    match interaction_name {
        "help"        => respond_help(ctx, interaction).await,
        "credits"     => respond_credits(ctx, interaction).await,
        "searchmemes" => respond_unimpl(ctx, interaction).await,
        "memeinfo"    => respond_unimpl(ctx, interaction).await,
        "listmemes"   => respond_listmemes(frepo, ctx, interaction).await,
        _ => println!(
            "{}... {:#?}",
            "UNEXPECTED INTERACTION".red().bold(),
            interaction
        ),
    }
}

async fn respond_help(ctx: Context, interaction: Interaction) {
    let help = MessageBuilder::new()
        .push_bold("MDLChef Bot\n")
        .push("This bot generates memes using MDL, the Meme Description Language.\n")
        .push(r#"Here is an example of a valid MDL sample:
```js
{
    version: "MDL/1.1",
    type: "meme",
    base: {
        format: "Meme.Matrix.WhatIfIToldYou"
    },
    caption: {
        topText: "what if i told you",
        bottomText: "you can code your memes"
    }
}
```"#)
        .build();
    ctx.http
        .create_interaction_response(
            *interaction.id.as_u64(),
            &interaction.token,
            &json!({
                "type": 4,
                "data": {
                    "content": help
                }
            }),
        )
        .await
        .unwrap();
}

async fn respond_credits(ctx: Context, interaction: Interaction) {
    let resp = MessageBuilder::new()
        .push_underline_line("Credits")
        .push_line("Thank you to...")
        .push_line("- The Rust programming language")
        .push_line("- The Serenity Discord library")
        .push_line("- The ImageMagick `caption` command for meme generation")
        .push_line("Note: The source code for this bot is NOT publicly available, due to the CyberDawgs' extreme \
            anti-open-source and pro-proprietary stance. We don't NEED public auditing. Everything in this bot is totally \
            and completely secure.")
        .build();
    ctx.http
        .create_interaction_response(
            *interaction.id.as_u64(),
            &interaction.token,
            &json!({"type": 4, "data": { "content": resp }}),
        ).await.unwrap();
}

async fn respond_unimpl(ctx: Context, interaction: Interaction) {
    let resp = MessageBuilder::new()
        .push_bold(":warning: Error\n")
        .push("This slash command is not yet implemented.")
        .build();
    ctx.http
        .create_interaction_response(
            *interaction.id.as_u64(),
            &interaction.token,
            &json!({"type": 4, "data": { "content": resp }}),
        ).await.unwrap();
}

async fn respond_listmemes(frepo: &FormatRepo, ctx: Context, interaction: Interaction) {
    // start writing message
    let mut mb = MessageBuilder::new();
    mb.push_underline("Listing available memes...\n");
    // print all memes into message
    for (memeid, _value) in &frepo.formats {
        mb.push(format!("{}\n", memeid));
    }
    // output the message as response
    let mut output = mb.build();
    if output.len() > 1990 {
        output.truncate(1990);
        output = format!("{} ...", output);
    }
    ctx.http
        .create_interaction_response(
            *interaction.id.as_u64(),
            &interaction.token,
            &json!({
                "type": 4,
                "data": {
                    "content": output
                }
            }),
        )
        .await
        .unwrap();
}
