mod router;

use std::net::SocketAddr;

use eyre::Result;

use crate::router::create_router;

pub struct App {
    address: [u8; 4],
    port: u16,
}

impl App {
    pub fn new() -> Self {
        let address = [127, 0, 0, 1];
        let port = 3000;
        Self { address, port }
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
