#[macro_export]
macro_rules! color_hex {
    ($h:expr) => {
        Color::from_rgb(
            (($h >> 16) & 0xFF) as f32 / 255.0,
            (($h >> 8) & 0xFF) as f32 / 255.0,
            ($h & 0xFF) as f32 / 255.0,
        )
    };
}
