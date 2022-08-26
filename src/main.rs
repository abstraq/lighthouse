const LOG_CONFIG: &str = "logger.yaml";

#[tokio::main]
async fn main() {
    log4rs::init_file(LOG_CONFIG, Default::default()).unwrap();
    log::info!("Start up complete, lighthouse is shining bright.");
}
