use serenity::{model::interactions::Interaction, prelude::*, utils::MessageBuilder};

use colored::*;
use serde_json::json;

use crate::{mdl::MdlMeme, meme_repository::*};

// RECEIVING POINT FOR ALL INTERACTIONS
pub async fn interaction_create(frepo: &FormatRepo, ctx: Context, interaction: Interaction) {
    //println!("inter {:#?}", interaction);
    let interaction_data = interaction.data.clone().expect("Interaction had no data");
    let interaction_name = interaction_data.name.as_str();
    let interaction_user = match &interaction {
        Interaction {
            member: Some(member),
            ..
        } => member.display_name().to_string(),
        Interaction {
            user: Some(user), ..
        } => user.name.to_string() + " (in DM)",
        _ => "nouser".to_string(),
    };
    println!(
        "--- {}\nGot interaction {} from user {}.",
        chrono::Local::now()
            .format("%a %b %e %T")
            .to_string()
            .bright_black(),
        interaction_name.yellow(),
        interaction_user.yellow()
    );
    match interaction_name {
        "help" => respond_help(ctx, interaction).await,
        "credits" => respond_credits(ctx, interaction).await,
        "searchmemes" => respond_unimpl(ctx, interaction).await,
        "memeinfo" => respond_memeinfo(frepo, ctx, interaction).await,
        "listmemes" => respond_listmemes(frepo, ctx, interaction).await,
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
```
"#)
        .push("Just send a valid MDL snippet in chat and the bot will automatically recognize it and respond. ")
        .push("It can be either standalone, in a \\`\\`\\` code structure, or surrounded by other text - anything should work.")
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
        .push_line("- ~~The ImageMagick `caption` command for meme generation~~")
        .push_line("- The `fontdue` and `image` crates")
        .push_line("- The competitors of DawgCTF 2021, who showed us the way of FOSS")
        .build();
    ctx.http
        .create_interaction_response(
            *interaction.id.as_u64(),
            &interaction.token,
            &json!({"type": 4, "data": { "content": resp }}),
        )
        .await
        .unwrap();
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
        )
        .await
        .unwrap();
}

async fn respond_listmemes(frepo: &FormatRepo, ctx: Context, interaction: Interaction) {
    // start writing message
    let mut mb = MessageBuilder::new();
    mb.push_underline("Listing available memes...\n");
    // print all memes into message
    let mut memeids: Vec<&String> = frepo.formats.keys().collect();
    memeids.sort();
    for memeid in memeids {
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

async fn respond_memeinfo(frepo: &FormatRepo, ctx: Context, interaction: Interaction) {
    let memeid = interaction
        .clone()
        .data
        .unwrap()
        .options
        .into_iter()
        .find(|o| o.name.eq("memeid"))
        .expect("No memeid interaction argument found")
        .value
        .expect("Memeid had no value");
    let memeid = memeid.as_str().expect("Memeid wasn't a string");

    // check if it's a valid memeid
    if let Some(format) = frepo.formats.get(memeid) {
        // it is valid!
        let insert_names = format
            .inserts
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        // construct the example MDL
        let mut inserts_mdl = String::new();
        if insert_names.len() > 0 {
            inserts_mdl.push_str(",\n  inserts: {\n");
            for (i, insert) in insert_names.iter().enumerate() {
                inserts_mdl.push_str(
                    format!(
                        "    {}: \"{}\"{}\n",
                        insert,
                        insert,
                        if i == insert_names.len() - 1 { "" } else { "," }
                    )
                    .as_str(),
                );
            }
            inserts_mdl.push_str("  }");
        }
        let example_mdl = format!(
            r#"{{
  version: "MDL/1.1",
  type: "meme",
  base: "{}",
  caption: {{
    topText: "",
    bottomText: ""
  }}{}
}}"#,
            format.memeid, inserts_mdl
        );
        // generate example meme from the example mdl
        let example_meme: MdlMeme = json5::from_str(&example_mdl).unwrap();
        let meme_image = crate::meme_generator::mdl_to_meme(&example_meme, frepo).unwrap();
        // send a temporary message with the example meme
        let sent_with_attachment = interaction
            .channel_id
            .unwrap()
            .send_message(&ctx, |m| {
                m.add_file(serenity::http::AttachmentType::Bytes {
                    data: std::borrow::Cow::from(meme_image),
                    filename: String::from("meme.png"),
                })
            })
            .await
            .unwrap();
        // get the url of the image we just uploaded
        let attachment_url = &sent_with_attachment.attachments[0].proxy_url;
        // send message with a proper embed
        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.interaction_response_data(|d| {
                    d.embed(|e| {
                        e.title(&format.memeid);
                        e.description("MDLChef Meme Information");
                        e.color(serenity::utils::Colour::from_rgb(133, 198, 232));
                        e.field("Filename", &format.image_path.to_str().unwrap(), false);
                        e.field(
                            "Inserts",
                            if insert_names.len() > 0 {
                                insert_names
                                    .iter()
                                    .map(|s| format!("`{}`", s))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            } else {
                                "*None*".to_string()
                            },
                            false,
                        );
                        e.field("Example MDL", format!("```js\n{}\n```", example_mdl), false);
                        e.thumbnail(attachment_url);
                        e
                    })
                })
            })
            .await
            .unwrap();
        // delete the temporary message
        sent_with_attachment.delete(&ctx).await.unwrap();
    } else {
        // invalid meme id
        interaction
            .create_interaction_response(ctx.http, |r| {
                r.interaction_response_data(|d| {
                    d.content(format!(":bangbang: Invalid meme identifier `{}`.", memeid))
                })
            })
            .await
            .unwrap();
    };
}
