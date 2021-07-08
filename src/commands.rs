use crate::{acmd, nh};
use serenity::{async_trait, client::Context};
pub mod information;
pub mod misc;
use information::*;
use log::error;
use misc::*;
use serenity::{
    builder::{CreateApplicationCommandOption, CreateEmbed},
    model::interactions::{Interaction, InteractionResponseType},
};

use std::{collections::HashMap, fmt::Display};
pub struct SlashCommandMetaData {
    pub name: String,
    pub description: String,
    pub options: Option<Vec<CreateApplicationCommandOption>>,
}
pub struct SlashCommandContext {
    pub client: Context,
    pub interaction: Interaction,
}
impl SlashCommandContext {
    async fn reply(&self, content: impl Display) -> crate::RuskyResult<()> {
        if let Interaction::ApplicationCommand(command) = &self.interaction {
            command
                .create_interaction_response(&self.client, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await?;
        }
        Ok(())
    }

    async fn reply_embed(&self, embed: &mut CreateEmbed) -> crate::RuskyResult<()> {
        if let Interaction::ApplicationCommand(command) = &self.interaction {
            command
                .create_interaction_response(&self.client, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.add_embed(embed.to_owned()))
                })
                .await?;
        }
        Ok(())
    }
}
#[async_trait]
pub trait SlashCommand {
    fn information(&self) -> SlashCommandMetaData;
    async fn execute(&self, context: &SlashCommandContext) -> crate::RuskyResult<()>;
}
pub struct CommandManager {
    pub commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>>,
}
impl CommandManager {
    pub fn init() -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>> = nh!();

        acmd!(commands <== PingCommand);
        acmd!(commands <== CatCommand);
        Self { commands }
    }

    pub async fn run_command(&self, query: &str, context: &SlashCommandContext) {
        if let Some(command) = self.commands.get(query) {
            if let Err(err) = command.execute(context).await {
                error!("{:?}", err);
            }
        }
    }
}

/*
* WARNING: UNSAFE AREA!
*/
unsafe impl Sync for CommandManager {}
unsafe impl Send for CommandManager {}
