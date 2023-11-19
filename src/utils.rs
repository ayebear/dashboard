use time::OffsetDateTime;

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
    let local = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let (h, m, s, micro) = local.to_hms_micro();
    let (h, m, s) = (h as f64, m as f64, s as f64 + micro as f64 / 1_000_000.);
    let total_metric_s = ((h * 3600. + m * 60. + s) / 86400.) * 100_000.;
    (
        local.year(),
        local.ordinal().into(),
        (total_metric_s / 10_000.).floor(),
        (total_metric_s % 10_000. / 100.).floor(),
        total_metric_s % 100.,
    )
}
