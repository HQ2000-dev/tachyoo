pub mod play;
pub mod raw;

use std::default;

use crate::{in_::{
    packets::play::Play, types::{Long, handshake::Handshake, login::hello::Hello, status::PingRequest, string::string::McString},
}, out::types::UUID};

#[derive(Default, Debug)]
pub enum Compression {
    #[default]
    Uncompressed,
    Compressed {
        threshold: u16,
    },
}

const _: () = {
    //we don't want the enum to get too big
    // TODO: consider boxing bigger variants or dynamic dispatch
    assert!(size_of::<Packet>() < 50);
};

#[derive(Debug)]
pub enum Packet {
    Handshake(Handshake),
    Status(Status),
    Login(Login),
    Config(Config),
    Play(Play),
}

#[derive(Debug)]
pub enum Status {
    //0
    StatusRequest,
    //1
    PingRequest(PingRequest),
}

impl Status {
    pub const fn protocol_id(&self) -> u8 {
        match self {
            Self::StatusRequest => 0x00,
            Self::PingRequest(_) => 0x01,
        }
    }
}

#[derive(Debug)]
pub enum Login {
    //0
    Hello(Hello),
    Key(Key),
    CustomQueryAnswer ,
    LoginAcknowledged ,
    CookieResponse ,
}

impl Login {
    pub const fn protocol_id(&self) -> u8 {
        match self {
            Self::Hello(_) => 0,
            Self::Key => 1,
            Self::CustomQueryAnswer => 2,
            Self::LoginAcknowledged => 3,
            Self::CookieResponse => 4,
        }
    }
}

#[derive(Debug)]
pub enum Config {
    ClientInformation = 0,
    CookieResponse = 1,
    CustomPayload = 2,
    FinishConfiguration = 3,
    KeepAlive = 4,
    Pong = 5,
    ResourcePack = 6,
    SelectKnownPacks = 7,
    CustomClickAction = 8,
    AcceptCodeOfConduct = 9,
}
