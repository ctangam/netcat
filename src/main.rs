use std::path::PathBuf;

use bytes::BytesMut;
use clap::{Parser, Subcommand};
use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream, UdpSocket}};


#[derive(Parser, Debug)]
enum Mode {
    #[command(about = "listen mode")]
    Listen {
        #[arg(short, long)]
        port: u16,

        #[arg(short, long)]
        udp: bool,
    },

    #[command(about = "connect mode")]
    Connect {
        host: String,

        ports: String,
    },

}

#[tokio::main]
async fn main() {
    let mode = Mode::parse();
    dbg!(&mode);

    match &mode {
        Mode::Listen { port, udp } => {
            if *udp {
                let server = UdpSocket::bind(format!("0.0.0.0:{}", port)).await.unwrap();
                
                let mut buf = BytesMut::with_capacity(1024);
                loop {
                    let (len, _) = server.recv_from(&mut buf).await.unwrap();
                    println!("Received: {}", String::from_utf8_lossy(&buf[..len]));

                    server.send_to(&buf[..len], format!("127.0.0.1:{}", port)).await.unwrap();
                    buf.clear();
                }
            } else {
                let server = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
                let (stream, _) = server.accept().await.unwrap();
                let mut stream = BufReader::new(stream);

                let mut buf = BytesMut::with_capacity(1024);
                loop {
                    stream.read_buf(&mut buf).await.unwrap();

                    if buf.is_empty() {
                        break;
                    }

                    println!("Received: {}", String::from_utf8_lossy(&buf));

                    stream.write(&buf).await.unwrap();

                    buf.clear();
                }
            }
        }

        Mode::Connect { host, ports } => {
            match ports.split_once('-') {
                Some((port, "")) => {
                    let port = port.parse::<u16>().unwrap();
                    let stream = TcpStream::connect(format!("{}:{}", host, port)).await.unwrap();
                    println!("Connected to {}:{}", host, port);
                }
                Some((start, end)) => {
                    let start = start.parse::<u16>().unwrap();
                    let end = end.parse::<u16>().unwrap();
                    for port in start..=end {
                        let stream = TcpStream::connect(format!("{}:{}", host, port)).await.unwrap();
                        println!("Connected to {}:{}", host, port);
                    }
                }

                None => {},

            }
        }
    }

    // Continued program logic goes here...
}