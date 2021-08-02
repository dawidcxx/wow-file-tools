use tokio::net::{TcpListener, TcpStream};

use crate::common::R;

pub fn handle_proxy_command(
    host: &String,
    username: &String,
    password: &String,
) -> R<Box<dyn erased_serde::Serialize>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_proxy_server().await });
    todo!("Handle proxy command continutation not implemented")
}

async fn run_proxy_server() {
    let listener = TcpListener::bind("127.0.0.1:3724").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process_connection(socket).await;
        });
    }
}

async fn process_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub async fn read_frame(&mut self) -> R<Option<Frame>> {
        todo!()
    }
}

enum Frame {
    AuthLogonChallenge(AuthLogonChallenge),
}

struct AuthLogonChallenge {
    cmd: u8,
    error: u8,
    size: u16,
    gamename: [u8; 4],
    version: [u8; 3],
    build: u16,
    platform: [u8; 4],
    os: [u8; 4],
    country: [u8; 4],
    timezone_bias: u32,
    ip: u32,
    username_length: u8,
    username: String,
}
