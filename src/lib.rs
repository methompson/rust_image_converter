use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::{DynamicImage, RgbImage};
use js_sys::Uint8Array;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn read_bytes(bytes: &[u8]) -> Uint8Array {
    log("Inside the read_bytes function");
    log(bytes.len().to_string().as_str());

    let buff = Cursor::new(bytes);

    let image_result = read_image_bytes(buff);
    let image = match image_result {
        Ok(image) => image,
        Err(err) => {
            let msg = format!("Error Opening Image: {:?}", err);
            error(msg.as_str());
            panic!();
        }
    };

    let conversion_result = write_image_to_jpeg(image);
    let conversion = match conversion_result {
        Ok(image) => image,
        Err(err) => {
            let msg = format!("Error Converting Image: {:?}", err);
            error(msg.as_str());
            panic!();
        }
    };

    let arr = Uint8Array::from(conversion.as_slice());

    return arr;
}

#[wasm_bindgen]
pub fn create_image_from_rgb(bytes: &[u8], height: u32, width: u32) -> Uint8Array {
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

    let conversion_result = write_image_to_jpeg(image);
    let conversion = match conversion_result {
        Ok(image) => image,
        Err(err) => {
            let msg = format!("Error Converting Image: {:?}", err);
            error(msg.as_str());
            panic!();
        }
    };

    let arr = Uint8Array::from(conversion.as_slice());

    return arr;
}

pub fn read_image_bytes(bytes: Cursor<&[u8]>) -> Result<DynamicImage, ImageError> {
    let image = ImageReader::new(bytes).with_guessed_format()?.decode()?;

    let width = "Width: ".to_string() + &image.width().to_string();
    let height = "Height: ".to_string() + &image.height().to_string();
    log(width.as_str());
    log(height.as_str());

    return Ok(image);
}

pub fn write_image_to_jpeg(image: DynamicImage) -> Result<Vec<u8>, ImageError> {
    let mut dat: Vec<u8> = Vec::new();
    image.write_to(
        &mut Cursor::new(&mut dat),
        image::ImageOutputFormat::Jpeg(50),
    )?;

    Ok(dat)
}
