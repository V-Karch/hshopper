use sqlx::{sqlite::SqlitePool, Pool};            

pub async fn connect() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {                                                                                                                                              
    let pool: Pool<sqlx::Sqlite> = SqlitePool::connect("sqlite:titledb.db").await?;                                                                                                                              
    return Ok(pool);                                                                                                                                                                                                     
}