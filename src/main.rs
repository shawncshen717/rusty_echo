use tokio::net::TcpListener; //tokio is TCP server that can accept incoming client connections asynchronously
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?; // sets up a TCP server listening on localhost at port 7878
    println!("Server running on 127.0.0.1:7878");

    loop {
        let (mut socket, addr) = listener.accept().await?; //declare socket as mutable so we can read and write through it. wait for client connection.
        println!("New connection from: {}", addr);

        // Spawn a new task for each client
        tokio::spawn(async move { //runs each block as separate task to avoid borrowing issues. allows the server to handle multiple clients concurrently.
            let mut buffer = [0; 1024]; //created buffer of 1 KB to store incoming data from client.

            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(n) => { 
                        let msg = String::from_utf8_lossy(&buffer[..n]); //takes in client's data.
                        println!("Received from {}: {}", addr, msg);

                        // Echo it back
                        if let Err(e) = socket.write_all(&buffer[..n]).await {
                            eprintln!("Failed to write to {}: {}", addr, e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}
