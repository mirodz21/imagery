
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
#[wasm_bindgen]

pub fn grayscale(encoded_file: &str)-> String{
log(&"grayscale".into());

let base64_to_vector = STANDARD.decode(encoded_file).unwrap();
let mut img = load_from_memory(&base64_to_vector).unwrap();
log(&"image loaded".into());

img = img.grayscale();
log(&"grayscale applied".into());

let mut buffer = vec![];

img.write_to(&mut buffer, Png).unwrap();
log(&"new image".into());

let encoded_img = STANDARD.encode(&buffer);
let data_url = format!(
  "data:image/png;base64,{}", encoded_img
);

data_url

}
