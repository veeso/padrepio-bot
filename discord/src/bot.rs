mod response;

use response::{Response, ResponseTokens};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::PadrepioClient;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use tracing::info;

const CMD_PADREPIO: &str = "padrepio";

#[allow(dead_code)]
pub struct Bot {
    client: PadrepioClient,
    guild: u64,
}

impl Bot {
    pub fn new(chatgpt_apikey: &str, guild: u64) -> anyhow::Result<Self> {
        let client = PadrepioClient::new(chatgpt_apikey)?;

        Ok(Self { client, guild })
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(self.guild);

        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name(CMD_PADREPIO)
                    .description("Chiedi a PadrePio")
                    .create_option(|option| {
                        option
                            .name("query")
                            .description("the query for padrepio")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
        })
        .await
        .unwrap();
    }

    // `interaction_create` runs when the user interacts with the bot
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // check if the interaction is a command
        if let Interaction::ApplicationCommand(command) = interaction {
            let future = match command.data.name.as_str() {
                CMD_PADREPIO => self.ask_padrepio(&command.data.options),
                command => unreachable!("Comando sconosciuto: {}", command),
            };

            // reply
            self.reply(&ctx, command.clone())
                .await
                .expect("Cannot respond to slash command");

            // execute
            let response = future.await;

            let response_content = match response {
                Ok(r) => r,
                Err(e) => {
                    error!("failed to get response {e}");
                    unreachable!("Impossibile elaborare la risposta");
                }
            };

            // send `response_content` to the discord server
            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    for token in response_content.tokens {
                        let query = if let CommandDataOptionValue::String(query) = command
                            .data
                            .options
                            .get(0)
                            .unwrap()
                            .resolved
                            .as_ref()
                            .unwrap()
                        {
                            query.clone()
                        } else {
                            String::default()
                        };
                        match token {
                            ResponseTokens::Text(text) => {
                                response.content(format!(
                                    "{} mi ha chiesto: {query}\n{text}",
                                    command.user.mention(),
                                ));
                            }
                        }
                    }

                    response
                })
                .await
                .expect("Cannot respond to slash command");
        }
    }
}

impl Bot {
    async fn ask_padrepio(&self, options: &[CommandDataOption]) -> anyhow::Result<Response> {
        let query = options.get(0).unwrap().resolved.as_ref().unwrap();
        if let CommandDataOptionValue::String(query) = query {
            let response = self.client.ask(query).await?;
            Ok(Response::default().text(response))
        } else {
            anyhow::bail!("not a valid string")
        }
    }

    async fn reply(
        &self,
        ctx: &Context,
        command: ApplicationCommandInteraction,
    ) -> anyhow::Result<()> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Grazie per avermi contattato figliolo. Aspetta un attimo che ci sto pensando..."))
            })
            .await?;

        Ok(())
    }
}
