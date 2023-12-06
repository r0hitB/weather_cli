use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub country: String,
}

//For Weather Forecast api
#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub main: Main,
    pub wind: Wind,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
    #[serde(rename = "3h")]
    pub three_hours: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
    #[serde(rename = "3h")]
    pub three_hours: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
    pub gust: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

//
//
//
//
//
//For air quality api
#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Components {
    pub co: f64,
    pub no: f64,
    pub no2: f64,
    pub o3: f64,
    pub so2: f64,
    pub pm2_5: f64,
    pub pm10: f64,
    pub nh3: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainAQI {
    pub aqi: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AQI {
    pub main: MainAQI,
    pub components: Components,
    pub dt: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirQuality {
    pub coord: Coord,
    pub list: Vec<AQI>,
}
