use std::sync::Arc;

use anyhow::{Ok, Result};
use twilight_http::Client;
use twilight_model::application::interaction::{Interaction, InteractionData};
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

mod debug_command;

pub async fn handle_command(interaction: &Interaction, api: &Arc<Client>) -> Result<InteractionResponse> {
	if let Some(InteractionData::ApplicationCommand(command)) = &interaction.data {
		let response = match &*command.name {
			"debug" => debug_command::handle_debug_command(interaction, command, api)?,
			_ => unimplemented_command_response(),
		};
		Ok(response)
	} else {
		unreachable!("This function should only be run when the interaction is of type application command.")
	}
}

fn unimplemented_command_response() -> InteractionResponse {
	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.content("This command has not been implemented yet.")
		.build();

	InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	}
}

fn unimplemented_subcommand_response() -> InteractionResponse {
	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.content("This subcommand has not been implemented yet.")
		.build();

	InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	}
}
