use crate::state::*;
use notan::prelude::*;
use notan::text::*;

pub const COLOR_STOCK_DOWN: Color   = Color::new(0.913725, 0.301961, 0.301961, 1.0);
pub const COLOR_STOCK_UP: Color     = Color::new(0.560784, 0.905882, 0.741176, 1.0);
pub const COLOR_BKG: Color          = Color::new(0.203922, 0.192157, 0.254902, 1.0);
pub const COLOR_VIOLET: Color       = Color::new(0.545098, 0.623529, 0.933333, 1.0);
// pub const COLOR_BKG_PURPL: Color = Color::new(0.294118, 0.254902, 0.533333, 1.0);
// pub const COLOR_FRG_PURPL: Color = Color::new(0.333333, 0.317647, 0.607843, 1.0);
pub const COLOR_GREY: Color         = Color::new(0.862745, 0.862745, 0.862745, 1.0);
pub const COLOR_GREEN: Color        = Color::new(0.560784, 0.905882, 0.741176, 1.0);




pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, _) = gfx.size();
    let cx = (width as f32) / 2.0;

    let mut text: Text<'_> = gfx.create_text();
    text.clear_color(COLOR_BKG);

    text.add(&state.date_time)
        .font(&state.font)
        .position(PADDING, PADDING)
        .color(COLOR_VIOLET)
        .size(FONT_SIZE);

    let weather = if let Ok(weather_results) = state.weather_results.try_lock() {
        weather_results.clone()
    } else {
        WeatherResults::new()
    };
    //current temp
    text.add(&weather.temp)
        .font(&state.font)
        .position(PADDING, PADDING * 2.0 + FONT_SIZE)
        .color(COLOR_GREEN)
        .size(FONT_SIZE);
    //temperature range
    text.add(&weather.temp_range)
        .font(&state.font)
        .position(PADDING, PADDING * 3.0 + FONT_SIZE * 2.0)
        .color(COLOR_GREY)
        .size(FONT_SIZE);
    //humidity
    text.add(&weather.hum)
        .font(&state.font)
        .position(PADDING, PADDING * 4.0 + FONT_SIZE * 3.0)
        .color(COLOR_GREY)
        .size(FONT_SIZE);
    //weather conditions
    text.chain(&weather.cond)
        .font(&state.font)
        .color(COLOR_GREEN)
        .size(FONT_SIZE);

    let stock_results = state.stock_results.lock().unwrap();
    text.add("\n")
        .font(&state.font)
        .position(cx, PADDING * 2.0)
        .color(Color::GRAY)
        .size(FONT_SIZE);
    if stock_results.stocks.is_empty() {
        text.chain("Stocks\n")
            .font(&state.font)
            .color(Color::GRAY)
            .size(FONT_SIZE);
    } else {
        for stock in &stock_results.stocks {
            let color = if stock.is_up {
                COLOR_STOCK_UP
            } else {
                COLOR_STOCK_DOWN
            };
            text.chain(&stock.display)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE);
        }
    }

    gfx.render(&text);
}
