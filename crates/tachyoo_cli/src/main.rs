use std::process;

//TODO: maybe independent?

use tachyoo::{error::ServerError, options::StartOptions};

#[snafu::report]
fn main() -> Result<(), ServerError> {
    let options = StartOptions::default();
    tachyoo::run(options)
}
