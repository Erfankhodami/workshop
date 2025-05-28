use reqwest::blocking;
use serde::Deserialize;
use text_io::read;

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    localtime: String,
}

#[derive(Debug, Deserialize)]
struct Condition {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Current {
    temp_c: f64,
    condition: Condition,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

fn main() {
    println!("Enter city name:");
    let city: String = read!("{}\n");
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key=3483c1b9f9d3491989f72353252705&q={}&aqi=no",city
    );
    let response = blocking::get(&url).unwrap();
    let tex = response.text().unwrap();
    let weather_data: WeatherResponse = serde_json::from_str(&tex).unwrap();
    /*println!("Location: {}", weather_data.location.name);*/
    println!("🗺 Location: {}", weather_data.location.name);
    println!("🌍 Country: {}", weather_data.location.country);
    println!("📌 Latitude: {}", weather_data.location.lat);
    println!("📌 Longitude: {}", weather_data.location.lon);
    println!("⌚ Local Time: {}", weather_data.location.localtime);
    println!("🌡 Temperature: {}°C", weather_data.current.temp_c);
    println!("☀️ Condition: {}", weather_data.current.condition.text);
}

