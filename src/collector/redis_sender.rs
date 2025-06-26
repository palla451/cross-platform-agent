use crate::collector::payload::Payload;
use redis::{Commands, Client};
use std::env;

pub fn send_to_redis(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
    let client = Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(payload)?;
    con.lpush("agent_payloads", json)?;

    println!("✅ Payload pubblicato su Redis.");
    Ok(())
}
