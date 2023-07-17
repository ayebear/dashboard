use chrono::prelude::*;
use notan::prelude::*;
use notan::text::*;

#[derive(AppState)]
struct State {
    font: Font,
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
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    State { font }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let date_as_string = Utc::now().to_string();
    let (width, height) = gfx.size();
    let (cw, ch) = ((width as f32) / 2.0, (height as f32) / 2.0);

    let mut text = gfx.create_text();
    text.clear_color(Color::BLACK);

    text.add(&date_as_string)
        .font(&state.font)
        .position(cw, ch - 64.0)
        .h_align_center()
        .color(Color::ORANGE)
        .size(128.0);

    gfx.render(&text);
}
