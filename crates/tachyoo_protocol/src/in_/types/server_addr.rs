use std::{io, net::IpAddr};

use tokio::io::AsyncReadExt;

use crate::{in_::{types::string::str::McStr}, out::types::string::McString};


pub async fn parse_server_addr<R: AsyncReadExt + Unpin>(
    reader: &mut R,
    len: usize,
) -> io::Result<ServerAddr> {
    let mut buf = [0u8; 255];
    reader.read_exact(&mut buf[0..len]).await?;

    Ok(ServerAddr::Data(Vec::from(&buf[0..len])))
}

#[derive(Debug)]
pub enum ServerAddr {
    Data(Vec<u8>), //IpAddr(IpAddr),
                   //Hostname(Hostname),
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

    //tmp, just to try test it
    const A: () = assert!(size_of::<Hostname>() < 10);

    //for manual creation?
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
    InvalidChar { pos: u8 },
}
