use std::sync::Arc;

use anyhow::Result;
use lighthouse::PermissionsError;
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

	let command_result = match &interaction.data {
		// Send application commands to the command handler.
		Some(InteractionData::ApplicationCommand(_)) => handle_command(&interaction, &api).await,
		_ => unimplemented_interaction_response(),
	};

	let response = match command_result {
		Ok(response) => response,
		Err(e) => {
			log::error!("Error handling interaction: {}", e);
			error_response(e)
		}
	};

	api.interaction(interaction.application_id)
		.create_response(interaction.id, &*interaction.token, &response)
		.exec()
		.await?;
	Ok(())
}

fn unimplemented_interaction_response() -> Result<InteractionResponse> {
	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.content("This interaction has not been implemented yet.")
		.build();

	let response = InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	};

	Ok(response)
}

fn error_response(error: anyhow::Error) -> InteractionResponse {
	let message = if let Some(perm_error) = error.downcast_ref::<PermissionsError>() {
		perm_error.to_string()
	} else {
		"An unknown error occurred.".to_string()
	};

	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.content(message)
		.build();

	InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	}
}
