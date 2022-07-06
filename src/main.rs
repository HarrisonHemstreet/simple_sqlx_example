use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = PgPool::connect("postgres://postgres:password@localhost/postgres").await?;

    let rows = sqlx::query!("select * from todos;")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        println!(
            "- [{}] {}: {}",
            if row.done { "x" } else { " " },
            row.id,
            &row.description,
        );
    }

    Ok(())
}
