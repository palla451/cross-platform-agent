use crate::collector::payload::Payload;
use redis::{Commands, Client};
use std::env;

/// Invia il payload serializzato a Redis sulla coda specificata in REDIS_QUEUE.
/// Usa la variabile d'ambiente REDIS_URL come connessione al server Redis.
pub fn send_to_redis(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // Recupera la URL di Redis e il nome della coda dalle variabili d'ambiente
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let queue_name = env::var("REDIS_QUEUE").unwrap_or_else(|_| "agent_payloads".to_string());

    // Crea il client Redis e apre una connessione
    let client = Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    // Serializza il payload in JSON
    let json = serde_json::to_string(payload)?;

    // Invia il JSON sulla coda (lista Redis)
    let _: () = con.lpush(queue_name.clone(), json)?;

    // Log di successo
    println!("✅ Payload pubblicato su Redis nella coda '{}'.", queue_name);
    Ok(())
}
