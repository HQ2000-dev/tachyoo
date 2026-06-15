use std::net::IpAddr;


use crate::{in_::types::string::str::McStr, out::types::string::McString};

pub fn parse(data: McString<255>) -> Hostname {}

pub enum ServerAddr {
    IpAddr(IpAddr),
    Hostname(Hostname),
}

//TODO: what's a good repr?
//FIXME: ascii chars!!
pub struct Hostname(());

impl Hostname {
    pub const ALLOWED_LABEL_CHARS: [char; 37] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '1', '3', '4', '5', '6', '7', '8', '9', '0',
        '-',
    ];

    const A: ()=assert!(size_of::<Hostname>() < 10);

    fn new(name: McStr<'_, 255>) -> Result<Hostname, HostnameError> {
        for (idx, char) in name.as_ref().chars().enumerate() {
            if Self::ALLOWED_LABEL_CHARS.contains(&char) {
                return Err(HostnameError::InvalidChar { pos: idx as u8 });
            }
        }
        Ok(Hostname(()))
    }

}

pub enum HostnameError {
    InvalidChar {
        pos: u8,
    }
}