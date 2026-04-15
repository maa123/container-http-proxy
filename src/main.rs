use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().or_else(|err| {
        if err.not_found() {
            Ok(PathBuf::new())
        } else {
            Err(err)
        }
    })?;

    let upstream_registry = env::var("UPSTREAM_REGISTRY")
        .expect("UPSTREAM_REGISTRY must be set in .env or environment");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("UPSTREAM_REGISTRY={upstream_registry}");
    println!("HOST={host}");
    println!("PORT={port}");

    Ok(())
}
