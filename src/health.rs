use crate::proto::health::{
    health_check_response::ServingStatus, health_server::Health as GrpcService, HealthCheckRequest,
    HealthCheckResponse,
};
use std::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

pub struct Health;

#[tonic::async_trait]
impl GrpcService for Health {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving.into(),
        }))
    }

    type WatchStream = UnboundedReceiverStream<Result<HealthCheckResponse, Status>>;

    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
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
