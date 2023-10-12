use crate::state::*;
use notan::prelude::*;
use notan::text::*;
// use notan::draw::*;

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
        .font(&state.symbol_font)
        .position(cx - PADDING, 100.0 +PADDING + FONT_SIZE_L)
        .color(COLOR_GREY)
        .h_align_center()
        .size(FONT_SIZE_S);
    
    
    let mut y_pos = 100.0 + PADDING + FONT_SIZE_L + 2.0 * FONT_SIZE_M;
    let y_pos_stocks = y_pos;
    // header for Weather saying the condition:
    //weather preamble
    let weather = if let Ok(weather_results) = state.weather_results.try_lock() {
        weather_results.clone()
    } else {
        WeatherResults::new()
    };
    //weather condition:
    //x pos is center between 2cx/5 and 4cx/5
    //0.8 - 0.4 = 0.4
    //x pos is 0.4*cx, centered
    text.add(&weather.cond)
            .font(&state.font)
            .position(0.5*cx, y_pos)
            .h_align_center()
            .color(COLOR_GREEN)
            .size(FONT_SIZE_M);

    y_pos += FONT_SIZE_M + PADDING;
    //1/2 of screen for weather
    let mut i = 0.0;
    for wtext in &state.weather_text {
        let color = if i == 0.0 {
            COLOR_GREEN
        } else {
            COLOR_GREY
        };
        text.add(wtext)
            .font(&state.font)
            .position(cx/5.0, y_pos + i * FONT_SIZE_S)
            .color(color)
            .size(FONT_SIZE_S);
        i += 1.0;
    }
    //weather stats
    let temp_text: [&String; 5] = [&weather.temp_f, &weather.temp,&weather.temp_h,&weather.temp_l,&weather.hum];
    i = 0.0;
    for wtext in temp_text {
        let color = if i == 0.0 {
            COLOR_GREEN
        } else {
            COLOR_GREY
        };
        text.add(wtext)
            .font(&state.font)
            .position(4.0 * cx/5.0, y_pos + i * FONT_SIZE_S)
            .h_align_right()
            .color(color)
            .size(FONT_SIZE_S);
        i += 1.0;
    }


    //STOCKS HERE:
    let stock_results = state.stock_results.lock().unwrap();
    text.add("Stonks\n")
        .font(&state.font)
        .position(cx, y_pos_stocks)
        .h_align_left()
        .color(Color::GRAY)
        .size(FONT_SIZE_M);
    y_pos = y_pos_stocks + FONT_SIZE_M;
    if stock_results.stocks.is_empty() {
        text.add("")
            .font(&state.symbol_font)
            .position(cx, y_pos)
            .size(FONT_SIZE_M);
        text.chain("GOOG  $123.67  02.23%\n")
            .font(&state.font)
            .color(COLOR_STOCK_UP)
            .size(FONT_SIZE_M);
        text.chain("can not fetch new data\n")
            .font(&state.font)
            .color(Color::GRAY)
            .size(FONT_SIZE_S);
    } else {
        for stock in &stock_results.stocks {
            let color = if stock.is_up {
                COLOR_STOCK_UP
            } else {
                COLOR_STOCK_DOWN
            };
            // let _space_num: usize= 6 - &stock.symbol.len();
            text.chain(&stock.display)
                .font(&state.font)
                .color(color)
                .size(FONT_SIZE_M);
            // text.chain(&stock.symbol)
            //     .font(&state.font)
            //     .color(color)
            //     .size(FONT_SIZE_M);
            
            // for i in 1..5 {
            //     text.chain("x")
            //         .font(&state.font)
            //         .color(COLOR_BKG)
            //         .size(FONT_SIZE_M);
            //     println!("{}", i);
            // }
            // text.chain(&stock.price)
            //     .font(&state.font)
            //     .color(color)
            //     .size(FONT_SIZE_M);
            // text.chain(&stock.percent)
            //     .font(&state.font)
            //     .color(color)
            //     .size(FONT_SIZE_M);
        }
    }

    gfx.render(&text);
    
}
