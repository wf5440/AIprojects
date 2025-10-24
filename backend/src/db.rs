use sqlx::{PgPool, Error};

pub async fn connect_db(database_url: &str) -> Result<PgPool, Error> {
    let pool = PgPool::connect(database_url).await?;
    
    // 测试连接
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
        
    Ok(pool)
}