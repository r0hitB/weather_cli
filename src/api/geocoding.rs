use crate::error::CustomError;
use crate::models::Location;
use reqwest;

pub async fn get_coordinates(city: &str, state: &str) -> Result<Location, CustomError> {
    let url = format!(
    "http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=1&appid=023356249c1f03f5843292dbc860105a&units=metric",
    city, state
);
    let response = reqwest::get(&url).await?;
    let location: Vec<Location> = response.json().await?;

    if let Some(first_location) = location.get(0) {
        Ok(first_location.clone())
    } else {
        Err(CustomError::new("No location data found"))
    }
}
