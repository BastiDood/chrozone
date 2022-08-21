mod error;

use alloc::string::ToString;
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    http::interaction::InteractionResponse,
};

/// Handler for the `/epoch` command.
fn on_epoch_command(data: CommandData) -> error::Result<InteractionResponse> {
    use chrono::{offset::LocalResult, TimeZone};
    use twilight_model::{
        application::interaction::application_command::{CommandDataOption, CommandOptionValue},
        channel::message::MessageFlags,
        http::interaction::{InteractionResponseData, InteractionResponseType::ChannelMessageWithSource},
    };

    // Set default epoch arguments
    let mut tz = chrono_tz::Tz::UTC;
    let mut year = None;
    let mut month = 1;
    let mut day = 1;
    let mut hour = 0;
    let mut minute = 0;
    let mut second = 0;

    // Parse each argument
    for CommandDataOption { name, value } in data.options {
        log::info!("Received argument {name} as {value:?}.");

        if name.as_str() == "timezone" {
            let text = if let CommandOptionValue::String(text) = value {
                text.into_boxed_str()
            } else {
                log::error!("Non-string command option value encountered for timezone.");
                return Err(error::Error::Fatal);
            };
            tz = match text.parse::<chrono_tz::Tz>() {
                Ok(timezone) => timezone,
                Err(err) => {
                    log::error!("Failed to set timezone: {err}.");
                    return Err(error::Error::UnknownTimezone);
                }
            };
            continue;
        }

        let num = if let CommandOptionValue::Integer(num) = value {
            num
        } else {
            log::error!("Incorrect command option value received.");
            return Err(error::Error::Fatal);
        };

        if name.as_str() == "year" {
            year = Some(match i32::try_from(num) {
                Ok(val) => val,
                Err(err) => {
                    log::error!("Integer argument is out of range: {err}.");
                    return Err(error::Error::InvalidArgs);
                }
            });
            continue;
        }

        let target = match name.as_str() {
            "month" => &mut month,
            "day" => &mut day,
            "hour" => &mut hour,
            "minute" => &mut minute,
            "secs" => &mut second,
            other => {
                log::error!("Unable to parse command name {other}.");
                return Err(error::Error::InvalidArgs);
            }
        };

        *target = match u32::try_from(num) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Integer argument is out of range: {err}.");
                return Err(error::Error::InvalidArgs);
            }
        };
    }

    let year = year.ok_or(error::Error::InvalidArgs)?;
    let date = match tz.ymd_opt(year, month, day) {
        LocalResult::Single(date) => date,
        LocalResult::None => {
            log::error!("Unable to create date instance.");
            return Err(error::Error::InvalidArgs);
        }
        LocalResult::Ambiguous(..) => {
            log::error!("Ambiguous local time requested.");
            return Err(error::Error::InvalidArgs);
        }
    };

    let msg = date.and_hms(hour, minute, second).timestamp().to_string();
    Ok(InteractionResponse {
        kind: ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(msg),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }),
    })
}

/// Handler for the `/help` command.
fn on_help_command() -> InteractionResponse {
    use alloc::{string::String, vec::Vec};
    use twilight_model::{
        channel::{
            embed::{Embed, EmbedField},
            message::MessageFlags,
        },
        http::interaction::{InteractionResponseData, InteractionResponseType::ChannelMessageWithSource},
    };

    let fields = Vec::from([
        EmbedField { inline: false, name: String::from("`/help`"), value: String::from("Summon this help menu.") },
        EmbedField {
            inline: false,
            name: String::from("`/epoch timezone year [month] [day] [hour] [min] [sec]`"),
            value: String::from(
                "Get the ISO-8601 timestamp (in seconds) for some date and timezone. Autocompletions enabled.",
            ),
        },
    ]);

    InteractionResponse {
        kind: ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            embeds: Some(Vec::from([Embed {
                author: None,
                color: Some(0xE5AE16),
                description: Some(String::from("List of supported commands and their arguments.")),
                fields,
                footer: None,
                image: None,
                kind: String::from("rich"),
                provider: None,
                thumbnail: None,
                timestamp: None,
                title: Some(String::from("Chrozone Help")),
                url: None,
                video: None,
            }])),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }),
    }
}

/// Router for the various command handlers.
fn on_app_command(data: CommandData) -> error::Result<InteractionResponse> {
    // TODO: Verify command ID.
    match data.name.as_str() {
        "epoch" => on_epoch_command(data),
        "help" => Ok(on_help_command()),
        _ => Err(error::Error::UnknownCommand),
    }
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
