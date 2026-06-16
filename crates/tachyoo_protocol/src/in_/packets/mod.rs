pub mod play;

use crate::in_::{packets::play::Play, types::Long};

const _: () = {
    //we don't want the enum to get too big
    // TODO: consider boxing bigger variants or dynamic dispatch
    assert!(size_of::<Packet>() < 20);
};

pub enum Packet {
    Status(Status),
    Login(Login),
    Config(Config),
    Play(Play),
}

pub enum Status {
    //0
    StatusRequest,
    //1
    PingRequest { timestamp: Long },
}

pub enum Login {
    Hello = 0,
    Key = 1,
    CustomQueryAnswer = 2,
    LoginAcknowledged = 3,
    CookieResponse = 4,
}

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

