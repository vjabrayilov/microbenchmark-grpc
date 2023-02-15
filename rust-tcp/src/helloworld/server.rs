use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = "0.0.0.0:10000".parse().unwrap();

    println!("server listening on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (read_half, mut write_half) = socket.into_split();
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 {
                    break
                }
                write_half.write(line.as_bytes()).await;
                line.clear();
            }
        });
    }
}
