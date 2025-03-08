use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::fs;
const CONFIG: &str = include_str!("../config.toml");

#[derive(Parser)]
#[command(name = "rust-wp-cli-poster")]
#[command(about = "CLI tool to send a new post to a WordPress instance", long_about = None)]
struct Args {
    /// The post content split into words (will be joined into one message)
    content: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    wordpress_url: String,
    username: String,
    application_password: String,
    category_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    let content = args.content.join(" ");

    // Parse the configuration file from compile-time configuration
    let config: Config = toml::from_str(CONFIG)?;

    // Parse category_id
    let category_id= config.category_id;

    // Create an HTTP client
    let client = Client::new();

    // Construct the JSON payload for the WordPress API
    let payload = json!({
        "title": "",
        "content": content,
        "status": "publish",
        "categories": [category_id]
    });

    // Send POST request with Basic Authentication using the Application Password
    let response = client
        .post(&config.wordpress_url)
        .basic_auth(&config.username, Some(&config.application_password))
        .json(&payload)
        .send()
        .await?;

    // Handle the response
    if response.status().is_success() {
        println!("Post created successfully.");
    } else {
        println!("Failed to create post. Status: {}", response.status());
        let error_text = response.text().await?;
        println!("Response: {}", error_text);
    }

    Ok(())
}
