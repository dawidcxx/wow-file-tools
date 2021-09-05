use super::client;
use crate::common::{err, R};
use anyhow::Context;
use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
        }
    }

    // Tries to parse a frame from the given connection and its buffer
    // Returns:
    //  - Ok(Some(frame)) => the frame read from the connection
    //  - Ok(None) => not enough data arrived, try again!
    //  - Err(err) => Malformed packet or TCP error
    pub async fn read_frame(&mut self) -> R<Option<client::Frame>> {
        let bytes_read_count = self.stream.read_buf(&mut self.buffer).await?;

        if bytes_read_count == 0 {
            // EOF or trying to read past [self.buffer]
            return err("Reset by peer".to_string());
        }

        let packet_type = self.buffer.get(0).context("Can't get packet_type")?.clone();

        if self.can_construct_frame(packet_type)? {
            let frame = self.parse_frame(packet_type)?;
            self.buffer.clear();
            return Ok(Some(frame));
        }

        return Ok(None);
    }

    fn can_construct_frame(&self, packet_type: u8) -> R<bool> {
        let is_complete = match packet_type {
            client::AUTH_LOGON_CHALLENGE_OP_CODE => {
                client::AuthLogonChallenge::can_construct_from_buffer(&self.buffer)
            }
            _ => {
                return err(format!(
                    "is_completed: Invalid packet_type: {}",
                    packet_type
                ))
            }
        };
        Ok(is_complete)
    }

    fn parse_frame(&mut self, packet_type: u8) -> R<client::Frame> {
        return match packet_type {
            client::AUTH_LOGON_CHALLENGE_OP_CODE => {
                let packet = client::AuthLogonChallenge::from_buffer(&mut self.buffer);
                Ok(client::Frame::AuthLogonChallenge(packet))
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
