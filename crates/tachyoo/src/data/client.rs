use std::sync::atomic::{AtomicU32, Ordering};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(u32);
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

impl ClientId {
    pub fn new() -> ClienId {
        ClientId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
pub struct ClientData {
    pub id: ClientId,
}

impl ClientData {
    pub fn new() -> ClientData {
        ClientData {
            id: ClientId::new(),
        }
    }
}
