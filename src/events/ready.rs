use anyhow::Result;
use twilight_model::gateway::payload::incoming::Ready;

/// Handler for the ready event.
///
/// This is called whenever a shard is ready and fully connected.
/// At the moment, this is only used to log a confirmation.
///
/// See https://discord.com/developers/docs/topics/gateway#ready
pub async fn handle_event(shard_id: u64, _: Box<Ready>) -> Result<()> {
    log::info!("Shard {} ready to recieve events.", shard_id);
    Ok(())
}
