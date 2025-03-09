use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
const CONFIG: &str = include_str!("../config.toml");

#[derive(Parser)]
#[command(name = "rust-wp-cli-poster")]
#[command(version, about = "CLI tool to send a new post to a WordPress instance", long_about = None)]
struct Args {
    /// The post title
    #[arg(short, long)]
    title: Option<String>,

    /// The post content split into words (will be joined into one message)
    content: Vec<String>,
    
    /// Draft mode - don't publish immediately
    #[arg(short, long)]
    draft: bool,
}

#[derive(Deserialize)]
struct Config {
    wordpress_url: String,
    username: String,
    application_password: String,
    category_id: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PostResponse {
    id: u32,
    link: String,
    #[serde(default)]
    title: PostTitle,
}

#[derive(Deserialize, Debug, Default)]
struct PostTitle {
    #[serde(default)]
    #[allow(dead_code)]
    rendered: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    let content = args.content.join(" ");
    let status = if args.draft { "draft" } else { "publish" };

    // Parse the configuration file from compile-time configuration
    let config: Config = toml::from_str(CONFIG).map_err(|e| {
        eprintln!("Error parsing config.toml: {}", e);
        e
    })?;

    // Create an HTTP client
    let client = Client::new();

    // Construct the JSON payload for the WordPress API
    let payload = json!({
        "title": args.title.unwrap_or_default(),
        "content": content,
        "status": status,
        "categories": [config.category_id]
    });

    println!("Sending post to {}...", config.wordpress_url);
    
    // Send POST request with Basic Authentication using the Application Password
    let response = client
        .post(&config.wordpress_url)
        .basic_auth(&config.username, Some(&config.application_password))
        .json(&payload)
        .send()
        .await?;

    // Handle the response
    let status = response.status();
    match response.json::<PostResponse>().await {
        Ok(post_data) => {
            println!("Post created successfully!");
            println!("ID: {}", post_data.id);
            println!("Link: {}", post_data.link);
        },
        Err(e) => {
            println!("Post appears to be created, but couldn't parse response: {}", e);
            println!("Response status: {}", status);
        }
    }

    Ok(())
}
