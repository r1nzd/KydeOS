pub fn generate_taskbar_texture(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let radius = (height as f32) / 2.0;
    let mut buf = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;

            let inside = is_inside_pill(fx, fy, width as f32, height as f32, radius);

            let idx = ((y * width + x) * 4) as usize;
            if inside {
                buf[idx] = color[0];
                buf[idx + 1] = color[1];
                buf[idx + 2] = color[2];
                buf[idx + 3] = color[3];
            } else {
                buf[idx] = 0;
                buf[idx + 1] = 0;
                buf[idx + 2] = 0;
                buf[idx + 3] = 0;
            }
        }
    }

    buf
}

fn is_inside_pill(fx: f32, fy: f32, width: f32, height: f32, radius: f32) -> bool {
    if fx >= radius && fx <= width - radius {
        return true;
    }

    let center_x = if fx < radius { radius } else { width - radius };
    let center_y = height / 2.0;

    let dx = fx - center_x;
    let dy = fy - center_y;
    (dx * dx + dy * dy) <= radius * radius
}
