# MDLChef

MDLChef is a Discord bot written in Rust using the Serenity framework. It generates memes from MDL, the **Meme Description Language** (ok, yes, it's just JSON).

## Building

To run, run `cargo run`. We recommend using `cargo run --release` for greatly improved performance (meme generation in ~150ms in release mode, versus ~4000ms in debug mode).

### Requirements

- Impact font (recommend using `msttcorefonts`).

### Hosting

Create a file in the root directory called `Settings.toml`.

Populate its contents similarly to as follows:

```toml
# MDLChef Bot Settings
# Do not share!

token = "DISCORD_BOT_TOKEN_HERE"
application_id = 123456789
meme_repo_folder = "memeformats"
impact_font_location = "/usr/share/fonts/truetype/msttcorefonts/Impact.ttf"
```

- `token` should be your Discord bot token.
- `application_id` should be your Discord application ID.
- `meme_repo_folder` points to the folder in the top-level directory which contains the meme repository. By default this is `memeformats`.
- `impact_font_location` is the location of the `Impact.ttf` file on your machine.

## Usage

Add the bot to a Discord server. Then, in any channel where the bot has read and write permissions, paste an MDL message, like follows:

```js
{
  version: "MDL/1.1",
  type: 'meme',
  base: {
    format: "Meme.DrakeYesNo"
  },
  caption: {
    topText: "Good thing",
    bottomText: "Hmm bad thing?"
  }
}
```

It can be either alone or in a code-block. The bot is triggered by the presence of `MDL/1` and a valid JSON5 object.

The bot also has several slash commands; enable the flag in main.rs to populate these for your own bot instance.

Have fun!