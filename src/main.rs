use failure_modes::FailureMode;
use health::Health;
use proto::health::health_server::HealthServer;
use std::str::FromStr;
use thiserror::Error;
use tonic::transport::Server;

mod failure_modes;
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
    #[error(transparent)]
    Mode(#[from] failure_modes::Error),
    #[error("Error in gRPC transport: {0}")]
    Transport(#[from] tonic::transport::Error),
}

/// Convenience wrapper around the tonic service
async fn run_service() -> Result<(), Error> {
    let address = "0.0.0.0:50051".parse().map_err(|_| Error::Address)?;

    let failure_mode = std::env::var("FAILURE_MODE")
        .ok()
        .map(|mode| FailureMode::from_str(&mode))
        .transpose()?
        .unwrap_or_default();

    println!(
        "gRPC service starting with failure mode {:?}",
        &failure_mode
    );

    Server::builder()
        .add_service(HealthServer::with_interceptor(
            Health::new(matches!(failure_mode, FailureMode::Method)),
            failure_mode.to_interceptor(),
        ))
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
