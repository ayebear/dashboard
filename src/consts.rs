use crate::color_hex;
use notan::prelude::Color;

pub const COLOR_STOCK_DOWN: Color = color_hex!(0xe94d4d);
pub const COLOR_STOCK_UP: Color = color_hex!(0x8fe7bd);
pub const COLOR_BKG: Color = color_hex!(0x343141);
pub const COLOR_VIOLET: Color = color_hex!(0x8b9fee);
// pub const COLOR_BKG_PURPL: Color = color_hex!(0x4b4188);
// pub const COLOR_FRG_PURPL: Color = color_hex!(0x55519b);
pub const COLOR_GREY: Color = color_hex!(0xdcdcdc);
pub const COLOR_GREEN: Color = color_hex!(0x8fe7bd);

pub const FONT_SIZE: f32 = 96.0;
pub const PADDING: f32 = 32.0;
pub const DATE_TIME_FREQ: f32 = 0.1;
pub const WEATHER_FREQ: f32 = 15.0 * 60.0;
pub const STOCK_FREQ: f32 = 60.0 * 60.0;
