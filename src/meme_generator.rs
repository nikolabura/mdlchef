//use colored::*;
//use json5;
use std::error::Error;
use magick_rust::{MagickWand, DrawingWand};

#[path = "./meme_repository.rs"]
mod meme_repository;
use crate::meme_repository::FormatRepo;

pub fn mdl_to_meme(mdl: &super::mdl::MdlMeme, frepo: &FormatRepo) -> Result<(), Box<dyn Error>> {
    let wand = MagickWand::new();

    // get format
    let fmt = &frepo.formats.get(&mdl.base.format)
        .ok_or(format!("Meme format {} not found.", &mdl.base.format))?;

    // read in the base image
    let image_path = fmt.image_path.to_str().unwrap();
    wand.read_image(image_path)?;

    wand.draw_rectangle(10,20,30,40);

    wand.write_image("bruh.png").unwrap();

    Ok(())
}
