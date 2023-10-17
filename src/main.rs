use notan::draw::DrawConfig;
use notan::prelude::*;
use notan::text::*;

mod consts;
mod draw;
mod state;
mod update;
mod utils;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .resizable(true)
        .maximized(true)
        .fullscreen(false);
    notan::init_with(state::setup)
        .add_config(win_config)
        .add_config(TextConfig)
        .add_config(DrawConfig)
        .update(update::update)
        .draw(draw::draw)
        .build()
}
