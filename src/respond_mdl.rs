use serenity::{model::channel::Message, prelude::*};

use colored::*;
use json5;

#[path = "./mdl.rs"]
mod mdl;
use mdl::MdlMeme;

#[path = "./meme_generator.rs"]
mod meme_generator;

#[path = "./meme_repository.rs"]
mod meme_repository;

/// Call this to respond to a message containing suspected MDL JSON.
pub async fn respond_mdl(frepo: &super::meme_repository::FormatRepo, ctx: Context, msg: &Message, mdlstr: &str) {
    // Print username
    print!(
        "Got likely MDL snippet from user {}... ",
        msg.member
            .as_ref()
            .unwrap()
            .nick
            .as_ref()
            .unwrap_or(&"#NONICK".to_string())
            .yellow()
    );
    // Attempt deserialization
    let meme: MdlMeme = match json5::from_str(mdlstr) {
        Ok(v) => v,
        Err(e) => {
            reply_error(
                ctx,
                msg,
                "MDL Parsing Failure",
                &e.to_string(),
                true,
            )
            .await;
            return;
        }
    };

    // VALIDATION
    if meme.r#type != "meme" {
        reply_error(
            ctx,
            msg,
            "MDL Validation Failure",
            "`type` field did not equal 'meme'.",
            false,
        )
        .await;
        return;
    }
    if meme.version != "MDL/1.1" {
        reply_error(
            ctx,
            msg,
            "MDL Validation Failure",
            "`version` field did not equal 'MDL/1.1'. This is the only supported \
            version as of now.",
            false,
        )
        .await;
        return;
    }

    // Appears to be a valid MDL meme
    println!("{}", "Looks valid!".green());
    println!("{:#?}", meme);

    // Generate the meme and handle errors
    let memegen_result = match meme_generator::mdl_to_meme(&meme, frepo) {
        Ok(_) => "".to_string(),
        Err(e) => e.to_string(),
    };
    if memegen_result != "" {
        reply_error(
            ctx,
            msg,
            "Meme Generation Failure",
            &memegen_result,
            true,
        )
        .await;
        return;
    }
}

pub async fn reply_error(ctx: Context, msg: &Message, title: &str, error: &str, code: bool) {
    let fulltext = if code {
        format!(":warning: __{}:__\n```\n{}\n```", title, error)
    } else {
        format!(":warning: __{}:__ {}", title, error)
    };
    println!(
        "Replying with error! {} {}",
        title.red().bold(),
        error.red()
    );
    let _ = msg.reply_ping(ctx.http, fulltext).await;
}
