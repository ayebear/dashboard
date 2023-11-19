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

/*
pub const COLOR_STOCK_DOWN: Color   = Color::new(0.913725, 0.301961, 0.301961, 1.0);
pub const COLOR_STOCK_UP: Color     = Color::new(0.560784, 0.905882, 0.741176, 1.0);
pub const COLOR_BKG: Color          = Color::new(0.203922, 0.192157, 0.254902, 1.0);
pub const COLOR_VIOLET: Color       = Color::new(0.545098, 0.623529, 0.933333, 1.0);
// pub const COLOR_BKG_PURPL: Color = Color::new(0.294118, 0.254902, 0.533333, 1.0);
// pub const COLOR_FRG_PURPL: Color = Color::new(0.333333, 0.317647, 0.607843, 1.0);
pub const COLOR_GREY: Color         = Color::new(0.862745, 0.862745, 0.862745, 1.0);
pub const COLOR_GREEN: Color        = Color::new(0.560784, 0.905882, 0.741176, 1.0);
*/

pub const FONT_SIZE: f32 = 96.0;
pub const PADDING: f32 = 32.0;
pub const DATE_TIME_FREQ: f32 = 0.1;
pub const WEATHER_FREQ: f32 = 15.0 * 60.0;
pub const STOCK_FREQ: f32 = 60.0 * 60.0;

//for the original dashboard:
pub const FONT_SIZE_L: f32 = 96.0;
pub const FONT_SIZE_M: f32 = 64.0;
pub const FONT_SIZE_S: f32 = 48.0;
// //testing on a smaller screen:
// pub const FONT_SIZE_L: f32 = 64.0;
// pub const FONT_SIZE_M: f32 = 48.0;
// pub const FONT_SIZE_S: f32 = 32.0;
