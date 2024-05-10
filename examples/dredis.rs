use anyhow::Result;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6378";

    let listener = TcpListener::bind(addr).await?;
    info!(addr = %addr, "listening");

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!(raddr = %raddr, "accepted");
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!(error = %e, "process_redis_conn");
            }
        });
    }
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!(bytes = n, "read");
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                let resp = "+OK\r\n".to_string();
                stream.write_all(resp.as_bytes()).await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!(raddr = %raddr, "closed");
    Ok(())
}
