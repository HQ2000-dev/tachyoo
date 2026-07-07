pub struct Array<T>(Box<[T]>);

async fn parse_array<R: AsyncReadExt + Unpin>(reader: &mut R, len: usize) -> io::Result<Handshake>