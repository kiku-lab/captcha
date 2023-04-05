use image::{ImageBuffer, Rgb, ImageEncoder};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use wasm_bindgen::prelude::*;
use rand::Rng;

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
    let mut last_x = 128;
    let mut rng = rand::thread_rng();
    for c in text.chars() {
        let y = rng.gen_range(1..63);
        draw_text_mut(&mut img, color, last_x, y, scale, &font, &c.to_string());
        last_x += 60;
    }
    // draw_text_mut(&mut img, color, 128, 64, scale, &font, &text);
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
pub fn generate_image(text: String) -> Vec<u8> {
    let img = generate(text);
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder.write_image(&img, 512, 256, image::ColorType::Rgb8).unwrap();
    png_data
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
