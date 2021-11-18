use proto::health::health_server::HealthServer;
use thiserror::Error;
use tonic::transport::Server;

mod health;
mod proto {
    pub mod health {
        tonic::include_proto!("grpc.health.v1");
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("Could not parse provided service address")]
    Address,
    #[error("Error in gRPC transport: {0}")]
    Transport(#[from] tonic::transport::Error),
}

async fn run_service() -> Result<(), Error> {
    let address = "0.0.0.0:50051".parse().map_err(|_| Error::Address)?;

    println!("gRPC service starting");

    Server::builder()
        .add_service(HealthServer::new(health::Health))
        .serve(address)
        .await?;

    println!("gRPC service stopped");

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = run_service().await {
        eprintln!("Stopping gRPC service with error: {}", error);
    }
}
