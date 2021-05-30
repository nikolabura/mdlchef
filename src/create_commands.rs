use serenity::model::interactions::ApplicationCommand;
use serenity::model::interactions::ApplicationCommandOptionType;
use serenity::prelude::*;

const _GUILD_ID: u64 = 552980096315686951;

pub async fn issue_command_creation(client: &Client) {
    let http = &(client.cache_and_http.http);

    ApplicationCommand::create_global_application_command(http, |a| {
        a.name("credits")
            .description("View credits and learn about the technology behind this bot.")
    })
    .await
    .unwrap();

    ApplicationCommand::create_global_application_command(http, |a| {
        a.name("help")
            .description("Receive help and usage info on the MDLChef bot.")
    })
    .await
    .unwrap();

    ApplicationCommand::create_global_application_command(http, |a| {
        a.name("listmemes")
            .description("List all available meme templates/formats.")
            .create_option(|o| {
                o.name("memeid")
                    .description("The identifier of the meme. Asterisk wildcards accepted.")
                    .kind(ApplicationCommandOptionType::String)
                    .required(false)
            })
    })
    .await
    .unwrap();

    ApplicationCommand::create_global_application_command(http, |a| {
        a.name("memeinfo")
            .description("Get detailed metadata on a meme.")
            .create_option(|o| {
                o.name("memeid")
                    .description("The fully-qualified identifier of the meme.")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    })
    .await
    .unwrap();
}
