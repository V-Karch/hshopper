use reqwest::Client;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::fs::File as AsyncFile;
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite, Row};            

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {                                                                                                                                              
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:titledb.db").await?;                                                                                                                              
    return Ok(pool);                                                                                                                                                                                                     
}

pub async fn get_title_id(name: &str, pool: &Pool<Sqlite>) -> i32 {
    return match sqlx::query("SELECT * FROM Titles WHERE title_name = ?")
    .bind(name)
    .fetch_one(pool)
    .await {
        Ok(row) => row.get("id"),
        Err(_) => -1,
    };
}

pub async fn download_with_progress(url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    let total_size = response.content_length().ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
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
    println!("Usage:");
    println!("./hshopper <title-name>");
    println!("For example: `./hshopper \"pokemon ultra moon\"` would result in downloading `pokemon-ultra-moon.cia`");
    println!("Note that this is encrypted and must be decrypted with cia-unix or something similar before use");
    println!("\nIf you are having trouble running this program, make sure that you have installed `geckodriver` and that it is running");
}