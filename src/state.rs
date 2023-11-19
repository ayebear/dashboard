use crate::consts::*;
use dotenvy::dotenv;
use notan::prelude::*;
use notan::text::*;
use std::collections::BTreeMap;
use std::env;
use std::sync::{Arc, Mutex};
use weather_util_rust::{
    config::Config,
    weather_api::{WeatherApi, WeatherLocation},
};

#[derive(AppState)]
pub struct State {
    pub runtime: tokio::runtime::Runtime,
    pub font: Font,
    pub date_time: String,
    pub date_time_count: f32,
    pub weather_fetch: Arc<tokio::sync::Mutex<WeatherFetch>>,
    pub weather_results: Arc<Mutex<WeatherResults>>,
    pub weather_count: f32,
    pub stocks: Vec<String>,
    pub stocks_api_key: String,
    pub stock_results: Arc<Mutex<StockResults>>,
    pub stock_count: f32,
}

pub struct WeatherFetch {
    pub weather_api: WeatherApi,
    pub location: WeatherLocation,
}

#[derive(Clone)]
pub struct WeatherResults {
    pub temp: String,
    pub temp_range: String,
    pub hum: String,
    pub cond: String,
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
pub struct StockResults {
    pub stocks: BTreeMap<String, Stock>,
}

#[derive(Default, Clone)]
pub struct Stock {
    pub display: String,
    pub is_up: bool,
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
        .create_font(include_bytes!("../assets/KulimPark-Bold.ttf"))
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
        weather_results: Arc::new(Mutex::new(WeatherResults::default())),
        weather_count: WEATHER_FREQ - 1.0,
        stocks,
        stocks_api_key,
        stock_results: Arc::new(Mutex::new(StockResults::default())),
        stock_count: STOCK_FREQ - 1.0,
    }
}
