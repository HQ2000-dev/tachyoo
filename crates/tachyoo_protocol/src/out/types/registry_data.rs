use crate::out::types::{Float, identifier::Identifier, option::prefixed::PrefixedOptional};

pub struct SoundEvent {
    sound_name: Identifier,
    //wiki says two fields, but this is equivalent
    fixed_range: PrefixedOptional<Float>,
}

impl SoundEvent {
    pub fn new(sound_name: Identifier) -> SoundEvent {
        SoundEvent {
            sound_name,
            fixed_range: PrefixedOptional::none(),
        }
    }

    pub fn new_fixed_range(sound_name: Identifier, range: Float) -> SoundEvent {
        SoundEvent {
            sound_name,
            fixed_range: PrefixedOptional::some(range),
        }
    }
}

pub struct ChatType {
    //TODO
}
