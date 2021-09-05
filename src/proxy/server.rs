#[derive(Debug)]
pub enum Frame {
    AuthLogonChallenge(AuthLogonChallenge),
}

pub const AUTH_LOGON_CHALLENGE_OP_CODE: u8 = 0;

pub struct AuthLogonChallenge {
    cmd: u8,
    protocol_version: u8,
    result: u8,
}