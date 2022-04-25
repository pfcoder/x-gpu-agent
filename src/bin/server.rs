use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use agent_channel::agent_channel_server::{AgentChannel, AgentChannelServer};
use agent_channel::Message;

pub mod agent_channel {
    tonic::include_proto!("agent_channel");
}

pub struct AgentChannelService;

#[tonic::async_trait]
impl AgentChannel for AgentChannelService {
    type CommunicateStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send + 'static>>;

    async fn communicate(
        &self,
        request: Request<tonic::Streaming<Message>>,
    ) -> Result<Response<Self::CommunicateStream>, Status> {
        let mut stream = request.into_inner();
        let output = async_stream::try_stream! {
            while let Some(message) = stream.next().await {
                let message = message?;
                println!("Received message: {:?}", message);
                yield message.clone();
            }
        };

        Ok(Response::new(Box::pin(output) as Self::CommunicateStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("AgentChannelServer listening on: {}", addr);

    let agent_channel = AgentChannelService {};

    let svc = AgentChannelServer::new(agent_channel);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
