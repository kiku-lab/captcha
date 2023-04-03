use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use wasm_bindgen::prelude::*;

fn generate(text: String) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::from_pixel(512, 256, Rgb::<u8>([255, 255, 255]));
    let font_size = 128;
    let font_data = include_bytes!("./captcha.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let color = Rgb([0, 0, 0]);
    let scale = Scale {
        x: font_size as f32,
        y: font_size as f32,
    };
    draw_text_mut(&mut img, color, 128, 64, scale, &font, &text);
    // Add some noise
    for _ in 0..1000 {
        let x = rand::random::<u32>() % 512;
        let y = rand::random::<u32>() % 256;
        img.put_pixel(x, y, Rgb([0, 0, 0]));
    }
    // Add distortion to text
    for _ in 0..10 {
        let x = rand::random::<u32>() % 512;
        let y = rand::random::<u32>() % 256;
        let x2 = rand::random::<u32>() % 512;
        let y2 = rand::random::<u32>() % 256;
        let color = img.get_pixel(x, y);
        img.put_pixel(x2, y2, *color);
    }
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
        img.save("sample.png").unwrap();
        assert_eq!(result, 4);
    }
}
