mod sql;
use sql;

#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
    age: i64,
    vision: f64,
    avatar: Vec<u8>,
}


// Ejemplo de uso:
pub fn lista_user() -> Vec<User> {
    let conn = get_connection();
    let users: Vec<User> = sql::query(&conn, "SELECT * FROM users");
    users 
}