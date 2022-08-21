mod command;
mod error;

use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::InteractionResponse,
};

/// Router for the various command handlers.
fn on_app_command(data: CommandData) -> error::Result<InteractionResponse> {
    use twilight_model::http::interaction::InteractionResponseType::ChannelMessageWithSource;

    // TODO: Verify command ID.
    let execute = match data.name.as_str() {
        "epoch" => command::epoch::execute,
        "help" => command::help::execute,
        _ => return Err(error::Error::UnknownCommand),
    };

    let payload = execute(data)?;
    Ok(InteractionResponse { kind: ChannelMessageWithSource, data: Some(payload) })
}

fn on_autocomplete(data: CommandData) -> Option<InteractionResponse> {
    use twilight_model::{
        application::{
            command::{CommandOptionChoice, CommandOptionType},
            interaction::application_command::{CommandDataOption, CommandOptionValue::Focused},
        },
        http::interaction::{InteractionResponseData, InteractionResponseType::ApplicationCommandAutocompleteResult},
    };

    if data.name.as_str() != "epoch" {
        return None;
    }

    let choices = data
        .options
        .into_iter()
        .find_map(|CommandDataOption { name, value }| match (name.as_str(), value) {
            ("timezone", Focused(comm, CommandOptionType::String)) => Some(comm.into_boxed_str()),
            _ => None,
        })
        .map(|query| crate::util::autocomplete_tz(&query, 25))
        .unwrap_or_default()
        .into_iter()
        .take(25)
        .map(|tz| {
            use alloc::borrow::ToOwned;
            let choice = tz.to_owned();
            CommandOptionChoice::String { name: choice.clone(), name_localizations: None, value: choice }
        })
        .collect();
    log::info!("Generated autocompletions: {:?}", choices);

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
        _ => {
            log::error!("Received unsupported interaction type.");
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
        use alloc::string::ToString;
        use twilight_model::{
            channel::message::MessageFlags,
            http::interaction::{InteractionResponseData, InteractionResponseType::ChannelMessageWithSource},
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
