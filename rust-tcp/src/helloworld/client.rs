use clap::Parser;
use std::time::SystemTime;
use tokio::{io::AsyncWriteExt, net::TcpStream, task::JoinSet};
use tokio::io::{AsyncBufReadExt, BufReader};

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

    let mut client_tasks = JoinSet::new();
    for _i in 0..num_clients {
        let addr = args.addr.clone();
        client_tasks.spawn(async move {
            let client = TcpStream::connect(addr.to_string()).await.unwrap();
            let (read_half, mut write_half) = client.into_split();
            let mut read_half = BufReader::new(read_half);
            let mut line = String::new();
            let start_time = SystemTime::now();
            for _j in 0..num_requests {
                write_half.write_all(b"Hello, World!\n").await.unwrap();
                read_half.read_line(&mut line).await;
                line.clear();
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
