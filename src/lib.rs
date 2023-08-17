mod app_state;
mod router;

use std::net::SocketAddr;

use eyre::Result;
use tera::Tera;

use crate::{app_state::AppState, router::create_router};

pub struct App {
    address: [u8; 4],
    port: u16,
    tera: Tera,
}

impl App {
    pub fn new() -> Result<Self> {
        let address = [127, 0, 0, 1];
        let port = 3000;
        let tera = Tera::new("templates/**/*.html")?;

        Ok(Self {
            address,
            port,
            tera,
        })
    }

    pub async fn serve(&self) -> Result<()> {
        tracing_subscriber::fmt::init();
        let state = AppState {
            tera: self.tera.clone(),
        };
        let router = create_router(state);
        let address = SocketAddr::from((self.address, self.port));
        tracing::info!("Server running on port {}", self.port);
        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;
        Ok(())
    }
}
