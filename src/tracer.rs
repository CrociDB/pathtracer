pub fn trace(width:u32, height:u32) -> Vec<u32> {
    let mut pixel_data = Vec::new();

    for i in 0..height {
        for j in 0..width {
            pixel_data.push(0xFFFFFF);
        }
    }

    pixel_data
}
