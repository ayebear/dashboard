use crate::consts::*;
use crate::state::*;
use notan::prelude::*;
use notan::text::*;

pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();
    let cx = (width as f32) / 2.0;

    let mut text: Text<'_> = gfx.create_text();
    text.clear_color(COLOR_BKG);

    //create heading: date and time, preferrably centered.
    //y pos is PADDING defined earlier. x pos should be closer to center.
    text.add(&state.date)
        .font(&state.font)
        .position(cx, 100.0)
        .h_align_center()
        .color(COLOR_VIOLET)
        .size(FONT_SIZE_L);
    //add time
    text.add(&state.time)
        .font(&state.font)
        .position(cx - PADDING, 100.0 + PADDING + FONT_SIZE_L)
        .color(COLOR_VIOLET)
        .h_align_center()
        .size(FONT_SIZE_L);

    let mut y_pos = 100.0 + PADDING + FONT_SIZE_L + 2.0 * FONT_SIZE_M;
    let y_pos_stocks = y_pos;
    let weather = state.weather_results.lock().unwrap();
    //weather condition:
    //x pos is center between 2cx/5 and 4cx/5
    //0.8 - 0.4 = 0.4
    //x pos is 0.4*cx, centered
    text.add(&weather.cond)
        .font(&state.font)
        .position(0.5 * cx, y_pos)
        .h_align_center()
        .color(COLOR_GREEN)
        .size(FONT_SIZE_M);

    y_pos += FONT_SIZE_M + PADDING;
    // Weather text and data
    let weather_items = [
        ("feels", &weather.temp_f, COLOR_GREEN),
        ("temp", &weather.temp, COLOR_GREY),
        ("high", &weather.temp_h, COLOR_GREY),
        ("low", &weather.temp_l, COLOR_GREY),
        ("humidity", &weather.hum, COLOR_GREY),
    ];
    for (i, (wtext, wdata, color)) in weather_items.iter().enumerate() {
        let y = y_pos + i as f32 * FONT_SIZE_M;
        // Weather text
        text.add(wtext)
            .font(&state.font)
            .position(cx / 5.0, y)
            .color(*color)
            .size(FONT_SIZE_M);
        // Weather data
        text.add(wdata)
            .font(&state.font)
            .position(4.0 * cx / 5.0, y)
            .h_align_right()
            .color(*color)
            .size(FONT_SIZE_M);
    }

    //STOCKS HERE:
    let stock_results = state.stock_results.lock().unwrap();
    text.add("STONKS\n")
        .font(&state.font)
        .position(cx, y_pos_stocks)
        .h_align_left()
        .color(Color::GRAY)
        .size(FONT_SIZE_M);
    y_pos = y_pos_stocks + FONT_SIZE_M + PADDING;
    text.add("")
        .font(&state.symbol_font)
        .position(cx, y_pos)
        .size(FONT_SIZE_M);
    if stock_results.stocks.is_empty() {
        text.chain("can not fetch new stock data\n")
            .font(&state.font)
            .color(Color::GRAY)
            .size(FONT_SIZE_S);
    } else {
        for stock in stock_results.stocks.values() {
            let color = match stock.is_up {
                true => COLOR_STOCK_UP,
                false => COLOR_STOCK_DOWN,
            };
            text.chain(&stock.symbol)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE_M);

            let space_num: usize = 8 - stock.symbol.len();
            for _i in 1..space_num {
                text.chain("x")
                    .font(&state.font)
                    .color(COLOR_BKG)
                    .size(FONT_SIZE_M);
            }

            //add leading zeros to stock price
            text.chain("$")
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE_M);

            let space_num: usize = 8 - stock.price.len();
            for _i in 1..space_num {
                text.chain("0")
                    .font(&state.font)
                    .color(Color::GRAY)
                    .size(FONT_SIZE_M);
            }

            text.chain(&stock.price)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE_M);

            text.chain(&stock.percent)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE_M);
        }
    }

    text.add(&state.metric_time)
        .font(&state.font)
        .position(PADDING, height as f32 - PADDING - FONT_SIZE)
        .color(COLOR_GREY)
        .size(FONT_SIZE);

    gfx.render(&text);
}
