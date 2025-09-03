use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use askama::Template;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;

// Template for the main page
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    city: String,
    temperature: String,
    feels_like: String,
    humidity: String,
    precipitation: String,
    wind_speed: String,
    condition: String,
    background_image: String,
    error: String,
}

// Form input structure
#[derive(Deserialize)]
struct CityForm {
    city: String,
}

// Weather data structure (for potential expansion)
#[derive(Serialize, Deserialize)]
struct WeatherData {
    temperature: f64,
    condition: String,
}

async fn index() -> impl IntoResponse {
    Html(
        IndexTemplate {
            city: String::new(),
            temperature: String::new(),
            feels_like: String::new(),
            humidity: String::new(),
            precipitation: String::new(),
            wind_speed: String::new(),
            condition: String::new(),
            background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(), // Default: sunny
            error: String::new(),
        }
        .render()
        .unwrap(),
    )
}

async fn get_weather(Form(form): Form<CityForm>) -> impl IntoResponse {
    let client = Client::new();

    // Step 1: Geocode city to get latitude and longitude
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        form.city
    );
    let geocoding_response = match client.get(&geocoding_url).send().await {
        Ok(resp) => resp,
        Err(e) => return Html(
            IndexTemplate {
                city: form.city.clone(),
                temperature: String::new(),
                feels_like: String::new(),
                humidity: String::new(),
                precipitation: String::new(),
                wind_speed: String::new(),
                condition: String::new(),
                background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(),
                error: format!("Failed to fetch geocoding data: {}", e),
            }
            .render()
            .unwrap(),
        ),
    };
    let geocoding_data: Value = match geocoding_response.json().await {
        Ok(data) => data,
        Err(e) => return Html(
            IndexTemplate {
                city: form.city.clone(),
                temperature: String::new(),
                feels_like: String::new(),
                humidity: String::new(),
                precipitation: String::new(),
                wind_speed: String::new(),
                condition: String::new(),
                background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(),
                error: format!("Failed to parse geocoding data: {}", e),
            }
            .render()
            .unwrap(),
        ),
    };

    let (lat, lon) = match geocoding_data["results"].as_array() {
        Some(results) if !results.is_empty() => {
            let result = &results[0];
            (
                result["latitude"].as_f64().unwrap_or(0.0),
                result["longitude"].as_f64().unwrap_or(0.0),
            )
        }
        _ => return Html(
            IndexTemplate {
                city: form.city.clone(),
                temperature: String::new(),
                feels_like: String::new(),
                humidity: String::new(),
                precipitation: String::new(),
                wind_speed: String::new(),
                condition: String::new(),
                background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(),
                error: format!("City '{}' not found", form.city),
            }
            .render()
            .unwrap(),
        ),
    };

    // Step 2: Fetch weather for coordinates
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,wind_speed_10m,weather_code",
        lat, lon
    );
    let weather_response = match client.get(&weather_url).send().await {
        Ok(resp) => resp,
        Err(e) => return Html(
            IndexTemplate {
                city: form.city.clone(),
                temperature: String::new(),
                feels_like: String::new(),
                humidity: String::new(),
                precipitation: String::new(),
                wind_speed: String::new(),
                condition: String::new(),
                background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(),
                error: format!("Failed to fetch weather data: {}", e),
            }
            .render()
            .unwrap(),
        ),
    };
    let weather_data: Value = match weather_response.json().await {
        Ok(data) => data,
        Err(e) => return Html(
            IndexTemplate {
                city: form.city.clone(),
                temperature: String::new(),
                feels_like: String::new(),
                humidity: String::new(),
                precipitation: String::new(),
                wind_speed: String::new(),
                condition: String::new(),
                background_image: "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D".to_string(),
                error: format!("Failed to parse weather data: {}", e),
            }
            .render()
            .unwrap(),
        ),
    };

    let temperature = weather_data["current"]["temperature_2m"]
        .as_f64()
        .unwrap_or(0.0);
    let apparent_temperature = weather_data["current"]["apparent_temperature"]
        .as_f64()
        .unwrap_or(0.0);
    let humidity = weather_data["current"]["relative_humidity_2m"]
        .as_f64()
        .unwrap_or(0.0);
    let precipitation = weather_data["current"]["precipitation"]
        .as_f64()
        .unwrap_or(0.0);
    let wind_speed = weather_data["current"]["wind_speed_10m"]
        .as_f64()
        .unwrap_or(0.0);
    let weather_code = weather_data["current"]["weather_code"]
        .as_i64()
        .unwrap_or(0);

    // Map weather code to condition (with emoji) and Unsplash image URL
    let (condition, background_image) = match weather_code {
        0 => ("Clear ☀️", "https://images.unsplash.com/photo-1604228741406-3faa38f4907a?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8c3Vubnl8ZW58MHx8MHx8fDA%3D"),
        1..=3 => ("Cloudy ☁️", "https://images.unsplash.com/photo-1536532184021-da5392b55da1?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NTh8fGNsb3VkeXxlbnwwfHwwfHx8MA%3D%3D"),
        45 | 48 => ("Fog 🌫️", "https://images.unsplash.com/photo-1543968996-ee822b8176e3?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8Zm9nfGVufDB8fDB8fHww"),
        51..=67 | 80..=82 => ("Rain 🌧️", "https://images.unsplash.com/photo-1605035015406-54c130d0bf89?q=80&w=1632&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"),
        71..=77 | 85..=86 => ("Snow ❄️", "https://images.unsplash.com/photo-1418985991508-e47386d96a71?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTB8fHNub3d8ZW58MHx8MHx8fDA%3D"),
        95..=99 => ("Thunderstorm ⚡", "https://images.unsplash.com/photo-1429514513361-8fa32282fd5f?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8dGh1bmRlcnN0b3JtfGVufDB8fDB8fHww"),
        _ => ("Unknown ❓", "https://images.unsplash.com/photo-1611406686532-0df7e54ea334?q=80&w=1074&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"),
    };

    Html(
        IndexTemplate {
            city: form.city,
            temperature: format!("{:.1}°C", temperature),
            feels_like: format!("{:.1}°C", apparent_temperature),
            humidity: format!("{:.0}%", humidity),
            precipitation: format!("{:.1} mm", precipitation),
            wind_speed: format!("{:.1} km/h", wind_speed),
            condition: condition.to_string(),
            background_image: background_image.to_string(),
            error: String::new(),
        }
        .render()
        .unwrap(),
    )
}

#[tokio::main]
async fn main() {
    // Set up router with routes
    let app = Router::new()
        .route("/", get(index))
        .route("/weather", post(get_weather));

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}