use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = baggins::server::run().await {
        eprintln!("baggins server failed: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
