use serde_json::{Map, Value, json};

use crate::{
    constants::{PROTOCOL_VERSION, STRING_VERSION},
    out::{
        TransferablePacket,
        Transfer,
        packet::Packet,
        types::{UUID, string::McString},
    },
};

/*  let json=Value::Object({let object=Map::new();
    object.insert()
    ["version", Value::Object(Map::from([
        ["name", STRING_VERSION],
        ["protocol", PROTOCOL_VERSION],
    ]))],
    // wiki: "However, third-party servers such as Spigot and Paper will return full components, so make sure you can handle both. "
    ["description", Value::Object(Map::from([
        ["text", "TODO"],
    ]))],
    ["players", Value::Object(Map::from([
        ["max", STRING_VERSION],
        ["online", PROTOCOL_VERSION],
        ["sample", PROTOCOL_VERSION],
    ]))],
    "players": {
        "max": 20,
        "online": 1,
        "sample": player_samples,
    },
    "description": {
        "text": "Hello, world!"
    },
    "favicon": "data:image/png;base64,<data>",
    "enforcesSecureChat": false
]));*/
impl StatusResponse {
pub fn new(/*players_max: u32, players_online: u32, player_samples: Option<&[UUID]>*/)
 -> StatusResponse {
    let json = json!({
        "version": {
            "name": "1.21.8",
            "protocol": 772
        },
        "players": {
            "max": 20,
            "online": 1,
            "sample": [
                {
                    "name": "thinkofdeath",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                }
            ]
        },
        "description": {
            "text": "TODO"
        },
        "enforcesSecureChat": false
    }
    );

    StatusResponse {
        data: McString::try_from(json.to_string()).unwrap(),
    }
}
}

pub struct StatusResponse {
    //TODO: replace with unlimited length mc string
    data: McString<9999>,
}

impl Transfer for StatusResponse {
    fn write_bytes(&self, buf: &mut crate::out::Buffer) {
        self.data.write_bytes(buf);
    }
}

//TODO: derive macro
impl TransferablePacket for StatusResponse {
    const ID: i32=0;
}
