pub mod packet;
pub(super) mod raw_packet;
pub mod types;

//types that implement conversion into a transferable repr
pub trait IntoTransferable: Sized {
    type TransferRepr: Transfer;
    fn into_transferable(self) -> Self::TransferRepr;
}

//types that are reprs for transmission
pub trait Transfer: for<'a> Into<&'a [u8]> {}
