use colored::*;
use image::{EncodableLayout, ImageBuffer, ImageEncoder};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Write;
use std::ops::Not;
use std::time::Instant;

use fontdue::layout::*;
use fontdue::*;

use crate::meme_repository::FormatRepo;

use once_cell::sync::Lazy;
static IMPACT_FONT: Lazy<Font> = Lazy::new(|| {
    let settings: &HashMap<String, String> = crate::SETTINGS
        .get()
        .expect("Settings.toml hashmap not found.");
    let impact_font_location = settings
        .get("impact_font_location")
        .expect("Error: impact_font_location not found in Settings.toml");
    let impact = std::fs::read(impact_font_location)
        .expect(&format!("Font file not found at {}", impact_font_location).to_string());
    let impact = Font::from_bytes(impact, FontSettings::default()).unwrap(); // EXPENSIVE ~50ms
    return impact;
});

/// Convert an MDL object to a PNG (represented as Vec<u8>).
pub fn mdl_to_meme(
    mdl: &super::mdl::MdlMeme,
    frepo: &FormatRepo,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // timer
    let start_time = Instant::now();

    // get format
    let fmt = frepo
        .formats
        .get(&mdl.base.format)
        .ok_or(format!("Meme format {} not found.", &mdl.base.format))?;
    print!("Generating {}... ", &mdl.base.format.blue());
    io::stdout().flush().unwrap();

    // read in the base image, get width and height
    let base_image_path = fmt.image_path.to_str().unwrap();
    let mut img = image::open(base_image_path)?.into_rgba8();
    let base_image_w = img.width();
    let base_image_h = img.height();
    let caption_height = (base_image_h / 3) as u32 - 20;

    // apply captions which exist
    if let Some(capt) = &mdl.caption.top_text {
        img = apply_caption(
            img,
            capt,
            20,
            10,
            base_image_w - 40,
            caption_height,
            VerticalAlign::Top,
        );
    }
    if let Some(capt) = &mdl.caption.center_text {
        img = apply_caption(
            img,
            capt,
            20,
            caption_height + 10,
            base_image_w - 40,
            caption_height,
            VerticalAlign::Middle,
        );
    }
    if let Some(capt) = &mdl.caption.bottom_text {
        img = apply_caption(
            img,
            capt,
            20,
            base_image_h - caption_height - 10,
            base_image_w - 40,
            caption_height,
            VerticalAlign::Bottom,
        );
    }

    // apply inserts which exist
    if let Some(inserts) = &mdl.inserts {
        let inserts = inserts
            .as_object()
            .ok_or("Key 'inserts' should be an object.")?;
        for (insert_name, insert_val) in inserts {
            let coords = fmt.inserts.get(insert_name).ok_or(format!(
                "This meme does not have an insert called \"{}\"",
                insert_name
            ))?;
            let insert_capt = insert_val
                .as_str()
                .ok_or("Insert value must be a string.")?;
            img = apply_caption(
                img,
                insert_capt,
                coords.0 .0,
                coords.0 .1,
                coords.1 .0 - coords.0 .0,
                coords.1 .1 - coords.0 .1,
                VerticalAlign::Middle,
            );
        }
    }

    // encode to PNG and output to vector
    let mut png_out = Vec::<u8>::new();
    image::codecs::png::PngEncoder::new(&mut png_out).write_image(
        img.as_bytes(),
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    )?;

    // end timer
    println!(
        "Done. Took {} ms.",
        start_time.elapsed().as_millis().to_string().yellow()
    );

    // output PNG vector
    Ok(png_out)
}

const PRINT_TIME_CAPTION_APPLICATION: bool = false;
fn print_time(start_time: &mut (Instant, Instant), text: &str) {
    if PRINT_TIME_CAPTION_APPLICATION {
        println!(
            "{}: {} ms. Total {} ms.",
            text,
            start_time.0.elapsed().as_millis().to_string().yellow(),
            start_time.1.elapsed().as_millis().to_string()
        );
        (*start_time).0 = Instant::now();
    }
}

/// Note: y is 0 at top, grows downwards.
fn apply_caption(
    mut base: image::RgbaImage,
    caption: &str,
    x_left: u32,
    y_top: u32,
    width: u32,
    height: u32,
    vert_align: VerticalAlign,
) -> image::RgbaImage {
    use image::DynamicImage::*;

    let mut start_time = (Instant::now(), Instant::now());
    print_time(&mut start_time, "start");

    let mut capt_img: image::GrayAlphaImage =
        image::ImageBuffer::from_pixel(base.width(), base.height(), image::LumaA([0, 0]));

    let mut layout = Layout::<()>::new(CoordinateSystem::PositiveYDown);

    print_time(&mut start_time, "layout");

    // initialize layout settings for caption area
    layout.reset(&LayoutSettings {
        x: x_left as f32,
        y: y_top as f32,
        max_width: Some(width as f32),
        max_height: Some(height as f32),
        horizontal_align: HorizontalAlign::Center,
        vertical_align: vert_align,
        wrap_style: WrapStyle::Word,
        wrap_hard_breaks: true,
    });

    // loop until the text fits
    let impfont: &Font = &IMPACT_FONT;
    let mut size: f32 = height as f32 * 0.8;
    while size > 6.0 {
        layout.clear();
        //println!("{:#?}", caption.as_bytes());
        layout.append(&[impfont], &TextStyle::new(caption, size, 0));
        //println!("{}, {}", height, layout.height());
        if layout.height() <= height as f32 && !(layout.lines() > caption.matches(' ').count() + 1)
        {
            break;
        }
        size -= 3.0;
    }

    print_time(&mut start_time, "loop till fits");

    // draw each glyph onto the capt_img
    for glyph in layout.glyphs() {
        //println!("{:#?}", glyph);
        let (metrics, bitmap) = IMPACT_FONT.rasterize_config(glyph.key);
        let height = metrics.height;
        let width = metrics.width;
        for j in 0..height {
            let val = &bitmap[j * width..(j + 1) * width];
            for (i, v) in val.iter().enumerate() {
                let image_x = (i + glyph.x as usize) as u32;
                let image_y = (j + glyph.y as usize) as u32;
                if (0..base.width()).contains(&image_x).not()
                    || (0..base.height()).contains(&image_y).not()
                {
                    continue;
                }
                let mut pixel = capt_img.get_pixel_mut(image_x, image_y);
                pixel.0[0] = std::cmp::max(pixel.0[0], *v);
                if *v > 0 {
                    pixel.0[1] = 255;
                }
            }
        }
    }

    print_time(&mut start_time, "draw glyphs");

    // add border around letters
    let mut transparency_plane =
        ImageBuffer::from_fn(capt_img.width(), capt_img.height(), |x, y| {
            return image::Luma([capt_img.get_pixel(x, y).0[1]]);
        });
    print_time(&mut start_time, "extract plane");
    imageproc::morphology::dilate_mut(
        &mut transparency_plane,
        imageproc::distance_transform::Norm::LInf,
        std::convert::TryInto::try_into((capt_img.height() + capt_img.width()) / (2 * 150))
            .unwrap(),
    );
    print_time(&mut start_time, "dialating");
    //let transparency_plane = imageproc::filter::gaussian_blur_f32(&transparency_plane, 1.2);
    let transparency_plane = imageproc::filter::box_filter(&transparency_plane, 1, 1);
    print_time(&mut start_time, "blurring");
    for (_, r) in capt_img.enumerate_rows_mut() {
        for (x, y, p) in r {
            p.0[1] = transparency_plane.get_pixel(x, y).0[0];
        }
    }

    print_time(&mut start_time, "restore plane");

    // overlay capt_img over the base
    image::imageops::overlay(
        &mut base,
        &image::DynamicImage::into_rgba8(ImageLumaA8(capt_img)),
        0,
        0,
    );

    print_time(&mut start_time, "overlay");

    return base;
}
