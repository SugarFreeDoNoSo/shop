use libsql::{de, Builder, Connection, Value};
use std::time::Duration;

pub async fn get_connection() -> Result<Connection, Box<dyn std::error::Error>> {
    // URL de la base de datos Turso
    let url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL debe estar configurada");
    // Token de autenticación
    let token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN debe estar configurado");

    let db = Builder::new_remote_replica("local.db", url, token)
        .sync_interval(Duration::from_secs(60))
        .build()
        .await?;

    let conn = db.connect()?;
    Ok(conn)
}

pub fn query<T>(
    conn: &Connection,
    query: &str,
    params: &[Value],
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut stmt = conn.prepare(query);
    let rows = stmt.query(params); 
    let mut results = Vec::new();

    while let Some(row) = rows.next(){
        let item: T = de::from_row(&row)?;
        results.push(item);
    }

    Ok(results)
}
