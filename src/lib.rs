use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use twilight_http::Client;
use twilight_model::application::interaction::{application_command::CommandData, Interaction};

#[async_trait]
pub trait CommandExecutor {
    async fn execute(
        &self,
        interaction: Interaction,
        data: Box<CommandData>,
        api: Arc<Client>,
    ) -> Result<()>;
}
