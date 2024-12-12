mod utils;

use tokio;
use std::time::Duration;
use thirtyfour::prelude::*;

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
    
    let title_id = utils::get_title_id(&parsed_argument, &pool).await;

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

            if let Err(e) = utils::download_with_progress(&url, &parsed_argument).await {
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
