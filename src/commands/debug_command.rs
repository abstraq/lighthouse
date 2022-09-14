use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use lighthouse::CommandExecutor;
use twilight_http::Client;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

pub struct DebugCommand;

/// Debug command that shows information about the bot.
#[async_trait]
impl CommandExecutor for DebugCommand {
    async fn execute(
        &self,
        interaction: Interaction,
        _: Box<CommandData>,
        discord_api: Arc<Client>,
    ) -> Result<()> {
        let interaction_id = interaction.id;
        let interaction_token = interaction.token.as_str();
        let interaction_client = discord_api.interaction(interaction.application_id);

        let hardware_info_text = create_info_code_block()?;

        let embed = EmbedBuilder::new()
            .title("Lighthouse Debug Information")
            .description(hardware_info_text)
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

        interaction_client
            .create_response(interaction_id, interaction_token, &response)
            .exec()
            .await?;

        Ok(())
    }
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
