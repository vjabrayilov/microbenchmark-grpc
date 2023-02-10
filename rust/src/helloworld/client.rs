

use std::time::SystemTime;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use clap::Parser;
use tokio::task::JoinSet;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 100000)]
    num_requests: u64,

    #[arg(long, default_value_t = 1)]
    num_clients: u64,

    #[arg(long, default_value_t = String::from("http://[::1]:50051"))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let num_requests = args.num_requests;
    let num_clients = args.num_clients;

    let client = GreeterClient::connect(args.addr.to_string()).await?;

    let mut client_tasks = JoinSet::new();
    for _i in 0..num_clients {
        let mut client = client.clone();
        client_tasks.spawn(async move {
            let start_time = SystemTime::now();
            for _j in 0..num_requests {
                let request = tonic::Request::new(HelloRequest {
                    name: String::from("world"),
                });
                let _response = client.say_hello(request).await;
            }
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time);
            let duration = duration.unwrap().as_secs();
            println!("single client throughput: {} op/s", num_requests/duration);
        });
    }

    while let Some(_response_result) = client_tasks.join_next().await {}

    Ok(())
}
