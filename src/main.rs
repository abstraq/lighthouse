use std::env;
use std::sync::Arc;

use anyhow::Result;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use twilight_gateway::{Cluster, Intents};
use twilight_http::Client;

mod commands;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize the logger.
	let _handle = init_logger()?;

	let token = env::var("DISCORD_TOKEN")?;
	let intents = Intents::AUTO_MODERATION_EXECUTION | Intents::GUILDS | Intents::GUILD_BANS | Intents::GUILD_MEMBERS;

	// Create a cluster of shards to connect to discord gateway.
	let (cluster, events) = Cluster::new(token.to_owned(), intents).await?;
	let cluster = Arc::new(cluster);
	let cluster_spawn = Arc::clone(&cluster);

	// Spawn a task to start all the shards in the cluster.
	tokio::spawn(async move {
		cluster_spawn.up().await;
	});

	// Create an HTTP Client for interacting with discord REST API.
	let api = Arc::new(Client::new(token));

	log::info!("Start up complete, event loop is spinning, lighthouse is shining bright.");

	events::start_loop(events, api).await;

	Ok(())
}

/// Initializes the logger.
///
/// Returns a handle to the logger.
fn init_logger() -> Result<Handle> {
	let encoder = PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{T}/{h({l})}] [{M}]: {m}{n}");
	let console_appender = ConsoleAppender::builder().target(Target::Stdout).encoder(Box::new(encoder)).build();

	let log_root = Root::builder().appender("console").build(log::LevelFilter::Info);
	let log_config = Config::builder()
		.appender(Appender::builder().build("console", Box::new(console_appender)))
		.build(log_root)
		.unwrap();

	let handle = log4rs::init_config(log_config)?;
	Ok(handle)
}
