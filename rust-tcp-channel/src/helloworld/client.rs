mod tcp;

use std::collections::HashMap;
use std::sync::Arc;
use clap::Parser;
use std::time::SystemTime;
use tokio::{io::AsyncWriteExt, net::TcpStream, task::JoinSet};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use crate::tcp::TcpLink;

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

    let channels = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    let channels_map = channels.clone();
    let client = Arc::new(TcpLink::new(&args.addr, channels_map).await);

    let mut client_tasks = JoinSet::new();
    for channel_id in 0..num_clients {
        let (sender, mut receiver) = mpsc::channel(num_requests as usize);
        let mut channels = channels.lock().await;
        channels.insert(channel_id, sender);
        drop(channels);
        let client = client.clone();
        client_tasks.spawn(async move {
            let start_time = SystemTime::now();
            for _j in 0..num_requests {
                client.send_await_response(channel_id, "Hello, World!").await;
                receiver.recv().await.unwrap();
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
