use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::{DynamicImage, RgbImage};
use js_sys::Uint8Array;
use num::clamp;
use serde::{Deserialize, Serialize};

use std::io::Cursor;
use wasm_bindgen::prelude::*;

/**
 * TODOs
 *
 * Provide a means to bring options into one of the entry points below
 * Research and write resize operations
 * Write option handler for resize and format operations.
 */

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct ImageOptionsInput {
    pub identifier: Option<String>,
    pub new_format: Option<String>,
    pub do_not_convert: Option<bool>,
    pub max_size: Option<f64>,
    pub image_quality: Option<f64>,
}

enum ImageType {
    Png,
    Jpeg,
    Gif,
    Bmp,
    Tiff,
    Same,
}

struct ImageOptions {
    pub new_format: ImageType,
    pub max_size: u32,
    pub image_quality: u8,
}

#[wasm_bindgen]
pub fn process_image(bytes: &[u8], input: JsValue) -> Uint8Array {
    let img_ops = parse_image_operations(input);

    let return_same = match img_ops.new_format {
        ImageType::Same => true,
        _ => false,
    };

    if return_same {
        log("Returning the same");
        return Uint8Array::from(bytes);
    }

    let buff = Cursor::new(bytes);

    // Step 1, get the image as a Dynamic image
    let image_result = read_image_bytes(buff);
    let image = match image_result {
        Ok(image) => image,
        Err(err) => {
            error(format!("Error Opening Image: {:?}", err).as_str());
            panic!();
        }
    };

    return process_image_ops(&image, &img_ops);
}

#[wasm_bindgen]
pub fn process_heif_image(bytes: &[u8], width: u32, height: u32, input: JsValue) -> Uint8Array {
    let mut img_ops = parse_image_operations(input);
    img_ops.new_format = match img_ops.new_format {
        ImageType::Same => ImageType::Jpeg,
        _ => img_ops.new_format,
    };
    // if image_operations.new_format == ImageType::Same {
    //     image_operations.new_format = ImageType::Jpeg;
    // }

    let pixels = bytes.to_vec();
    let buf_option = RgbImage::from_raw(width, height, pixels);

    let buf = match buf_option {
        None => {
            error("Image Buffer reading failed");
            panic!();
        }
        Some(buf) => buf,
    };

    let image = DynamicImage::ImageRgb8(buf);

    return process_image_ops(&image, &img_ops);
}

fn process_image_ops(image: &DynamicImage, image_operations: &ImageOptions) -> Uint8Array {
    // Step 2, process the image's size
    let resized_img = resize_image(&image, image_operations);

    // Step 3, Export the image as a new format
    let conversion_result = write_image(resized_img, image_operations);
    let conversion = match conversion_result {
        Ok(image) => image,
        Err(err) => {
            let msg = format!("Error Converting Image: {:?}", err);
            error(msg.as_str());
            panic!();
        }
    };

    // Step 4, Return the data
    let arr = Uint8Array::from(conversion.as_slice());

    return arr;
}

fn resize_image(image: &DynamicImage, img_ops: &ImageOptions) -> DynamicImage {
    if img_ops.max_size == 0 {
        return image.clone();
    }

    let img = image.resize(
        img_ops.max_size as u32,
        img_ops.max_size as u32,
        image::imageops::Lanczos3,
    );

    return img;
}

fn write_image(image: DynamicImage, img_ops: &ImageOptions) -> Result<Vec<u8>, ImageError> {
    let result: Result<Vec<u8>, ImageError> = match img_ops.new_format {
        ImageType::Png => write_image_with_format(image, image::ImageOutputFormat::Png),
        ImageType::Jpeg => {
            write_image_with_format(image, image::ImageOutputFormat::Jpeg(img_ops.image_quality))
        }
        ImageType::Gif => write_image_with_format(image, image::ImageOutputFormat::Gif),
        ImageType::Bmp => write_image_with_format(image, image::ImageOutputFormat::Bmp),
        ImageType::Tiff => write_image_with_format(image, image::ImageOutputFormat::Tiff),
        ImageType::Same => Ok(image.into_bytes()),
    };

    return result;
}

fn write_image_with_format(
    image: DynamicImage,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, ImageError> {
    let mut dat: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut dat), format)?;

    Ok(dat)
}

fn read_image_bytes(bytes: Cursor<&[u8]>) -> Result<DynamicImage, ImageError> {
    let image = ImageReader::new(bytes).with_guessed_format()?.decode()?;

    let width = "Width: ".to_string() + &image.width().to_string();
    let height = "Height: ".to_string() + &image.height().to_string();
    log(width.as_str());
    log(height.as_str());

    return Ok(image);
}

fn parse_image_operations(input: JsValue) -> ImageOptions {
    let parsed_value: ImageOptionsInput = serde_wasm_bindgen::from_value(input).unwrap();

    let new_format_str = match parsed_value.new_format {
        Some(x) => x,
        None => "".to_string(),
    };

    let new_format = match new_format_str.as_str() {
        "png" => ImageType::Png,
        "jpeg" => ImageType::Jpeg,
        "gif" => ImageType::Gif,
        "bmp" => ImageType::Bmp,
        "tiff" => ImageType::Tiff,
        _ => ImageType::Same,
    };

    let max_size = match parsed_value.max_size {
        Some(x) => x.round() as u32,
        None => 0,
    };

    let image_quality = clamp(
        match parsed_value.image_quality {
            Some(x) => x.round() as u8,
            None => 50,
        },
        0,
        100,
    );

    let image_options = ImageOptions {
        new_format,
        max_size,
        image_quality,
    };

    // log(("New Format: ".to_string() + &new_format_str).as_str());
    // log(("Image Identifier: ".to_string() + &image_options.identifier.to_string()).as_str());
    // log(("Do Not Convert: ".to_string() + &image_options.do_not_convert.to_string()).as_str());
    // log(("max size: ".to_string() + &image_options.max_size.to_string()).as_str());
    // log(("image quality: ".to_string() + &image_options.image_quality.to_string()).as_str());

    return image_options;
}
