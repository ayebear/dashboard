use chrono::prelude::*;

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

pub fn metric_time() -> (i32, i32, f64, f64, f64) {
    let local = Local::now();
    let (h, m, s) = (
        local.hour() as f64,
        local.minute() as f64,
        local.second() as f64 + local.timestamp_subsec_micros() as f64 / 1_000_000.,
    );
    let total_metric_s = ((h * 3600. + m * 60. + s) / 86400.) * 100_000.;
    (
        local.year(),
        local.ordinal().try_into().unwrap(),
        (total_metric_s / 10_000.).floor(),
        (total_metric_s % 10_000. / 100.).floor(),
        total_metric_s % 100.,
    )
}
