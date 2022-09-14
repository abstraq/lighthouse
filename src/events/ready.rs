use anyhow::Result;
use twilight_model::gateway::payload::incoming::Ready;

pub async fn handle_event(shard_id: u64, payload: Box<Ready>) -> Result<()> {
    let user = format!("{}#{}", payload.user.name, payload.user.discriminator);
    log::info!(
        "Shard {} ready to recieve events for user {}.",
        shard_id,
        user
    );
    Ok(())
}
