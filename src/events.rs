use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use twilight_gateway::cluster::Events;
use twilight_gateway::Event;
use twilight_http::Client;

mod interaction_create;
mod ready;

pub async fn start_loop(mut event_stream: Events, api: Arc<Client>) {
	while let Some((shard_id, event)) = event_stream.next().await {
		tokio::spawn(handle_event(shard_id, event, Arc::clone(&api)));
	}
}

/// Forwards events to the appropriate handler.
async fn handle_event(shard_id: u64, event: Event, api: Arc<Client>) -> Result<()> {
	log::trace!("Received event on shard {shard_id}: {:?}", event);
	match event {
		Event::Ready(payload) => ready::handle_event(shard_id, payload).await,
		Event::InteractionCreate(event) => interaction_create::handle_event(event, api).await,
		_ => Ok(()),
	}
}
