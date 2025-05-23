mod command;
mod embed;
mod error;

use twilight_model::{
    application::interaction::{Interaction, application_command::CommandData},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

/// Router for the various command handlers.
fn on_app_command(data: CommandData) -> error::Result<InteractionResponse> {
    // TODO: Verify command ID.
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(match data.name.as_str() {
            "epoch" => command::epoch::execute(data)?,
            "help" => command::help::execute(data).ok_or(error::Error::UnknownCommand)?,
            "info" => command::info::execute(),
            other => {
                log::error!("Invoked unknown /{other} command.");
                return Err(error::Error::UnknownCommand);
            }
        }),
    })
}

fn on_autocomplete(data: CommandData) -> Option<InteractionResponse> {
    use twilight_model::{
        application::{
            command::{CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType},
            interaction::application_command::{CommandDataOption, CommandOptionValue::Focused},
        },
        http::interaction::{
            InteractionResponseData, InteractionResponseType::ApplicationCommandAutocompleteResult,
        },
    };

    if data.name.as_str() != "epoch" {
        return None;
    }

    let choices: Vec<_> = data
        .options
        .into_iter()
        .find_map(|CommandDataOption { name, value }| match (name.as_str(), value) {
            ("timezone", Focused(comm, CommandOptionType::String)) => Some(comm.into_boxed_str()),
            _ => None,
        })
        .map(|query| crate::util::autocomplete_tz(&query, 25))
        .unwrap_or_default()
        .into_vec() // TODO: Remove this intermediate step in Edition 2024.
        .into_iter()
        .take(25)
        .map(|tz| CommandOptionChoice {
            name: tz.replace('_', " "),
            name_localizations: None,
            value: CommandOptionChoiceValue::String(String::from(tz.as_ref())),
        })
        .collect();

    log::info!("Generated {} autocompletions", choices.len());
    Some(InteractionResponse {
        kind: ApplicationCommandAutocompleteResult,
        data: Some(InteractionResponseData { choices: Some(choices), ..Default::default() }),
    })
}

fn try_respond(interaction: Interaction) -> error::Result<InteractionResponse> {
    use twilight_model::{
        application::interaction::{
            InteractionData,
            InteractionType::{ApplicationCommand, ApplicationCommandAutocomplete, Ping},
        },
        http::interaction::InteractionResponseType::Pong,
    };

    let is_comm = match interaction.kind {
        ApplicationCommand => true,
        ApplicationCommandAutocomplete => false,
        Ping => {
            log::info!("Received a ping.");
            return Ok(InteractionResponse { kind: Pong, data: None });
        }
        other => {
            log::error!("Received unsupported interaction type {other:?}.");
            return Err(error::Error::UnsupportedInteractionType);
        }
    };

    let data = match interaction.data.ok_or(error::Error::MissingPayload)? {
        InteractionData::ApplicationCommand(data) => *data,
        _ => {
            log::error!("Missing payload from application command invocation.");
            return Err(error::Error::Fatal);
        }
    };

    if is_comm {
        log::info!("Received application command.");
        on_app_command(data)
    } else {
        log::info!("Received autocompletion request.");
        on_autocomplete(data).ok_or(error::Error::UnknownCommand)
    }
}

pub fn respond(interaction: Interaction) -> InteractionResponse {
    try_respond(interaction).unwrap_or_else(|err| {
        use std::string::ToString;
        use twilight_model::{
            channel::message::MessageFlags,
            http::interaction::{
                InteractionResponseData, InteractionResponseType::ChannelMessageWithSource,
            },
        };
        InteractionResponse {
            kind: ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(err.to_string()),
                flags: Some(MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
        }
    })
}
