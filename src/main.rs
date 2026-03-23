#[tokio::main]
async fn main() {
    if let Err(error) = backtoschool_rust::run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
