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

pub async fn add_title(id: u32, name: &str, pool: &Pool<Sqlite>) -> Result<i32, sqlx::Error> {
    match sqlx::query("SELECT id FROM Titles WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
    {
        Ok(_) => {
            return Ok(-1);
        }
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("INSERT INTO Titles (id, title_name) VALUES (?, ?)")
                .bind(id)
                .bind(name)
                .execute(pool)
                .await?;

            return Ok(id as i32);
        }
        Err(e) => Err(e),
    }
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

pub async fn setup_and_download(title_name: &str, pool: &SqlitePool) {
    let title_id = get_title_id(&title_name, &pool).await;

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

    let request_url = extract_url(&base_text);
    println!("Requesting URL `{}`...", request_url);
    if let Err(e) = download_with_progress(&request_url, &title_name).await {
        eprintln!("Error during download: {}", e);
    }
}

pub fn extract_url(base_text: &Vec<String>) -> &str {
    for i in base_text {
        if i.contains("Direct Download") {
            if let Some(start) = i.find("href=\"") {
                let url_start = start + 6;
                if let Some(end) = i[url_start..].find('"') {
                    let url = &i[url_start..url_start + end];
                    return url;
                }
            }
            break;
        }
    }
    return "";
}

pub fn display_help_message() {
    println!("{}Usage:{}", WHITE, RESET);
    println!("{}./hshopper help {} // Displays this message", BLUE, GREEN);
    println!("{}./hshopper download {}<title-name> {}// Starts downloading the requested title to a cia file{}", BLUE, WHITE, GREEN, RESET);
    println!("{}./hshopper batch-download {}\"<title-name>\" \"<title-name>\" \"<title-name>\" ... {}// Downloads multiple requested titles to respective cia files{}",
        BLUE, WHITE, GREEN, RESET
    );
    println!("{}./hshopper search {}<title-name> {}// Searches the title database and gives you the results that are most like your search{}",
        BLUE, WHITE, GREEN, RESET
    );
    println!("{}./hshopper add {}<id> <title-name> {}// Adds a title with it's matching id to the database{}",
        BLUE, WHITE, GREEN, RESET
    );
    println!(
        "{}./hshopper list-supported {}// Lists all supported titles{}",
        BLUE, GREEN, RESET
    );
}
