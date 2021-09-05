use anyhow::Context;
use bytes::{Buf, BytesMut};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::common::{err, R};

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
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process_connection(socket).await;
        });
    }
}

async fn process_connection(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    while let Ok(Some(frame)) = connection.read_frame().await {
        println!("frame: {:?}", frame)
    }

    loop {
        match connection.read_frame().await {
            Ok(None) => continue,
            Ok(Some(frame)) => {
                println!("Read frame: {:?}", frame);
                break;
            }
            Err(err) => {
                println!("Error while trying to read_frame: {}", err);
                break;
            }
        }
    }
}

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
            cursor: 0,
        }
    }

    // Tries to parse a frame from the given connection and its buffer
    // Returns:
    //  - Ok(Some(frame)) => the frame read from the connection
    //  - Ok(None) => not enough data arrived, try again!
    //  - Err(err) => Malformed packet or TCP error
    pub async fn read_frame(&mut self) -> R<Option<Frame>> {
        let bytes_read_count = self.stream.read(&mut self.buffer[self.cursor..]).await?;
        self.cursor += bytes_read_count;

        if bytes_read_count == 0 {
            // EOF or trying to read past [self.buffer]
            return err("Reset by peer".to_string());
        }

        let packet_type = self.buffer.get(1).context("Can't get packet_type")?.clone();
        let packet_current_size = self.buffer.len();

        if self.is_completed(packet_current_size, packet_type)? {
            let frame = self.parse_frame(packet_type)?;
            self.buffer.clear();
            return Ok(Some(frame));
        }

        return Ok(None);
    }

    fn is_completed(&self, current_size: usize, packet_type: u8) -> R<bool> {
        let is_complete = match packet_type {
            1 => AuthLogonChallenge::is_fully_read(current_size, &self.buffer),
            _ => return err("Invalid packet type".to_string()),
        };
        Ok(is_complete)
    }

    fn parse_frame(&mut self, packet_type: u8) -> R<Frame> {
        return match packet_type {
            1 => {
                let packet = AuthLogonChallenge::from_buffer(&mut self.buffer);
                Ok(Frame::AuthLogonChallenge(packet))
            }
            _ => {
                return err(format!(
                    "parse_frame: unknown packet_type passed: {}",
                    packet_type
                ))
            }
        };
    }
}

#[derive(Debug)]
enum Frame {
    AuthLogonChallenge(AuthLogonChallenge),
}

#[derive(Debug)]
struct AuthLogonChallenge {
    cmd: u8,             // offset :: 1
    error: u8,           // offset :: 2
    size: u16,           // offset :: 4
    game_name: [u8; 4],  // offset :: 8
    version: [u8; 3],    // offset :: 11
    build: u16,          // offset :: 13
    platform: [u8; 4],   // offset :: 17
    os: [u8; 4],         // offset :: 21
    country: [u8; 4],    // offset :: 25
    timezone_bias: u32,  // offset :: 29
    ip: u32,             // offset :: 33
    username_length: u8, // offset :: 34
    username: String,
}

impl AuthLogonChallenge {
    fn from_buffer(buffer: &mut BytesMut) -> Self {
        assert!(
            buffer.len() >= 34,
            "AuthLogonChallenge packet must be at least 34 bytes"
        );
        let cmd = buffer.get_u8();
        let error = buffer.get_u8();
        let size = buffer.get_u16();
        let game_name = [
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
        ];
        let version = { [buffer.get_u8(), buffer.get_u8(), buffer.get_u8()] };
        let build = buffer.get_u16();
        let platform = [
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
        ];
        let os = [
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
        ];
        let country = [
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
            buffer.get_u8(),
        ];
        let timezone_bias = buffer.get_u32();
        let ip = buffer.get_u32();
        let username_length = buffer.get_u8();
        return AuthLogonChallenge {
            cmd,
            error,
            size,
            game_name,
            version,
            build,
            platform,
            os,
            country,
            timezone_bias,
            ip,
            username_length,
            username: "".to_string(),
        };
    }

    fn is_fully_read(current_size: usize, buf: &BytesMut) -> bool {
        const BASE_SIZE: usize = 34;

        if current_size >= BASE_SIZE {
            let username_length = buf.get(BASE_SIZE).unwrap().clone() as usize;
            return current_size >= (BASE_SIZE + username_length);
        } else {
            return false;
        }
    }
}
