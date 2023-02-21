use clap::Parser;
use hyper::{body::HttpBody as _, Client, Uri};
use std::time::SystemTime;
use tokio::{net::TcpStream, task::JoinSet};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 100000)]
    num_requests: u64,

    #[arg(long, default_value_t = 1)]
    num_clients: u64,

    #[arg(long, default_value_t = String::from("0.0.0.0:10000"))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let num_requests = args.num_requests;
    let num_clients = args.num_clients;

    let client = Client::new();

    let mut client_tasks = JoinSet::new();
    for _i in 0..num_clients {
        let client = client.clone();
        client_tasks.spawn(async move {
            let start_time = SystemTime::now();
            for _j in 0..num_requests {
                let request = hyper::Request::builder()
                    .method("POST")
                    .uri("http://172.31.15.160:10000")
                    // .header(hyper::header::HOST, "0.0.0.0")
                    .body(hyper::Body::from("Hello, World!")).unwrap();
                let res = client.request(request).await.unwrap();
                let buf = hyper::body::to_bytes(res).await.unwrap();
            }
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time);
            let duration = duration.unwrap().as_secs_f64();
            let tput = num_requests as f64 / duration;
            println!("single client throughput: {} op/s", tput);
            tput
        });
    }

    let mut total_tput = 0.0;
    while let Some(result) = client_tasks.join_next().await {
        let result = match result {
            Ok(result) => result,
            Err(_) => 0.0,
        };
        total_tput += result;
    }
    println!("total throughput: {} op/s", total_tput);

    Ok(())
}
