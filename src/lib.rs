use wasm_bindgen::prelude::*;
use web_sys::console;
use base64::{ encode, decode };
use image::load_from_memory;
use image::ImageOutputFormat::Png;

// #[wasm_bindgen::predlude::wasm_bindgen]
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    console::log_1(&JsValue::from_str("Grascale Called"));

    let base64_to_vector = decode(encoded_file).unwrap();
    console::log_1(&JsValue::from_str("Image Decoded"));

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    console::log_1(&JsValue::from_str("Image Loaded"));

    img = img.grayscale();
    console::log_1(&JsValue::from_str("Grayscale effect applied"));

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    console::log_1(&JsValue::from_str("New Image Written"));

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}