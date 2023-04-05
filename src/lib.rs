use image::{ImageBuffer, ImageEncoder, Rgb};
use imageproc::drawing::draw_text_mut;
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
    // 画像に歪みを加える
    for y in 0..128 {
        for x in 0..256 {
            let dx = rng.gen_range(-10..10) as i32;
            let dy = rng.gen_range(-10..10) as i32;
            let new_x = (x as i32 + dx).clamp(0, 256 as i32 - 1) as u32;
            let new_y = (y as i32 + dy).clamp(0, 128 as i32 - 1) as u32;
            let pixel = img.get_pixel(new_x, new_y);
            img.put_pixel(x, y, *pixel);
        }
    }
    img
}

fn convert_to_bytes(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder
        .write_image(&img, 256, 128, image::ColorType::Rgb8)
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
