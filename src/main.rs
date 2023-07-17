use chrono::prelude::*;
use dotenvy::dotenv;
use itertools::join;
use notan::prelude::*;
use notan::text::*;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use weather_util_rust::config::Config;
use weather_util_rust::weather_api::WeatherApi;
use weather_util_rust::weather_api::WeatherLocation;

const FONT_SIZE: f32 = 128.0;
const PADDING: f32 = 32.0;
const DATE_TIME_FREQ: f32 = 0.1;
const WEATHER_FREQ: f32 = 15.0 * 60.0;
const STOCK_FREQ: f32 = 60.0 * 60.0;

#[derive(AppState)]
struct State {
    runtime: tokio::runtime::Runtime,
    font: Font,
    date_time: String,
    date_time_count: f32,
    weather_fetch: Arc<tokio::sync::Mutex<WeatherFetch>>,
    weather_results: Arc<Mutex<WeatherResults>>,
    weather_count: f32,
    stocks: Vec<String>,
    stocks_api_key: String,
    stock_results: Arc<Mutex<StockResults>>,
    stock_count: f32,
}

struct WeatherFetch {
    weather_api: WeatherApi,
    location: WeatherLocation,
}

#[derive(Clone)]
struct WeatherResults {
    temp: String,
    temp_range: String,
    hum: String,
    cond: String,
}

impl WeatherResults {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for WeatherResults {
    fn default() -> Self {
        WeatherResults {
            temp: String::from("?°F"),
            temp_range: String::from("[?—?°F]"),
            hum: String::from("?%"),
            cond: String::from("  ???"),
        }
    }
}

#[derive(Default, Clone)]
struct StockResults {
    stocks: Vec<Stock>,
}

#[derive(Default, Clone)]
struct Stock {
    display: String,
    is_up: bool,
}

impl StockResults {
    fn new() -> Self {
        Default::default()
    }
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
    dotenv().expect(".env file not found");
    let stocks_api_key = env::var("STOCKS_API_KEY").unwrap();
    let stocks: Vec<String> = env::var("STOCKS")
        .unwrap()
        .split(',')
        .map(str::to_string)
        .collect();

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
        weather_fetch: Arc::new(tokio::sync::Mutex::new(WeatherFetch {
            weather_api,
            location,
        })),
        weather_results: Arc::new(Mutex::new(WeatherResults::new())),
        weather_count: WEATHER_FREQ - 1.0,
        stocks,
        stocks_api_key,
        stock_results: Arc::new(Mutex::new(StockResults::new())),
        stock_count: STOCK_FREQ - 1.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    let dt = app.timer.delta_f32();
    state.date_time_count += dt;
    state.weather_count += dt;
    state.stock_count += dt;

    if state.date_time_count >= DATE_TIME_FREQ {
        state.date_time_count = 0.0;
        state.date_time = Local::now().format("%A %B %d,  %I:%M:%S %p").to_string();
    }

    if state.weather_count >= WEATHER_FREQ {
        state.weather_count = 0.0;
        println!("fetching weather data...");
        let weather_fetch = state.weather_fetch.clone();
        let weather_results = state.weather_results.clone();
        state.runtime.spawn(async move {
            let weather = weather_fetch.lock().await;
            let weather_data = weather
                .weather_api
                .get_weather_data(&weather.location)
                .await;
            if let Ok(weather_data) = weather_data {
                println!("{:?}", weather_data);
                let mut weather_out = weather_results.lock().unwrap();
                weather_out.temp = format!(
                    "{:.2}°F  ~{:.2}°F",
                    weather_data.main.temp.fahrenheit(),
                    weather_data.main.feels_like.fahrenheit(),
                );
                weather_out.temp_range = format!(
                    "[{:.2}—{:.2}°F]",
                    weather_data.main.temp_min.fahrenheit(),
                    weather_data.main.temp_max.fahrenheit()
                );
                weather_out.hum = format!("{}%", weather_data.main.humidity);
                weather_out.cond = format!(
                    "  {}",
                    join(weather_data.weather.iter().map(|cond| &cond.main), " ")
                );
            } else {
                println!("error fetching weather data :(");
            }
        });
    }

    if state.stock_count >= STOCK_FREQ {
        state.stock_count = 0.0;
        println!("update stocks");
        let stock_results = state.stock_results.clone();
        let stocks_api_key = state.stocks_api_key.clone();
        state.runtime.spawn(async move {
            // Get stocks
            // let client = twelvedata::Client::new(&stocks_api_key);
            // let tsla_price = realtime_price(&stocks_api_key, "TSLA").await;
            let av = alpha_vantage::set_api(stocks_api_key, reqwest::Client::new());
            let tsla = av.quote("TSLA").json().await.unwrap();

            // Store results
            let mut stock_results = stock_results.lock().unwrap();
            stock_results.stocks.clear();
            stock_results.stocks.push(Stock {
                display: format!("TSLA ${} %{}\n", tsla.price(), tsla.change_percent()),
                is_up: tsla.change_percent().is_sign_positive(),
            });
            // stock_results.stocks.push(Stock {
            //     display: "AAPL $0.01\n".into(),
            //     is_up: false,
            // });
            println!("stocks updated");
        });
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

    gfx.render(&text);
}
