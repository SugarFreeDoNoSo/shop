use redis::Commands;
use redis::RedisResult;

/// Estructura para manejar la conexión y operaciones con Redis.
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    /// Crea una nueva instancia de RedisCache.
    pub fn new(url: &str) -> RedisCache {
        let client = redis::Client::open(url).expect("URL de Redis inválida");
        RedisCache { client }
    }

    /// Obtiene un valor de Redis dado una clave.
    pub fn get(&self, key: &str) -> Option<String> {
        let mut con = self.client.get_connection().expect("Conexión a Redis falló");
        let result: RedisResult<String> = con.get(key);
        result.ok()
    }

    /// Establece un valor en Redis para una clave dada.
    pub fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut con = self.client.get_connection()?;
        con.set(key, value)
    }
}
