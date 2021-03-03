//use colored::*;
//use json5;
use std::process::Stdio;
use std::str;
use std::error::Error;
use std::collections::HashMap;
use std::process::Command;

#[path = "./meme_repository.rs"]
mod meme_repository;
use crate::meme_repository::FormatRepo;

pub fn mdl_to_meme(
    mdl: &super::mdl::MdlMeme,
    frepo: &FormatRepo,
    settings: &HashMap<String, String>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // get format
    let fmt = &frepo.formats.get(&mdl.base.format)
        .ok_or(format!("Meme format {} not found.", &mdl.base.format))?;

    // read in the base image
    let image_path = fmt.image_path.to_str().unwrap();

    // get impact font
    let impact_font_location = settings
        .get("impact_font_location")
        .expect("Error: impact_font_location not found in Settings.toml");

    // determine image width and height
    let img_w: i32 = str::from_utf8(&Command::new("identify")
                         .stderr(Stdio::inherit())
                         .arg("-format").arg("%w").arg(image_path)
                         .output()?.stdout)?.parse()?;
    let img_h: i32 = str::from_utf8(&Command::new("identify")
                         .stderr(Stdio::inherit())
                         .arg("-format").arg("%h").arg(image_path)
                         .output()?.stdout)?.parse()?;
    let avgdim = (img_w + img_h) / 2;

    // start building the generator command   
    let mut gen_cmd = Command::new("convert");
    gen_cmd.arg(image_path)
           .stderr(Stdio::inherit())
           .arg("-background").arg("none")
           .arg("-font").arg(impact_font_location)
           .arg("-fill").arg("white")
           .arg("-strokewidth").arg(format!("{}", avgdim / 200))
           .arg("-stroke").arg("black")
           .arg("-size").arg(format!("{}x{}", img_w - img_w/12, img_h/4));

    // add each caption
    if let Some(capt) = &mdl.caption.top_text {
        gen_cmd.arg("-gravity").arg("north").arg(format!("caption:{}", capt))
               .arg("-composite");
    }
    if let Some(capt) = &mdl.caption.center_text {
        gen_cmd.arg("-gravity").arg("center").arg(format!("caption:{}", capt))
               .arg("-composite");
    }
    if let Some(capt) = &mdl.caption.bottom_text {
        gen_cmd.arg("-gravity").arg("south").arg(format!("caption:{}", capt))
               .arg("-composite");
    }

    // run generator
    gen_cmd.arg("png:-");
    let output_image = gen_cmd.output()?.stdout;

    Ok(output_image)
}
