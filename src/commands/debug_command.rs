use std::sync::Arc;

use anyhow::Result;
use lighthouse::PermissionsError;
use twilight_http::Client;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use super::unimplemented_subcommand_response;

/// ID of user allowed to run this command (me).
const ADMINISTRATOR_ID: Id<UserMarker> = Id::new(671477574932627516);

/// Debug command that shows information about the bot.
pub fn handle_debug_command(interaction: &Interaction, data: &CommandData, _: &Arc<Client>) -> Result<InteractionResponse> {
	// The debug command should only be allowed to run by the bot owner.
	if interaction.author_id() == Some(ADMINISTRATOR_ID) {
		// We can unwrap here because it is expected that the debug command is always sent with a subcommand.
		let subcommand = data.options.first().unwrap();
		let subcommand_name = &*subcommand.name;

		let response = match subcommand_name {
			"info" => debug_info_response()?,
			_ => unimplemented_subcommand_response(),
		};
		Ok(response)
	} else {
		Err(anyhow::anyhow!("Rawr"))
	}
}

fn debug_info_response() -> Result<InteractionResponse> {
	let embed = EmbedBuilder::new()
		.title("Lighthouse Debug Information")
		.description(create_info_code_block()?)
		.color(0x545863)
		.validate()?
		.build();

	let data = InteractionResponseDataBuilder::new()
		.flags(MessageFlags::EPHEMERAL)
		.embeds([embed])
		.build();

	let response = InteractionResponse {
		kind: InteractionResponseType::ChannelMessageWithSource,
		data: Some(data),
	};

	Ok(response)
}

fn create_info_code_block() -> Result<String> {
	let cpu_info = procfs::CpuInfo::new()?;
	let cpu_model = cpu_info.model_name(0).unwrap_or("Unknown");
	let cpu_cores = cpu_info.num_cores();

	let os_info = os_info::get();
	let lighthouse_process = procfs::process::Process::myself()?;
	let lighthouse_process_statm = lighthouse_process.statm()?;
	// TODO: Calculate page size at runtime or use a crate for getting process memory usage.
	let used_memory_in_mb = (lighthouse_process_statm.resident * 4) / 1000;

	let formatted_text = indoc::formatdoc! {
		"```ansi
        \u{001b}[0;33mSystem Information
        \u{001b}[0;32mOS: \u{001b}[0m{os_info}
        \u{001b}[0;32mCPU Model: \u{001b}[0m{cpu_model}
        \u{001b}[0;32mTotal CPU Cores: \u{001b}[0m{cpu_cores}

        \u{001b}[0;33mProcess Stats
        \u{001b}[0;32mMemory Usage: \u{001b}[0m{used_memory_in_mb}MB
        ```",
	};
	Ok(formatted_text)
}
