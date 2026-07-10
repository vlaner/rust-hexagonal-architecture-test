use anyhow::Result;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let config = rust_backend::config::load()?;
    let address = config.bind_address();
    let listener = TcpListener::bind(&address)?;
    println!("Server running on http://{}", address);

    rust_backend::run(listener, config).await?.await?;

    Ok(())
}
