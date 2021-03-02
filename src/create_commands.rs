use serenity::{
    prelude::*,
};

use serde_json::json;
use std::io;
use std::io::Write;

const GUILD_ID: u64 = 552980096315686951;

pub async fn issue_command_creation(client: &Client, application_id: u64) {
    let http = &(client.cache_and_http.http);
    let mut commands_array = Vec::new();

    // SLASH COMMANDS BEGIN

    commands_array.push(json!({
        "name": "help",
        "description": "Receive help and usage info on the MDLChef bot."
    }));

    commands_array.push(json!({
        "name": "listmemes",
        "description": "List all available memes.",
        "options": [
            {
                "name": "memeid",
                "description": "The identifier of the meme. Wildcards accepted.",
                "type": 3,
                "required": false
            }
        ]
    }));

    commands_array.push(json!({
        "name": "searchmemes",
        "description": "Search for a meme by text.",
        "options": [
            {
                "name": "query",
                "description": "A string to look for in the meme ID or metadata.",
                "type": 3,
                "required": true
            }
        ]
    }));

    commands_array.push(json!({
        "name": "memeinfo",
        "description": "Get detailed metadata on a meme.",
        "options": [
            {
                "name": "memeid",
                "description": "The fully-qualified identifier of the meme.",
                "type": 3,
                "required": true
            }
        ]
    }));

    // END SLASH COMMANDS

    print!("Creating slash commands... ");
    io::stdout().flush().unwrap();
    for command in commands_array {
        http.create_guild_application_command(
            application_id,
            GUILD_ID,
            &command
        ).await.unwrap();
        print!("#");
        io::stdout().flush().unwrap();
    }
    println!(" Done.");

    // This code block allows you to flush all slash commands
    const DELETE_ALL_COMMANDS: bool = false;
    if DELETE_ALL_COMMANDS {
        let all_commands = http.get_guild_application_commands(
            application_id,
            GUILD_ID,
        ).await.unwrap();
        println!("Result: {:#?}", all_commands);
        for command in all_commands {
            let cid = command.id.as_u64();
            let result = http.delete_guild_application_command(
                application_id,
                GUILD_ID,
                *cid
            ).await.unwrap();
            println!("Deletion: {:?}", result);
        }
    }
}