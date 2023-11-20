use crate::{consts::*, state::*, utils::metric_time};
use chrono::prelude::*;
use futures::future;
use itertools::join;
use notan::prelude::*;

pub fn update(app: &mut App, state: &mut State) {
    let dt = app.timer.delta_f32();
    state.date_time_count += dt;
    state.weather_count += dt;
    state.stock_count += dt;

    // Consume state updates
    while let Ok(event) = state.rx.try_recv() {
        match event {
            ChangeState::Stock(stock) => {
                state
                    .stock_results
                    .stocks
                    .insert(stock.symbol.clone(), stock);
            }
            ChangeState::Weather(weather) => {
                state.weather_results = weather;
            }
        }
    }

    if state.date_time_count >= DATE_TIME_FREQ {
        state.date_time_count = 0.0;
        state.date = Local::now().format("%A, %B %d").to_string();
        state.time = Local::now().format("%I:%M:%S %p").to_string();
        let (y, d, h, m, s) = metric_time();
        state.metric_time = format!(
            "{} — Day {}, {:01}:{:02}:{:02} Metric Time",
            y,
            d,
            h,
            m,
            s.floor()
        );
    }

    if state.weather_count >= WEATHER_FREQ {
        state.weather_count = 0.0;
        println!("fetching weather data...");
        let tx = state.tx.clone();
        let weather_fetch = state.weather_fetch.clone();
        state.runtime.spawn(async move {
            let weather_data = weather_fetch
                .weather_api
                .get_weather_data(&weather_fetch.location)
                .await;
            if let Ok(weather_data) = weather_data {
                println!("{:?}", weather_data);
                //temperature format
                let temp = format!("{:.2}°F", weather_data.main.temp.fahrenheit());
                let temp_f = format!("~{:.2}°F", weather_data.main.feels_like.fahrenheit());
                let temp_l = format!("{:.2}°F", weather_data.main.temp_min.fahrenheit());
                let temp_h = format!("{:.2}°F", weather_data.main.temp_max.fahrenheit());
                let hum = format!("{}%", weather_data.main.humidity);
                let cond = join(weather_data.weather.iter().map(|cond| &cond.main), " ");
                tx.send(ChangeState::Weather(WeatherResults {
                    temp,
                    temp_f,
                    temp_l,
                    temp_h,
                    hum,
                    cond,
                }))
                .unwrap();
            } else {
                println!("error fetching weather data :(");
            }
        });
    }

    if state.stock_count >= STOCK_FREQ {
        state.stock_count = 0.0;
        println!("\nupdate stocks");
        let stock_results = state.stock_results.clone();
        let stocks_api_key = state.stocks_api_key.clone();
        let stocks = state.stocks.clone();
        let tx = state.tx.clone();
        state.runtime.spawn(async move {
            // Get stocks
            let av = alpha_vantage::set_api(stocks_api_key, reqwest::Client::new());
            let calls = stocks.into_iter().map(|ticker| {
                let av = &av;
                async move { av.quote(&ticker).json().await }
            });
            let results = future::join_all(calls).await;

            // Store stock results
            for result in results.into_iter().flatten() {
                let is_up = result.change_percent().is_sign_positive();
                tx.send(ChangeState::Stock(Stock {
                    //put symbol, price, percent into individual strings: draws will update the necessary
                    //space between them accordingly.
                    symbol: result.symbol().to_string(),
                    price: format!("{:.2}", result.price()),
                    percent: format!("   {:.2}%\n", result.change_percent()),
                    is_up,
                }))
                .unwrap();
            }
            println!("\nstocks updated {}", stock_results.stocks.len());
        });
    }
}
