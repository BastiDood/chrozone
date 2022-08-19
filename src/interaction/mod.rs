mod error;

use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::InteractionResponse,
};

fn on_app_command(data: CommandData) -> error::Result<InteractionResponse> {
    todo!()
}

fn on_autocomplete(data: CommandData) -> InteractionResponse {
    use alloc::borrow::ToOwned;
    use twilight_model::{
        application::{
            command::{CommandOptionChoice, CommandOptionType},
            interaction::application_command::{CommandDataOption, CommandOptionValue::Focused},
        },
        http::interaction::{InteractionResponseData, InteractionResponseType::ChannelMessageWithSource},
    };

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
        .map(|tz| CommandOptionChoice::String {
            name: alloc::string::String::from("timezone"),
            name_localizations: None,
            value: tz.to_owned(),
        })
        .collect();

    InteractionResponse {
        kind: ChannelMessageWithSource,
        data: Some(InteractionResponseData { choices: Some(choices), ..Default::default() }),
    }
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
        Ping => return Ok(InteractionResponse { kind: Pong, data: None }),
        _ => return Err(error::Error::UnsupportedInteractionType),
    };

    let data = match interaction.data.ok_or(error::Error::MissingPayload)? {
        InteractionData::ApplicationCommand(data) => *data,
        _ => return Err(error::Error::Fatal),
    };

    Ok(if is_comm { on_app_command(data)? } else { on_autocomplete(data) })
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
