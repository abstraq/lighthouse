use std::sync::Arc;

use anyhow::{Ok, Result};
use lighthouse::CommandExecutor;
use twilight_http::Client;
use twilight_model::application::interaction::{application_command::CommandData, Interaction};

use self::debug_command::DebugCommand;

mod debug_command;

pub async fn handle_command(
    interaction: Interaction,
    data: Box<CommandData>,
    api: Arc<Client>,
) -> Result<()> {
    match &*data.name {
        "debug" => DebugCommand.execute(interaction, data, api).await,
        _ => Ok(()),
    }
}
