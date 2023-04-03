use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use wasm_bindgen::prelude::*;

fn generate(text: String) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(1024, 512);
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255])
    }
    let font_size = 32;
    let font_data = include_bytes!("../fonts/captcha.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let color = Rgb([0, 0, 0]);
    /* 
    let scale = Scale {
        x: font_size as f32,
        y: font_size as f32,
    };
    draw_text_mut(&mut img, color, 10, 10, scale, &font, &text);
    */
    // img.save("hi.png").unwrap();
    img
}

#[wasm_bindgen]
pub fn generate_captcha(text: String) -> Vec<u8> {
    let img = generate(text);
    img.to_vec()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let img = generate("93829".to_string());
        img.save("he.png").unwrap();
        assert_eq!(result, 4);
    }
}
