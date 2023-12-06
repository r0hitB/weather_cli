use crate::models::{Forecast, Location};
use reqwest;

pub async fn get_weather(location: Location) {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid=023356249c1f03f5843292dbc860105a&units=metric",
        location.lat, location.lon,
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<Forecast>().await {
                Ok(weather) => {
                    display_forecast(location, weather);
                }
                Err(err) => {
                    eprintln!("Failed to parse weather data: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch weather data: {}", err);
        }
    }
}

fn display_forecast(location: Location, weather: Forecast) {
    println!(
        "<========Weather forecast for {}, {}:==========>",
        location.name, location.country
    );

    println!("Latitude: {},  Longitude: {}", location.lat, location.lon);

    println!("Temperature: {} °C", weather.main.temp);
    println!("Feels Like: {} °C", weather.main.feels_like);
    println!("Temperature (Max): {} °C", weather.main.temp_max);
    println!("Temperature (Min): {} °C", weather.main.temp_min);
    println!("Pressure: {} hPa", weather.main.pressure);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Wind Speed: {} m/s", weather.wind.speed);
    println!("Wind Direction: {} degrees", weather.wind.deg);
    if let Some(gust) = weather.wind.gust {
        println!("Wind Gust: {} m/s", gust);
    }

    // Weather conditions
    for condition in &weather.weather {
        println!("Weather Condition: {}", condition.main);
        println!("Description: {}", condition.description);
    }

    // Clouds
    println!("Cloudiness: {}%", weather.clouds.all);

    // Rain
    if let Some(rain) = &weather.rain {
        if let Some(one_hour) = rain.one_hour {
            println!("Rain volume for the last 1 hour: {} mm", one_hour);
        }
        if let Some(three_hours) = rain.three_hours {
            println!("Rain volume for the last 3 hours: {} mm", three_hours);
        }
    } else {
        println!("No rain data available");
    }

    // Snow
    if let Some(snow) = &weather.snow {
        if let Some(one_hour) = snow.one_hour {
            println!("Snow volume for the last 1 hour: {} mm", one_hour);
        }
        if let Some(three_hours) = snow.three_hours {
            println!("Snow volume for the last 3 hours: {} mm", three_hours);
        }
    } else {
        println!("No snow data available");
    }

    // Additional info
    //println!("Time of data calculation: {}", weather.dt);
    //println!("Country code: {}", weather.sys.country);
    //println!("Sunrise time: {}", weather.sys.sunrise);
    //println!("Sunset time: {}", weather.sys.sunset);
    //println!("Timezone: {}", weather.timezone);
    //println!("City name: {}", weather.name);
}
