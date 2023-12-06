use crate::models::{AirQuality, Location};
use reqwest;
use serde_json;

pub async fn get_air_quality(location: Location) {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid=023356249c1f03f5843292dbc860105a&units=metric",
        location.lat, location.lon,
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        if let Ok(aqi) = serde_json::from_str::<AirQuality>(&text) {
                            display_aqi(&aqi);
                        } else {
                            eprintln!(
                                "Failed to parse air quality data: {:?}",
                                serde_json::from_str::<serde_json::Value>(&text)
                            );
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to read response text: {}", err);
                    }
                }
            } else {
                eprintln!("Failed to fetch air quality data: {}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Failed to make the request: {}", err);
        }
    }
}

fn display_aqi(data: &AirQuality) {
    if let Some(aqi) = data.list.first() {
        println!("<======Air quality data======>");
        println!("AQI: {}", aqi.main.aqi);
        println!("CO: {}", aqi.components.co);
        println!("NO: {}", aqi.components.no);
        println!("NO2: {}", aqi.components.no2);
        println!("O3: {}", aqi.components.o3);
        println!("SO2: {}", aqi.components.so2);
        println!("PM2.5: {}", aqi.components.pm2_5);
        println!("PM10: {}", aqi.components.pm10);
        println!("NH3: {}", aqi.components.nh3);
    } else {
        println!("No air quality data available.");
    }
}
