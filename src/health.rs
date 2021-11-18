use crate::proto::health::{
    health_check_response::ServingStatus, health_server::Health as GrpcService, HealthCheckRequest,
    HealthCheckResponse,
};
use std::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

/// Dummy Health service that can be configured to fail
#[derive(Debug)]
pub struct Health {
    should_fail: bool,
}

impl Health {
    pub fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[tonic::async_trait]
impl GrpcService for Health {
    #[tracing::instrument]
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        tracing::info!("Initiating single health check");

        if self.should_fail {
            tracing::warn!("Cancelling health check");
            Err(Status::internal("Failure within the Check method"))
        } else {
            Ok(Response::new(HealthCheckResponse {
                status: ServingStatus::Serving.into(),
            }))
        }
    }

    type WatchStream = UnboundedReceiverStream<Result<HealthCheckResponse, Status>>;

    #[tracing::instrument]
    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        tracing::info!("Initiating stream of health checks");

        if self.should_fail {
            tracing::warn!("Cancelling health check stream");
            Err(Status::internal("Failure within the Watch method"))
        } else {
            let (transmitter, receiver) = tokio::sync::mpsc::unbounded_channel();

            #[allow(unreachable_code)]
            tokio::spawn(async move {
                loop {
                    transmitter.send(Ok(HealthCheckResponse {
                        status: ServingStatus::Serving.into(),
                    }))?;

                    tokio::time::sleep(Duration::from_secs(1)).await;
                }

                Ok::<_, tokio::sync::mpsc::error::SendError<_>>(())
            });

            Ok(Response::new(UnboundedReceiverStream::new(receiver)))
        }
    }
}
