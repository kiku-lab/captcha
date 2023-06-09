use image::{codecs::png::PngEncoder, ColorType, ImageBuffer, ImageEncoder, Rgb};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rand::Rng;
use rusttype::{Font, Scale};
use wasm_bindgen::prelude::*;

fn generate(text: String) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::from_pixel(256, 128, Rgb::<u8>([255, 255, 255]));
    let font_size = 64;
    let font_data = include_bytes!("./captcha.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let color = Rgb([0, 0, 0]);
    let scale = Scale {
        x: font_size as f32,
        y: font_size as f32,
    };
    let mut last_x = 32;
    let mut rng = rand::thread_rng();
    for c in text.chars() {
        let y = rng.gen_range(1..85);
        draw_text_mut(&mut img, color, last_x, y, scale, &font, &c.to_string());
        last_x += 32;
    }
    // Add some noise
    for _ in 0..1000 {
        let x = rand::random::<u32>() % 256;
        let y = rand::random::<u32>() % 128;
        img.put_pixel(x, y, Rgb([0, 0, 0]));
    }
    // Add some lines
    for _ in 0..10 {
        let x1 = rand::random::<u32>() % 256;
        let y1 = rand::random::<u32>() % 128;
        let x2 = rand::random::<u32>() % 256;
        let y2 = rand::random::<u32>() % 128;
        draw_line_segment_mut(
            &mut img,
            (x1 as f32, y1 as f32),
            (x2 as f32, y2 as f32),
            Rgb([0, 0, 0]),
        );
    }
    img
}

fn convert_to_bytes(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut png_data = Vec::new();
    let encoder = PngEncoder::new(&mut png_data);
    encoder
        .write_image(&img, 256, 128, ColorType::Rgb8)
        .unwrap();
    png_data
}

#[wasm_bindgen]
pub fn generate_image(text: String) -> Vec<u8> {
    let img = generate(text);
    convert_to_bytes(img)
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
        convert_to_bytes(img.clone());
        img.save("sample.png").unwrap();
        assert_eq!(result, 4);
    }
}
