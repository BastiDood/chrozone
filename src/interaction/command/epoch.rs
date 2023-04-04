use super::error;
use twilight_model::{
    application::interaction::application_command::CommandData, http::interaction::InteractionResponseData,
};

/// Handler for the `/epoch` command.
pub fn execute(data: CommandData) -> error::Result<InteractionResponseData> {
    use alloc::{
        format,
        string::{String, ToString},
        vec::Vec,
    };
    use chrono::{offset::LocalResult, TimeZone};
    use twilight_model::{
        application::interaction::application_command::{CommandDataOption, CommandOptionValue},
        channel::message::{
            embed::{Embed, EmbedField},
            MessageFlags,
        },
    };

    // Set default epoch arguments
    let mut tz = None;
    let mut year = None;
    let mut month = 1;
    let mut day = 1;
    let mut hour = 0;
    let mut minute = 0;
    let mut second = 0;
    let mut preview = false;

    // Parse each argument
    for CommandDataOption { name, value } in data.options {
        log::info!("Received argument [{name}] as {value:?}.");

        if name.as_str() == "timezone" {
            let text = if let CommandOptionValue::String(text) = value {
                text.into_boxed_str()
            } else {
                log::error!("Non-string command option value encountered for timezone.");
                return Err(error::Error::Fatal);
            };
            tz = Some(match text.parse::<chrono_tz::Tz>() {
                Ok(timezone) => timezone,
                Err(err) => {
                    log::error!("Failed to set timezone: {err}.");
                    return Err(error::Error::UnknownTimezone);
                }
            });
            continue;
        }

        if name.as_str() == "preview" {
            preview = if let CommandOptionValue::Boolean(preview) = value {
                preview
            } else {
                log::error!("Non-boolean command option value encountered for preview.");
                return Err(error::Error::Fatal);
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
                    return Err(error::Error::OutOfRange);
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
                log::error!("Unsigned integer argument is out of range: {err}.");
                return Err(error::Error::OutOfRange);
            }
        };
    }

    let (tz, year) = tz.zip(year).ok_or(error::Error::MissingRequired)?;
    let timestamp = match tz.with_ymd_and_hms(year, month, day, hour, minute, second) {
        LocalResult::Single(date) => date.timestamp(),
        LocalResult::None => {
            log::error!("Unable to create date instance.");
            return Err(error::Error::InvalidArgs);
        }
        LocalResult::Ambiguous(..) => {
            log::error!("Ambiguous local time requested.");
            return Err(error::Error::AmbiguousTime);
        }
    };

    Ok(if preview {
        InteractionResponseData {
            embeds: Some(Vec::from([Embed {
                title: Some(String::from("Timestamp Preview")),
                description: Some(String::from("Here are the possible ways to format your timestamp.")),
                fields: Vec::from([
                    {
                        let value = format!("<t:{timestamp}:t>");
                        EmbedField { inline: false, name: format!("Short Time (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:T>");
                        EmbedField { inline: false, name: format!("Long Time (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:d>");
                        EmbedField { inline: false, name: format!("Short Date (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:D>");
                        EmbedField { inline: false, name: format!("Long Date (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:f>");
                        EmbedField { inline: false, name: format!("Short Full Date + Time (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:F>");
                        EmbedField { inline: false, name: format!("Long Full Date + Time (`{value}`)"), value }
                    },
                    {
                        let value = format!("<t:{timestamp}:R>");
                        EmbedField { inline: false, name: format!("Relative (`{value}`)"), value }
                    },
                ]),
                ..super::embed::base()
            }])),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }
    } else {
        InteractionResponseData {
            content: Some(timestamp.to_string()),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }
    })
}
