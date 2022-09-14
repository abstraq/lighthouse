use std::sync::Arc;

use anyhow::Result;
use twilight_http::Client;
use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

use crate::commands::handle_command;

pub async fn handle_event(event: Box<InteractionCreate>, api: Arc<Client>) -> Result<()> {
    let interaction = event.0;
    match &interaction.data {
        Some(InteractionData::ApplicationCommand(command)) => {
            handle_command(interaction.clone(), command.clone(), api).await
        }
        _ => Ok(()),
    }
}
