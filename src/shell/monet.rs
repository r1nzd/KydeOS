use image::GenericImageView;

pub fn extract_accent_color(image_path: &str) -> Option<[u8; 3]> {
    let img = image::open(image_path).ok()?;
    let (width, height) = img.dimensions();

    let sample_step = ((width * height) / 20000).max(1);

    let mut best_color: Option<[u8; 3]> = None;
    let mut best_score: f32 = -1.0;

    let mut idx: u32 = 0;
    for pixel in img.pixels() {
        if idx % sample_step != 0 {
            idx += 1;
            continue;
        }
        idx += 1;

        let (_x, _y, rgba) = pixel;
        let r = rgba[0] as f32 / 255.0;
        let g = rgba[1] as f32 / 255.0;
        let b = rgba[2] as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let lightness = (max + min) / 2.0;
        let saturation = if max == min {
            0.0
        } else if lightness < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        let lightness_penalty = 1.0 - (lightness - 0.5).abs() * 2.0;
        let score = saturation * lightness_penalty.max(0.1);

        if score > best_score {
            best_score = score;
            best_color = Some([rgba[0], rgba[1], rgba[2]]);
        }
    }

    if best_score < 0.05 {
        return Some([0x67, 0x50, 0xA4]);
    }

    best_color
}

pub fn color_to_hex(color: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
}
