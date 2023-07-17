use chrono::prelude::*;
use notan::prelude::*;
use notan::text::*;
use weather_util_rust::config::Config;
use weather_util_rust::weather_api::WeatherApi;
use weather_util_rust::weather_api::WeatherLocation;

const FONT_SIZE: f32 = 128.0;
const PADDING: f32 = 32.0;
const DATE_TIME_FREQ: f32 = 0.1;
const WEATHER_FREQ: f32 = 30.0 * 60.0;

#[derive(AppState)]
struct State {
    runtime: tokio::runtime::Runtime,
    font: Font,
    date_time: String,
    date_time_count: f32,
    weather: Weather,
    weather_count: f32,
}

#[derive(Clone)]
struct Weather {
    weather_api: WeatherApi,
    location: WeatherLocation,
    temp: String,
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
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    let weather_config = Config::init_config(None).unwrap();
    let weather_api = WeatherApi::new(
        &weather_config.api_key.clone().unwrap(),
        &weather_config.api_endpoint,
        &weather_config.api_path,
        &weather_config.geo_path,
    );
    let location = WeatherLocation::ZipCode {
        zipcode: 20906,
        country_code: Some(isocountry::CountryCode::USA),
    };

    State {
        runtime,
        font,
        date_time: String::from("?"),
        date_time_count: DATE_TIME_FREQ,
        weather: Weather {
            weather_api,
            location,
            temp: String::from("? °F"),
        },
        weather_count: WEATHER_FREQ,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.date_time_count += app.timer.delta_f32();

    if state.date_time_count >= DATE_TIME_FREQ {
        state.date_time_count = 0.0;
        state.date_time = Local::now().format("%A %B %d,  %I:%M:%S %p").to_string();
    }

    if state.weather_count >= WEATHER_FREQ {
        state.weather_count = 0.0;
        println!("fetching weather data...");
        let weather = state.weather.clone();
        state.runtime.spawn(async move {
            let weather_data = weather
                .weather_api
                .get_weather_data(&weather.location)
                .await;
            if let Ok(weather_data) = weather_data {
                println!("{:?}", weather_data);
            } else {
                println!("error fetching weather data :(");
            }
        });
        // state.weather.temp = ;
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

    text.add(&state.weather.temp)
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
