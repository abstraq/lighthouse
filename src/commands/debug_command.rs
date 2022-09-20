use std::process;
use std::sync::Arc;

use anyhow::Result;
use lighthouse::PermissionsError;
use sysinfo::{CpuExt, Pid, PidExt, ProcessExt, System, SystemExt};
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
///
/// This command can only be run by the bot owner indicated by the `ADMINISTRATOR_ID` constant.
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
		Err(anyhow::anyhow!(PermissionsError::Restricted))
	}
}

/// An `InteractionResponse` that shows information about the bot.
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
	let system = System::new_all();
	let os_info = system.long_os_version().unwrap_or_else(|| "Unknown".to_owned());
	let cpu_model = system.global_cpu_info().brand().to_owned();
	let cpu_cores = system
		.physical_core_count()
		.map(|count| count.to_string())
		.unwrap_or_else(|| "Unknown".to_owned());

	// We can unwrap here because it is guaranteed that the process ID is valid.
	let current_process = system.process(Pid::from_u32(process::id())).unwrap();

	let memory_usage_in_mb = current_process.memory() / 1000000;
	let formatted_text = indoc::formatdoc! {
		"```ansi
        \u{001b}[0;42mSystem Information                   \u{001b}[0m
        \u{001b}[0;37mOS: \u{001b}[0m{os_info}
        \u{001b}[0;37mCPU Model: \u{001b}[0m{cpu_model}
        \u{001b}[0;37mTotal CPU Cores: \u{001b}[0m{cpu_cores}

        \u{001b}[0;42mProcess Stats                        \u{001b}[0m
        \u{001b}[0;37mMemory Usage: \u{001b}[0m{memory_usage_in_mb}MB
        ```",
	};
	Ok(formatted_text)
}
