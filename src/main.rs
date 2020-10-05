use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Person {
    id: u64,
    name: String,
}

async fn clean(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM people").execute(pool).await?;
    Ok(())
}

async fn create(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO people(id, name) VALUES(1, 'John Doe')")
        .execute(pool)
        .await?;
    Ok(())
}

async fn find(pool: &MySqlPool) -> Result<Person, sqlx::Error> {
    let found = sqlx::query_as::<_, Person>("SELECT id, name FROM people WHERE id = ?")
        .bind(1_u64)
        .fetch_one(pool)
        .await?;
    Ok(found)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:secret@localhost:3306/dev")
        .await?;

    clean(&pool).await?;
    create(&pool).await?;
    let person = find(&pool).await?;
    assert_eq!(
        person,
        Person {
            id: 1,
            name: "John Doe".to_string(),
        }
    );

    Ok(())
}
