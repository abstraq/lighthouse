use std::sync::Arc;

use anyhow::Result;
use twilight_http::Client;
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::handle_command;

/// Handler for the interaction create event.
///
/// This is called whenever a user uses one of the new interactions
/// (slash commands, buttons, etc).
///
/// See https://discord.com/developers/docs/topics/gateway#interaction-create
pub async fn handle_event(event: Box<InteractionCreate>, api: Arc<Client>) -> Result<()> {
	let interaction = Box::new(event.0);

	let response = match &interaction.data {
		// Send application commands to the command handler.
		Some(InteractionData::ApplicationCommand(_)) => handle_command(&interaction, &api).await?,
		_ => unimplemented_interaction_response(),
	};

	api.interaction(interaction.application_id)
		.create_response(interaction.id, &*interaction.token, &response)
		.exec()
		.await?;

	Ok(())
}

fn unimplemented_interaction_response() -> InteractionResponse {
	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.content("This interaction has not been implemented yet.")
		.build();

	InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	}
}
