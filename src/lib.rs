mod router;

use crate::router::create_router;
use eyre::Result;
use std::net::SocketAddr;

pub struct App {
    address: [u8; 4],
    port: u16,
}

impl App {
    pub fn new() -> Result<Self> {
        let address = [127, 0, 0, 1];
        let port = 3000;

        Ok(Self { address, port })
    }

    pub async fn serve(&self) -> Result<()> {
        tracing_subscriber::fmt::init();
        let router = create_router();
        let address = SocketAddr::from((self.address, self.port));
        tracing::info!("Server running on port {}", self.port);
        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;
        Ok(())
    }
}
