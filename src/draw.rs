use crate::consts::*;
use crate::state::*;
use notan::prelude::*;
use notan::text::*;

pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();
    let cx = (width as f32) / 2.0;

    let mut text: Text<'_> = gfx.create_text();
    text.clear_color(COLOR_BKG);

    text.add(&state.date_time)
        .font(&state.font)
        .position(PADDING, PADDING)
        .color(COLOR_VIOLET)
        .size(FONT_SIZE);

    let weather = state.weather_results.lock().unwrap();
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
        for stock in stock_results.stocks.values() {
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

    text.add(&state.metric_time)
        .font(&state.font)
        .position(PADDING, height as f32 - PADDING - FONT_SIZE)
        .color(COLOR_GREY)
        .size(FONT_SIZE);

    gfx.render(&text);
}
