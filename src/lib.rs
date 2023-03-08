use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log; // will allow us to log one item

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into()); // into() will auto convert the string to a js value

    // transform into binary
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    // load file
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    // apply grayscale
    img = img.grayscale();
    log(&"grayscale effect applied".into());

    // write to buffer
    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    // encode the image as a string
    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);
    //return the image as a string to js
    data_url
}
