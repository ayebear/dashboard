use chrono::prelude::*;
use notan::prelude::*;
use notan::text::*;

const FONT_SIZE: f32 = 128.0;
const PADDING: f32 = 32.0;
const DATE_TIME_FREQ: f32 = 0.1;

#[derive(AppState)]
struct State {
    font: Font,
    date_time: String,
    date_time_count: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .resizable(true)
        .maximized(true)
        .fullscreen(true);
    notan::init_with(setup)
        .add_config(win_config)
        .add_config(TextConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        date_time: String::from("?"),
        date_time_count: 0.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.date_time_count += app.timer.delta_f32();

    if state.date_time_count >= DATE_TIME_FREQ {
        state.date_time_count = 0.0;
        state.date_time = Local::now().format("%A %B %d,  %I:%M:%S %p").to_string();
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, _) = gfx.size();
    let cx = (width as f32) / 2.0;

    let mut text = gfx.create_text();
    text.clear_color(Color::BLACK);

    text.add(&state.date_time)
        .font(&state.font)
        .position(PADDING, PADDING)
        .color(Color::ORANGE)
        .size(FONT_SIZE);

    text.add("72-90 °F\n82%")
        .font(&state.font)
        .position(PADDING, PADDING * 2.0 + FONT_SIZE)
        .color(Color::AQUA)
        .size(FONT_SIZE);

    text.add("NVDA $542.69\nAMD $157.24\nTSLA $303.89")
        .font(&state.font)
        .position(cx, PADDING * 2.0 + FONT_SIZE)
        .color(Color::GREEN)
        .size(FONT_SIZE);

    gfx.render(&text);
}
