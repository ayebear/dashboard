use crate::state::*;
use notan::prelude::*;
use notan::text::*;

pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, _) = gfx.size();
    let cx = (width as f32) / 2.0;

    let mut text = gfx.create_text();
    text.clear_color(Color::BLACK);

    text.add(&state.date_time)
        .font(&state.font)
        .position(PADDING, PADDING)
        .color(Color::ORANGE)
        .size(FONT_SIZE);

    let weather = if let Ok(weather_results) = state.weather_results.try_lock() {
        weather_results.clone()
    } else {
        WeatherResults::new()
    };
    text.add(&weather.temp)
        .font(&state.font)
        .position(PADDING, PADDING * 2.0 + FONT_SIZE)
        .color(Color::AQUA)
        .size(FONT_SIZE);
    text.add(&weather.temp_range)
        .font(&state.font)
        .position(PADDING, PADDING * 3.0 + FONT_SIZE * 2.0)
        .color(Color::GRAY)
        .size(FONT_SIZE);
    text.add(&weather.hum)
        .font(&state.font)
        .position(PADDING, PADDING * 4.0 + FONT_SIZE * 3.0)
        .color(Color::BLUE)
        .size(FONT_SIZE);
    text.chain(&weather.cond)
        .font(&state.font)
        .color(Color::YELLOW)
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
                Color::GREEN
            } else {
                Color::RED
            };
            text.chain(&stock.display)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE);
        }
    }

    gfx.render(&text);
}
