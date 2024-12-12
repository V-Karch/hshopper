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