use tokio;
use clap::Parser;

mod cli_parse;
mod utils;

#[tokio::main]
async fn main() {
    let cli = cli_parse::Cli::parse();

    let pool = utils::connect()
        .await
        .expect("Failed to load database pool");

    match &cli.command {
        Some(cli_parse::Commands::ListSupported) => {
            let supported = utils::get_supported_titles(&pool).await;
            println!("{}Supported titles:{}", utils::BLUE, utils::RESET);
            println!("{}{}{}", utils::WHITE, supported.join(", "), utils::RESET);
            println!(
                "\n{}{} total supported titles.{}",
                utils::GREEN,
                supported.len(),
                utils::RESET
            );
        }
        Some(cli_parse::Commands::Search { title }) => {
            let search_title = title.join("-").to_ascii_lowercase();
            println!(
                "{}Searching for title {}`{}`{}...{}",
                utils::BLUE,
                utils::WHITE,
                search_title,
                utils::BLUE,
                utils::RESET
            );

            let results = utils::search_titles_by_name(&pool, &search_title).await;

            println!("{}Top 10 Related Results:{}\n", utils::BLUE, utils::RESET);
            println!("{}{}{}", utils::WHITE, results.join(", "), utils::RESET);
        }
        Some(cli_parse::Commands::Add { id, name }) => {
            let title = name.join("-");
            println!(
                "{}Attempting to add title {}`{}`{} with {}id `{}`{}",
                utils::BLUE,
                utils::WHITE,
                title,
                utils::BLUE,
                utils::WHITE,
                id,
                utils::RESET
            );

            let add_result = match utils::add_title(*id, &title, &pool).await {
                Ok(value) => value,
                Err(_) => -1,
            };

            if add_result >= 1 {
                println!(
                    "{}Added title {}`{}`{} to the database with {}id `{}`{}",
                    utils::BLUE,
                    utils::WHITE,
                    title,
                    utils::BLUE,
                    utils::WHITE,
                    id,
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
                    id,
                    utils::RESET
                );
            }
        }
        Some(cli_parse::Commands::Download { name }) => {
            let title_name = name.join("-").to_ascii_lowercase();
            let title_id = utils::get_title_id(&title_name, &pool).await;

            if title_id < 0 {
                println!("Title `{}` was not found in the database", &title_name);
                return;
            }

            let base_text = reqwest::get(&format!("https://hshop.erista.me/t/{}", &title_id))
                .await
                .expect("Failed to make initial request")
                .text()
                .await
                .expect("Failed to parse request text")
                .lines()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();

            let request_url = utils::extract_url(&base_text);
            println!("Requesting URL `{}`...", request_url);
            if let Err(e) = utils::download_with_progress(&request_url, &title_name).await {
                eprintln!("Error during download: {}", e);
            }
        }
        None => {
            utils::display_help_message();
        }
    }
}
