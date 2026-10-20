pub fn generate_titlebar_texture(width: u32, height: u32, bg_color: [u8; 4]) -> Vec<u8> {
    let mut buf = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            buf[idx] = bg_color[0];
            buf[idx + 1] = bg_color[1];
            buf[idx + 2] = bg_color[2];
            buf[idx + 3] = bg_color[3];
        }
    }

    let btn_size: u32 = (height as f32 * 0.5) as u32;
    let btn_gap: u32 = (height as f32 * 0.25) as u32;
    let btn_y = (height - btn_size) / 2;

    let colors: [[u8; 4]; 3] = [
        [0x90, 0x90, 0x90, 0xFF],
        [0x70, 0x90, 0x70, 0xFF],
        [0xC0, 0x50, 0x50, 0xFF],
    ];

    for (i, color) in colors.iter().enumerate() {
        let btn_x = width.saturating_sub((btn_gap + btn_size) * (i as u32 + 1));
        for by in 0..btn_size {
            for bx in 0..btn_size {
                let x = btn_x + bx;
                let y = btn_y + by;
                if x < width && y < height {
                    let idx = ((y * width + x) * 4) as usize;
                    buf[idx] = color[0];
                    buf[idx + 1] = color[1];
                    buf[idx + 2] = color[2];
                    buf[idx + 3] = color[3];
                }
            }
        }
    }

    buf
}

pub fn titlebar_button_rects(width: i32, height: i32) -> [(i32, i32, i32, i32); 3] {
    let btn_size = (height as f32 * 0.5) as i32;
    let btn_gap = (height as f32 * 0.25) as i32;
    let btn_y = (height - btn_size) / 2;

    let mut rects = [(0, 0, 0, 0); 3];
    for i in 0..3 {
        let btn_x = (width - (btn_gap + btn_size) * (i as i32 + 1)).max(0);
        rects[i] = (btn_x, btn_y, btn_size, btn_size);
    }
    rects
}
