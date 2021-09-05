use bytes::{Buf, BytesMut};
use serde::__private::from_utf8_lossy;

#[derive(Debug)]
pub enum Frame {
    AuthLogonChallenge(AuthLogonChallenge),
}

pub const AUTH_LOGON_CHALLENGE_OP_CODE: u8 = 0;

#[derive(Debug)]
pub struct AuthLogonChallenge {
    cmd: u8,             // offset :: 0
    error: u8,           // offset :: 1
    size: u16,           // offset :: 3
    game_name: [u8; 4],  // offset :: 7
    version: [u8; 3],    // offset :: 10
    build: u16,          // offset :: 12
    platform: [u8; 4],   // offset :: 16
    os: [u8; 4],         // offset :: 20
    country: [u8; 4],    // offset :: 24
    timezone_bias: u32,  // offset :: 28
    ip: u32,             // offset :: 32
    username_length: u8, // offset :: 33
    username: String,    // offset :: 34
}

impl AuthLogonChallenge {
    pub fn from_buffer(buffer: &mut BytesMut) -> Self {
        assert!(buffer.len() >= 34, "Invalid packet size");
        let cmd = buffer.get_u8();
        assert!(cmd == AUTH_LOGON_CHALLENGE_OP_CODE, "Invalid Opcode");
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
        let username = from_utf8_lossy(&buffer.chunk()).to_string();
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
            username,
        };
    }

    pub fn can_construct_from_buffer(buf: &BytesMut) -> bool {
        const BASE_SIZE: usize = 33;
        let current_buffer_size = buf.len();
        if current_buffer_size >= BASE_SIZE {
            let username_length = buf.get(BASE_SIZE).unwrap().clone() as usize;
            return current_buffer_size >= (BASE_SIZE + username_length);
        } else {
            return false;
        }
    }
}
