use anyhow::Result;
use futures::StreamExt;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use std::env;
use std::sync::Arc;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::cluster::Events;
use twilight_gateway::{Cluster, Event, Intents};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    let _handle = init_logger()?;

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::AUTO_MODERATION_EXECUTION
        | Intents::GUILDS
        | Intents::GUILD_BANS
        | Intents::GUILD_MEMBERS;

    // Initialize the cluster for the gateway and get the stream of events.
    let (_cluster, event_stream) = init_gateway_cluster(token.to_owned(), intents).await?;

    // Create a HTTP Client for interacting with discord REST API.
    let discord_api = Arc::new(HttpClient::new(token));

    // Create a cache to store the data from the gateway.
    //.We only want to cache guild member information.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MEMBER)
        .build();

    log::info!("Start up complete, event loop is spinning, lighthouse is shining bright.");

    start_event_loop(event_stream, cache, discord_api).await;

    Ok(())
}

/// Initializes the logger.
///
/// Returns a handle to the logger.
fn init_logger() -> Result<Handle> {
    let encoder = PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{T}/{h({l})}] [{M}]: {m}{n}");
    let console_appender = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(encoder))
        .build();

    let log_root = Root::builder()
        .appender("console")
        .build(log::LevelFilter::Info);
    let log_config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(log_root)
        .unwrap();

    let handle = log4rs::init_config(log_config)?;
    Ok(handle)
}

/// Initializes the gateway cluster.
///
/// Returns a tuple containing the cluster and the stream of events.
async fn init_gateway_cluster(token: String, intents: Intents) -> Result<(Arc<Cluster>, Events)> {
    // Create a cluster of shards to connect to discord gateway.
    let (cluster, events) = Cluster::new(token, intents).await?;
    let cluster = Arc::new(cluster);
    let cluster_spawn = Arc::clone(&cluster);

    // Spawn a task to start all the shards in the cluster.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    Ok((cluster, events))
}

/// Starts an event loop to handle the events from the gateway.
async fn start_event_loop(mut event_stream: Events, cache: InMemoryCache, rest: Arc<HttpClient>) {
    while let Some((shard_id, event)) = event_stream.next().await {
        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, Arc::clone(&rest)));
    }
}

/// Forwards events to the appropriate handler.
async fn handle_event(shard_id: u64, event: Event, http: Arc<HttpClient>) -> Result<()> {
    log::trace!("Received event on shard {shard_id}: {:?}", event);
    match event {
        Event::Ready(event) => {
            let current_user = event.user;
            log::info!(
                "Shard {shard_id} ready, Logged in as {current_user_name}#{current_user_discrim} ({current_user_id}).",
                current_user_name = current_user.name,
                current_user_discrim = current_user.discriminator,
                current_user_id = current_user.id.get(),
            );
        }
        _ => {}
    }
    Ok(())
}
