use std::error::Error;
use std::time::Duration;

use futures::stream;

use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

use agent_channel::agent_channel_client::AgentChannelClient;
use agent_channel::Message;

pub mod agent_channel {
    tonic::include_proto!("agent_channel");
}

async fn run(client: &mut AgentChannelClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let message = Message {
                message_type: 1,
                message_data: "Hello, world!".to_string(),
            };

            yield message;
        }
    };

    let response = client.communicate(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("Message = {:?}", note);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentChannelClient::connect("http://[::1]:10000").await?;

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    run(&mut client).await?;

    Ok(())
}
