use anyhow::Result;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("Server running on http://{}", address);

    rust_backend::run(listener).await?.await?;

    Ok(())
}
