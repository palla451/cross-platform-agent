use crate::collector::payload::Payload;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;
use std::path::Path;

pub fn send_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    println!("📤 Inizio invio del payload...");

    // Stampa le variabili di ambiente se presenti
    match env::var("URL_BACKEND") {
        Ok(val) => println!("🔗 URL_BACKEND = {}", val),
        Err(_) => println!("⚠️  Variabile URL_BACKEND non trovata"),
    }

    match env::var("BEARER_TOKEN") {
        Ok(_) => println!("🔐 Token trovato"),
        Err(_) => println!("⚠️  Variabile BEARER_TOKEN non trovata"),
    }

    // Verifica esistenza del file .env
    if !Path::new(".env").exists() {
        eprintln!("⚠️  Nessun file .env trovato. Assicurati che le variabili siano settate.");
    }

    let url = env::var("URL_BACKEND")?;
    let token = env::var("BEARER_TOKEN")?;

    let client = Client::new();

    println!("🌍 Tentativo di POST a: {}", url);

    // Invia la richiesta, gestendo errori di rete
    let res = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(payload)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("✅ Payload inviato correttamente.");
            } else {
                eprintln!(
                    "❌ Errore HTTP: status={} body={}",
                    response.status(),
                    response.text().unwrap_or_else(|_| "N/A".into())
                );
            }
        }
        Err(err) => {
            eprintln!("❌ Errore di rete durante l'invio: {}", err);
            return Err(Box::new(err));
        }
    }

    Ok(())
}
