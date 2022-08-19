mod error;

use chrono::Timelike;
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::InteractionResponse,
};

fn create_parsed_from_now() -> chrono::format::ParseResult<chrono::format::Parsed> {
    let now = chrono::Local::now();

    use chrono::Datelike;
    let mut parsed = chrono::format::Parsed::new();
    parsed.set_year(now.year().into())?;
    parsed.set_month(now.month().into())?;
    parsed.set_day(now.day().into())?;
    parsed.set_hour(now.hour().into())?;
    parsed.set_second(now.second().into())?;

    Ok(parsed)
}

fn on_app_command(data: CommandData) -> error::Result<InteractionResponse> {
    use alloc::string::ToString;
    use chrono::format::Parsed;
    use twilight_model::{
        application::interaction::application_command::{CommandDataOption, CommandOptionValue},
        channel::message::MessageFlags,
        http::interaction::{InteractionResponseData, InteractionResponseType::ChannelMessageWithSource},
    };

    // TODO: Verify command ID.

    // Set default epoch arguments
    let mut tz = chrono_tz::Tz::UTC;
    let mut parsed = create_parsed_from_now().unwrap();

    // Parse each argument
    for CommandDataOption { name, value } in data.options {
        let setter = match name.as_str() {
            "timezone" => {
                let text = if let CommandOptionValue::String(text) = value {
                    text.into_boxed_str()
                } else {
                    return Err(error::Error::Fatal);
                };
                tz = if let Ok(timezone) = text.parse::<chrono_tz::Tz>() {
                    timezone
                } else {
                    return Err(error::Error::UnknownTimezone);
                };
                continue;
            }
            "year" => Parsed::set_year,
            "month" => Parsed::set_month,
            "day" => Parsed::set_day,
            "hour" => Parsed::set_hour,
            "minute" => Parsed::set_minute,
            "secs" => Parsed::set_second,
            _ => return Err(error::Error::InvalidArgs),
        };

        let num = if let CommandOptionValue::Integer(num) = value {
            num
        } else {
            return Err(error::Error::Fatal);
        };

        if setter(&mut parsed, num).is_err() {
            return Err(error::Error::InvalidArgs);
        }
    }

    let timestamp = if let Ok(datetime) = parsed.to_datetime_with_timezone(&tz) {
        datetime.timestamp()
    } else {
        return Err(error::Error::InvalidArgs);
    };

    Ok(InteractionResponse {
        kind: ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(timestamp.to_string()),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }),
    })
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

    // TODO: Verify command ID.

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
