use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use sqlx::{sqlite::SqlitePool, Pool, Row, Sqlite};
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;

pub static BLUE: &str = "\x1b[38;2;98;152;214m";
pub static GREEN: &str = "\x1b[38;2;10;178;46m";
pub static WHITE: &str = "\x1b[38;2;255;255;255m";
pub static RESET: &str = "\x1b[0m";

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:titledb.db").await?;
    return Ok(pool);
}

pub async fn get_title_id(name: &str, pool: &Pool<Sqlite>) -> i32 {
    return match sqlx::query("SELECT * FROM Titles WHERE title_name = ?")
        .bind(name)
        .fetch_one(pool)
        .await
    {
        Ok(row) => row.get("id"),
        Err(_) => -1,
    };
}

pub async fn search_titles_by_name(pool: &SqlitePool, query: &str) -> Vec<String> {
    let query = format!("%{}%", query).replace(" ", "-");

    match sqlx::query("SELECT title_name FROM Titles WHERE title_name LIKE ? LIMIT 10")
        .bind(query)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                let title: String = row.get("title_name");
                format!("\"{}\"", title)
            })
            .collect(),
        Err(_) => vec![],
    }
}

pub async fn get_supported_titles(pool: &SqlitePool) -> Vec<String> {
    match sqlx::query("SELECT title_name FROM Titles")
        .fetch_all(pool)
        .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                let title: String = row.get("title_name");
                format!("\"{}\"", title)
            })
            .collect(),
        Err(_) => vec![],
    }
}

pub async fn download_with_progress(
    url: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.GREEN} [{elapsed_precise}] [{wide_bar:.cyan/BLUE}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut file = AsyncFile::create(format!("{}.cia", name)).await?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}

pub fn display_help_message() {
    println!("{}Usage:{}", WHITE, RESET);
    println!("{}./hshopper {}\"<title-name>\" {}// Starts downloading the requested title to a cia file{}", BLUE, WHITE, GREEN, RESET);
    println!("{}./hshopper search {}\"<title-name>\" {}// Searches the title database and gives you the results that are most like your search{}",
        BLUE, WHITE, GREEN, RESET
    );
    println!(
        "{}./hshopper {}list-supported {}// Lists all supported titles{}",
        BLUE, WHITE, GREEN, RESET
    );
    println!("{}\nIf you are having trouble running this program, make sure that you have installed {}`geckodriver`{} and that it is running{}",
        WHITE, BLUE, WHITE, RESET
    );
}
