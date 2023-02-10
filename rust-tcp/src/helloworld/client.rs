use clap::Parser;
use std::time::SystemTime;
use tokio::{io::AsyncWriteExt, net::TcpStream, task::JoinSet};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 1000000)]
    num_requests: u128,

    #[arg(long, default_value_t = 1)]
    num_clients: u64,

    #[arg(long, default_value_t = String::from("127.0.0.1:50051"))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let num_requests: u128 = args.num_requests;
    let num_clients = args.num_clients;

    let mut client_tasks = JoinSet::new();
    for _i in 0..num_clients {
        // let mut client = client.clone();
        let mut client = TcpStream::connect(args.addr.to_string()).await?;
        client_tasks.spawn(async move {
            let start_time = SystemTime::now();
            for _j in 0..num_requests {
                // let request = tonic::Request::new(HelloRequest {
                //     name: String::from("world"),
                // });
                // let _response = client.say_hello(request).await;
                client.write_all(b"hello world\n").await.unwrap();
                // io::write_all(client, "hello world\n");
            }
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time);
            let duration = duration.unwrap().as_millis();
            println!(
                "single client throughput: {} op/ms",
                num_requests / duration
            );
        });
    }

    while let Some(_response_result) = client_tasks.join_next().await {}

    Ok(())
}
