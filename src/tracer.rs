pub fn trace(width:u32, height:u32) -> Vec<u32> {
    let mut pixel_data = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let mut r = i as f32 / height as f32;
            let mut g = j as f32 / width as f32;
            let mut b = 1.0;

            pixel_data.push(pack_colors(
                (r * 255.0) as u8, 
                (g * 255.0) as u8, 
                (b * 255.0) as u8));
        }
    }

    pixel_data
}

pub fn unpack_colors(col:u32) -> (u8, u8, u8) {
    (((col >> 16) & 0xFF) as u8, ((col >> 8) & 0xFF) as u8, (col & 0xFF) as u8)
}

pub fn pack_colors(r:u8, g:u8, b:u8) -> u32 {
    ((r as u32) << 16) + ((g as u32) << 8) + b as u32
}
