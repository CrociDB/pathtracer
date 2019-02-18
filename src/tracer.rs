pub fn trace(width:u32, height:u32) -> Vec<i32> {
    let mut pixel_data = Vec::new();

    for i in (height - 1)..0 {
        for j in 0..width {
            pixel_data.push(0);
        }
    }

    pixel_data
}
