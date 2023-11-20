use crate::consts::*;
use dotenvy::dotenv;
use notan::prelude::*;
use notan::text::*;
use std::collections::BTreeMap;
use std::env;
use std::sync::mpsc::{channel, Receiver, Sender};
use weather_util_rust::{
    config::Config,
    weather_api::{WeatherApi, WeatherLocation},
};

#[derive(AppState)]
pub struct State {
    pub runtime: tokio::runtime::Runtime,
    pub font: Font,
    pub symbol_font: Font,
    pub date: String,
    pub time: String,
    pub metric_time: String,
    pub date_time_count: f32,
    pub weather_fetch: WeatherFetch,
    pub weather_results: WeatherResults,
    pub weather_count: f32,
    pub stocks: Vec<String>,
    pub stocks_api_key: String,
    pub stock_results: StockResults,
    pub stock_count: f32,
    pub rx: Receiver<ChangeState>,
    pub tx: Sender<ChangeState>,
}

#[derive(Clone)]
pub struct WeatherFetch {
    pub weather_api: WeatherApi,
    pub location: WeatherLocation,
}

#[derive(Clone)]
pub struct WeatherResults {
    pub temp_f: String,
    pub temp: String,
    pub temp_h: String,
    pub temp_l: String,
    pub hum: String,
    pub cond: String,
}

impl Default for WeatherResults {
    fn default() -> Self {
        WeatherResults {
            temp: String::from("?°F"),
            temp_f: String::from("?°F"),
            temp_h: String::from("?°F"),
            temp_l: String::from("?°F"),
            hum: String::from("?%"),
            cond: String::from("???"),
        }
    }
}

#[derive(Default, Clone)]
pub struct StockResults {
    pub stocks: BTreeMap<String, Stock>,
}

#[derive(Default, Clone)]
pub struct Stock {
    pub symbol: String,
    pub price: String,
    pub percent: String,
    pub is_up: bool,
}

#[derive(Clone)]
pub enum ChangeState {
    Stock(Stock),
    Weather(WeatherResults),
}

pub fn setup(app: &mut App, gfx: &mut Graphics) -> State {
    app.window().set_cursor(CursorIcon::None);
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
        .create_font(include_bytes!("../assets/UbuntuMono-Bold.ttf"))
        .unwrap();
    let symbol_font = gfx
        .create_font(include_bytes!("../assets/RubikMonoOne-Regular.ttf"))
        .unwrap();

    let weather_config = Config::init_config(None).unwrap();
    let weather_api = WeatherApi::new(
        &weather_config.api_key.clone().unwrap(),
        &weather_config.api_endpoint,
        &weather_config.api_path,
        &weather_config.geo_path,
    );
    let location = WeatherLocation::ZipCode {
        zipcode: weather_config.zipcode.expect("zipcode not in .env"),
        country_code: Some(isocountry::CountryCode::USA),
    };
    let (tx, rx) = channel();

    State {
        runtime,
        font,
        symbol_font,
        date: String::from("?"),
        time: String::from("?"),
        metric_time: String::from("?"),
        date_time_count: DATE_TIME_FREQ,
        weather_fetch: WeatherFetch {
            weather_api,
            location,
        },
        weather_results: WeatherResults::default(),
        weather_count: WEATHER_FREQ - 1.0,
        stocks,
        stocks_api_key,
        stock_results: StockResults::default(),
        stock_count: STOCK_FREQ - 1.0,
        tx,
        rx,
    }
}
