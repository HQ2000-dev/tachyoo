use tokio::io;

use crate::out::{
    Transfer, Writable,
    types::{
        UUID,
        array::PrefixedArray,
        identifier::Identifier,
        option::{prefixed::PrefixedOptional, unprefixed::Optional},
        string::McString,
        var::int::VarInt,
    },
};

pub struct ResolvableProfile {
    profile_kind: VarInt,
    unpack: Unpack,
    body: Optional<Identifier>,
    cape: Optional<Identifier>,
    elytra: Optional<Identifier>,
    //wide=0, slim=1
    model: Optional<VarInt>,
}

enum Unpack {
    Partial {
        username: PrefixedOptional<McString<16>>,
        uuid: PrefixedOptional<UUID>,
        //max 16 (TODO)
        props: PrefixedArray<GameProfileProp>,
    },
    Complete(GameProfile),
}

#[async_trait::async_trait]
impl Transfer for Unpack {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        match self {
            Unpack::Complete(profile) => {
                profile.write_data(writeable).await?;
            }
            Unpack::Partial {
                username,
                uuid,
                props,
            } => {
                username.write_data(writeable).await?;
                uuid.write_data(writeable).await?;
                props.write_data(writeable).await?;
            }
        }

        Ok(())
    }
}

impl ResolvableProfile {
    //TODO: as needed
    //pub fn new
}

#[async_trait::async_trait]
impl Transfer for ResolvableProfile {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.profile_kind.write_data(writeable).await?;
        self.unpack.write_data(writeable).await?;
        self.body.write_data(writeable).await?;
        self.cape.write_data(writeable).await?;
        self.elytra.write_data(writeable).await?;
        self.model.write_data(writeable).await?;

        Ok(())
    }
}

pub struct GameProfile {
    uuid: UUID,
    username: McString<16>,
    //max 16 (TODO again)
    props: PrefixedArray<GameProfileProp>,
}

struct GameProfileProp {
    name: McString<64>,
    val: McString<32767>,
    signature: PrefixedOptional<McString<1024>>,
}

#[async_trait::async_trait]
impl Transfer for GameProfileProp {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.name.write_data(writeable).await?;
        self.val.write_data(writeable).await?;
        self.signature.write_data(writeable).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Transfer for GameProfile {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.uuid.write_data(writeable).await?;
        self.username.write_data(writeable).await?;
        self.props.write_data(writeable).await?;

        Ok(())
    }
}
