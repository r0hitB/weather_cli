# WeatherCLI
WeatherCLI is a command-line tool written in Rust that provides weather details using the OpenWeather API.

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

### Building from Source (for Developers)
git clone https://github.com/r0hitB/weather_cli.git

cd weather_cli

cargo build --release


## Usage
Once installed, you can run WeatherCLI from the terminal. It will prompt you to enter the city name and country code. Alternatively, you can provide the city name and country code directly as arguments.
weatherCLI            # Interactive mode
weatherCLI {city} {country}  # Direct mode

This will display weather details along with AQI (Air Quality Index) data.

## License
This project is licensed under the MIT License

