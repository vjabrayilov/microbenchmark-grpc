use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = "127.0.0.1:50051".parse().unwrap();
    // let greeter = MyGreeter::default();

    println!("GreeterServer listening on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                // print!("{}", String::from_utf8_lossy(&buf));
                // socket
                //     .write_all(&buf[0..n])
                //     .await
                //     .expect("failed to write data to socket");
                // drop(socket);
            }
            // drop(socket);
        });
        // drop(socket);
    }
}
