# WeatherCLI
WeatherCLI is a command-line tool that provides weather details using the OpenWeather API.

## Installation
### Linux Users
 Clone the repository
 git clone https://github.com/r0hitB/weather_cli.git
 cd weather_cli
 Build the project
 cargo build --release
 Run the tool
./target/release/weatherCLI


### Windows Users
Visit the releases page on GitHub.
Download the appropriate binary for your system.
Extract the downloaded ZIP file.
Open a command prompt and navigate to the extracted directory.
Run the weatherCLI.exe executable.


## Usage
Once the tool is installed, you can use it to get weather details. Open a terminal or command prompt and run:

Interactively enter city name and country code
weatherCLI

Provide city name and country code as arguments
weatherCLI {city_name} {country_code}

This will display weather details along with AQI (Air Quality Index) data.

## License
This project is licensed under the MIT License

