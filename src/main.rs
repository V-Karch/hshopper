mod utils;

use tokio;
use sqlx::{Pool, Row, Sqlite};
use reqwest::Client;
use std::time::Duration;
use thirtyfour::prelude::*;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::fs::File as AsyncFile;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        println!("Game title required.");
        return Ok(());
    }

    let parsed_argument = args[0]
        .to_ascii_lowercase()
        .replace(" ", "-");

    let pool = utils::connect()
        .await
        .expect("Failed to load database pool");
    
    let title_id = get_title_id(&parsed_argument, &pool).await;

    if title_id < 0 {
        println!("Title `{}` was not found in the database", &parsed_argument);
        return Ok(());
    }

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless().expect("Failed to set headless mode");

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.get(&format!("https://hshop.erista.me/t/{}", title_id)).await?;
    driver.set_implicit_wait_timeout(Duration::from_secs(5)).await?;

    let download_button = driver
        .find(By::XPath("/html/body/main/div[2]/div/div[2]/div/div[2]/div[1]/a"))
        .await?;

    let possible_download_url = download_button.attr("href").await?;

    match possible_download_url {
        Some(url) => {
            println!("Requesting URL `{}`...", url);

            if let Err(e) = download_with_progress(&url, &parsed_argument).await {
                eprintln!("Error during download: {}", e);
            }
        }
        None => {
            println!("Failed to get download URL");
        }
    }

    driver.quit().await?;

    Ok(())
}

async fn get_title_id(name: &str, pool: &Pool<Sqlite>) -> i32 {
    return match sqlx::query("SELECT * FROM Titles WHERE title_name = ?")
    .bind(name)
    .fetch_one(pool)
    .await {
        Ok(row) => row.get("id"),
        Err(_) => -1,
    };
}

async fn download_with_progress(url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    let total_size = response.content_length().ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut file = AsyncFile::create(format!("{}.cia", name)).await?; // Use async file creation with tokio::fs::File.
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?; // Use async file writing.
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}
