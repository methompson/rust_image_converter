use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use js_sys::Uint8Array;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn read_bytes(bytes: &[u8]) -> Uint8Array {
    log("Inside the read_bytes function");
    log(bytes.len().to_string().as_str());

    let buff = Cursor::new(bytes);

    let image_result = read_image_bytes(buff);
    let image = match image_result {
        Ok(image) => image,
        Err(err) => panic!("Error Opening Image: {:?}", err),
    };

    let conversion_result = write_image_to_jpeg(image);
    let conversion = match conversion_result {
        Ok(image) => image,
        Err(err) => panic!("Error Converting Image: {:?}", err),
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
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageOutputFormat::Jpeg(50),
    )?;

    Ok(bytes)
}
