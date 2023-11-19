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
        .set_resizable(true)
        .set_maximized(true)
        .set_fullscreen(true);
    notan::init_with(state::setup)
        .add_config(win_config)
        .add_config(TextConfig)
        .update(update::update)
        .draw(draw::draw)
        .build()
}
