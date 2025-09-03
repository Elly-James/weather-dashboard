# 🌤️ Weather Dashboard

A simple, real-time weather web application built with **Rust** and **Axum** as part of the Moringa AI Capstone Project.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## 📋 Overview

This project is a beginner-friendly toolkit that leverages generative AI prompts to explore and learn **Rust** (a systems programming language) and **Axum** (a modern, lightweight web framework for Rust). The application serves as an educational resource for developers starting with these technologies.

### 🎯 Objectives

- **Technology Stack**: Rust (programming language) + Axum (web framework)
- **Purpose**: Explore memory-safe, high-performance backend development
- **Goal**: Create an interactive weather dashboard with real-time API integration

### ✨ Features

- 🏙️ City-based weather search
- 🌡️ Real-time weather data via Open-Meteo API
- 🎨 Dynamic background images based on weather conditions
- 📱 Responsive web interface
- ⚡ Async/await support for optimal performance

## 🔧 Technology Stack

### What is Rust?
Rust is a systems programming language focused on **safety** and **performance**, featuring a strong type system and ownership model that prevents common programming errors.

### What is Axum?
Axum is a modern Rust web framework built on **Tokio** for asynchronous HTTP handling, designed for simplicity and scalability.

### Real-world Applications
- **Discord**: Uses Rust for backend infrastructure
- **Dropbox**: File storage systems
- **Firefox**: Browser engine components
- **Game Engines**: Performance-critical applications

## 📋 System Requirements

| Component | Requirement |
|-----------|-------------|
| **Operating System** | Linux, macOS, or Windows |
| **Rust Version** | 1.80+ |
| **Package Manager** | Cargo (included with Rust) |
| **Editor** | VS Code with Rust Analyzer extension (recommended) |

## 🚀 Installation & Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:
```bash
source $HOME/.cargo/env
```

### 2. Clone the Repository

```bash
git clone https://github.com/Elly-James/weather-dashboard.git
cd weather-dashboard
```

### 3. Install Dependencies

```bash
cargo build
```

### 4. Run the Application

```bash
cargo run
```

🌐 Open your browser and navigate to: **http://127.0.0.1:3000**

## 💻 Usage Example

### What the Application Does

1. **Input**: Enter a city name in the search form
2. **Processing**: Fetches real-time weather data from Open-Meteo API
3. **Output**: Displays comprehensive weather information:
   - Current temperature
   - "Feels like" temperature
   - Humidity levels
   - Precipitation data
   - Wind speed
   - Weather conditions
4. **Visual**: Updates background image based on current weather conditions

### Sample Code

```rust
async fn get_weather(Form(form): Form<CityForm>) -> impl IntoResponse {
    let client = Client::new();
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1", 
        form.city
    );
    let geocoding_response = client.get(&geocoding_url).send().await.unwrap();
    let geocoding_data: Value = geocoding_response.json().await.unwrap();
    
    let (lat, lon) = (
        geocoding_data["results"][0]["latitude"].as_f64().unwrap(),
        geocoding_data["results"][0]["longitude"].as_f64().unwrap()
    );
    
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,wind_speed_10m,weather_code", 
        lat, lon
    );
    let weather_response = client.get(&weather_url).send().await.unwrap();
    let weather_data: Value = weather_response.json().await.unwrap();
    
    let temperature = format!("{:.1}°C", 
        weather_data["current"]["temperature_2m"].as_f64().unwrap()
    );
    
    Html(IndexTemplate { 
        city: form.city, 
        temperature, 
        // ... other weather data
    }.render().unwrap())
}
```

### Expected Output

**Input**: "Nairobi"

**Output**:
```
Weather in Nairobi
Temperature: 25.0°C
Feels Like: 24.5°C
Humidity: 65%
Wind Speed: 12 km/h
Condition: Partly Cloudy
```
*Background dynamically changes to reflect current weather conditions*

## 🤖 AI Development Process

### Prompt Engineering

**Initial Prompt**: 
> "Provide a step-by-step guide to build a simple weather web app in Rust with Axum, using reqwest for Open-Meteo API calls, Askama for templates, and dynamic background images based on weather conditions."

**Curriculum Reference**: [ai.moringaschool.com](https://ai.moringaschool.com)

### AI Contribution Analysis

✅ **Highly Useful**:
- Initial Axum setup and configuration
- API integration patterns with reqwest
- Askama templating structure
- Weather code to image URL mapping

🔄 **Required Refinement**:
- Error handling patterns
- Template syntax optimization
- Rust-specific error management strategies

## 🛠️ Common Issues & Solutions

### Issue 1: Askama Template Compilation Error

**Problem**: 
```
error: Option<String> does not implement Display
```

**Solution**:
```rust
// ❌ Before
error: Option<String>

// ✅ After  
error: String

// Use is_empty() checks in templates
```

**Reference**: [Askama Documentation](https://docs.rs/askama/)

### Issue 2: City Not Found Errors

**Problem**: Open-Meteo API returns empty results for misspelled cities

**Solution**:
- Added user-friendly error messages
- Input validation and feedback
- Styled error display components

**Reference**: [API Error Handling Best Practices](https://stackoverflow.com)

## 📚 Resources & References

### 📖 Official Documentation
- [Rust Programming Language](https://doc.rust-lang.org/)
- [Axum Web Framework](https://docs.rs/axum/)
- [Open-Meteo API](https://open-meteo.com/en/docs)

### 🎥 Learning Resources
- [Rust Basics Tutorial](https://youtube.com) *(Educational)*
- [Getting Started with Axum](https://blog.example.com) *(Tutorial)*

### 🔗 Helpful Links
- [Rust Community](https://www.rust-lang.org/community)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Built with ❤️ using Rust & Axum**

*Part of the Moringa AI Capstone Project*

</div>