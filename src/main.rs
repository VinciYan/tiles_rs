use tiles_rs::{run_server, start_file_logger, start_logger, Args};
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

     // Setup logger
     if let Err(error) = start_file_logger(&args) {
        start_logger().expect("Failed to start logging");
        log::warn!("Using fallback logging due to an error: {:?}", error);
    };

    run_server(args.tiles_dir, args.host, args.port).await
}