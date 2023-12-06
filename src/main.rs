mod api;
mod error;
mod models;

use api::air_quality;
use api::geocoding;
use api::weather;
use clap::{App, Arg};
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() {
    let matches = App::new("WeatherCLI")
        .version("1.0")
        .about("A Rust CLI application that provides weather and AQI data")
        .arg(Arg::with_name("city").help("Sets the city to use").index(1)) // this makes it a positional argument
        .arg(
            Arg::with_name("country")
                .help("Sets the country to use")
                .index(2),
        ) // this makes it a positional argument
        .get_matches();

    let mut city = matches.value_of("city").unwrap_or("").to_string();
    let mut country = matches.value_of("country").unwrap_or("").to_string();
    if city.is_empty() || country.is_empty() {
        println!("Welcome to WeatherCLI!");
        println!("Please enter the city name and country code when prompted.");

        if city.is_empty() {
            print!("City: ");
            io::stdout().flush().unwrap(); // flush it to the screen
            city = String::new();
            io::stdin().read_line(&mut city).unwrap();
            city = city.trim().to_string(); // remove the newline character
        }

        if country.is_empty() {
            print!("Country: ");
            io::stdout().flush().unwrap(); // flush it to the screen
            country = String::new();
            io::stdin().read_line(&mut country).unwrap();
            country = country.trim().to_string(); // remove the newline character
        }
    }

    // Call the API to get the coordinates of the location
    let location_result = geocoding::get_coordinates(&city, &country).await;
    match location_result {
        Ok(location) => {
            // Call the API to get the weather using the obtained location
            weather::get_weather(location.clone()).await;
            air_quality::get_air_quality(location).await;
        }
        Err(err) => {
            eprintln!("Failed to fetch location data: {}", err);
            process::exit(1);
        }
    }
}
