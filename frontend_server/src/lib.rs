use std::path::Path;

use async_trait::async_trait;
use axum::{Router, Server};
use common::messaging::crossbeam::CrossbeamMessagingFactory;
use common::AsterService;
use tower_http::services::{ServeDir, ServeFile};

pub struct FrontendServer {
    frontend_files: String,
}

#[async_trait]
impl AsterService for FrontendServer {
    const SERVICE_PORT: u16 = 3030;
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        self.frontend_files =
            std::env::var("ASTER_UI_PATH").unwrap_or("./frontend/build".to_string());
        Ok(())
    }
    async fn run(&mut self) -> Result<(), anyhow::Error> {
        // send index.html for any unknown path (SPA)
        let index = Path::new(&self.frontend_files).join("index.html");

        // serve files from the frontend directory
        let serve_dir =
            ServeDir::new(self.frontend_files.clone()).not_found_service(ServeFile::new(index));
        let router = Router::new().nest_service("/", serve_dir);

        // run the server
        Ok(
            Server::bind(&format!("0.0.0.0:{}", Self::SERVICE_PORT).parse().unwrap())
                .serve(router.into_make_service())
                .await?,
        )
    }
}
