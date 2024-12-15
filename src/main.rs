mod utils;

use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        utils::display_help_message();
        return Ok(());
    }

    let parsed_argument = args[0].to_ascii_lowercase().replace(" ", "-");

    let pool = utils::connect()
        .await
        .expect("Failed to load database pool");

    if parsed_argument == "list-supported" {
        let supported = utils::get_supported_titles(&pool).await;
        println!("{}Supported titles:{}", utils::BLUE, utils::RESET);
        println!("{}{}{}", utils::WHITE, supported.join(", "), utils::RESET);
        println!("\n{}{} total supported titles.{}", utils::GREEN, supported.len(), utils::RESET);
        return Ok(());
    }

    if parsed_argument == "search" && args.len() >= 2 {
        let search_title = args[1..].join("-").to_ascii_lowercase();

        println!(
            "{}Searching for title {}`{}`{}...{}",
            utils::BLUE,
            utils::WHITE,
            search_title,
            utils::BLUE,
            utils::RESET
        );

        let results =
            utils::search_titles_by_name(&pool, &search_title)
                .await;

        println!("{}Top 10 Related Results:{}\n", utils::BLUE, utils::RESET);
        println!("{}{}{}", utils::WHITE, results.join(", "), utils::RESET);

        return Ok(());
    }

    if parsed_argument == "add" && args.len() >= 2 {
        let title_id = match args[1].parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("{}When adding to the title database, the second argument must be an {}integer id{} for the title, not {}`{}`{}", 
                utils::BLUE, utils::WHITE, utils::BLUE, utils::WHITE, &args[1], utils::RESET
            );
                return Ok(());
            }
        };

        let title = &args[2..].join("-");
        println!(
            "{}Attempting to add title {}`{}`{} with {}id `{}`{}",
            utils::BLUE,
            utils::WHITE,
            title,
            utils::BLUE,
            utils::WHITE,
            title_id,
            utils::RESET
        );

        let add_result = match utils::add_title(title_id, &title, &pool).await {
            Ok(value) => value,
            Err(_) => -1,
        };

        if add_result >= 1 {
            println!(
                "{}Added title {}`{}`{} to the databae with {}id `{}`{}",
                utils::BLUE,
                utils::WHITE,
                title,
                utils::BLUE,
                utils::WHITE,
                title_id,
                utils::RESET
            );
        } else {
            println!(
                "{}Could not add title {}`{}`{} to database with {}id `{}`{}",
                utils::GREEN,
                utils::WHITE,
                title,
                utils::GREEN,
                utils::WHITE,
                title_id,
                utils::RESET
            )
        }

        return Ok(());
    }

    let title_id = utils::get_title_id(&parsed_argument, &pool).await;

    if title_id < 0 {
        println!("Title `{}` was not found in the database", &parsed_argument);
        return Ok(());
    }

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless().expect("Failed to set headless mode");

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver
        .get(&format!("https://hshop.erista.me/t/{}", title_id))
        .await?;
    driver
        .set_implicit_wait_timeout(Duration::from_secs(5))
        .await?;

    let download_button = driver
        .find(By::XPath(
            "/html/body/main/div[2]/div/div[2]/div/div[2]/div[1]/a",
        ))
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
