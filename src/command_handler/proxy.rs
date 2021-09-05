use crate::{common::R, proxy::connection::Connection};
use tokio::net::{TcpListener, TcpStream};

pub fn handle_proxy_command(
    _host: &String,
    _username: &String,
    _password: &String,
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
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Got a connection: '{:?}'.", addr);
        tokio::spawn(async move {
            process_connection(socket).await;
        });
    }
}

async fn process_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    loop {
        match connection.read_frame().await {
            Ok(None) => continue,
            Ok(Some(frame)) => {
                println!("Read frame: {:?}", frame);
                continue;
            }
            Err(err) => {
                println!("Error while trying to read_frame: {}", err);
                break;
            }
        }
    }
}
