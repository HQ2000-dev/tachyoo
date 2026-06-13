use std::process;

#[snafu::report]
fn main() -> Result<(), ServerError> {
    let options = todo!();

    tachyoo::run(options)
}
