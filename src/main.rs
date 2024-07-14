use tokio::net::TcpListener;
use tracing::{info, warn};

use simple_redis::{process_redis_conn, Database};

mod resp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);
    let db = Database::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        let cloned_db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, cloned_db).await {
                warn!("Error processing connection: {:?}", e)
            }
        });
    }
}
