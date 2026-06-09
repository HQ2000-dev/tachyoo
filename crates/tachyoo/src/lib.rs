pub struct StartOptions {}

//fatal server error
pub struct ServerError(());

pub fn run(options: StartOptions) -> Result<(), ServerError> {
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    let client_data = ClientData::new();

    todo!()
}
