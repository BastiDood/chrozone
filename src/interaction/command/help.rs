use alloc::{string::String, vec::Vec};
use twilight_model::{
    application::interaction::application_command::CommandData,
    channel::message::embed::{Embed, EmbedField},
    http::interaction::InteractionResponseData,
};

fn epoch() -> Embed {
    Embed {
        title: Some(String::from("`/epoch` Command")),
        description: Some(String::from("Generates the ISO-8601 timestamp at a given date and timezone.")),
        fields: Vec::from([
            EmbedField {
                inline: false,
                name: String::from("`timezone`"),
                value: String::from("Required. Must be an officially registered timezone from the IANA Time Zone Database. For convenience, dynamic autocompletions are enabled."),
            },
            EmbedField {
                inline: false,
                name: String::from("`year`"),
                value: String::from("Required. Must be a reasonably valid year."),
            },
            EmbedField {
                inline: false,
                name: String::from("`month`"),
                value: String::from("Must be a value from `1` (default) to `12`, where `1` is January and `12` is December."),
            },
            EmbedField {
                inline: false,
                name: String::from("`day`"),
                value: String::from("Must be a value from `1` (default) to `31`. Note that the days `29` to `31` are only invalid for certain months."),
            },
            EmbedField {
                inline: false,
                name: String::from("`hour`"),
                value: String::from("Must be a value from `0` (default) to `23` (i.e. 24-hour format), where `0` is `12am` and `23` is `11pm`"),
            },
            EmbedField {
                inline: false,
                name: String::from("`minute`"),
                value: String::from("Must be a value from `0` (default) to `59`."),
            },
            EmbedField {
                inline: false,
                name: String::from("`second`"),
                value: String::from("Must be a value from `0` (default) to `60`. The 60th second accounts for possible leap seconds."),
            },
            EmbedField {
                inline: false,
                name: String::from("`preview`"),
                value: String::from("Enables preview mode for all timestamp formatting options. Defaults to `false`."),
            },
        ]),
        ..super::embed::base()
    }
}

fn help() -> Embed {
    Embed {
        title: Some(String::from("`/help` Command")),
        description: Some(String::from("Provides extra details for specific commands.")),
        fields: Vec::from([
            EmbedField {
                inline: false,
                name: String::from("`/epoch`"),
                value: String::from("Shows extra information for each argument of the `/epoch` command."),
            },
            EmbedField {
                inline: false,
                name: String::from("`/help`"),
                value: String::from("Provides extra details on how to use the `/help` command."),
            },
        ]),
        ..super::embed::base()
    }
}

fn default() -> Embed {
    Embed {
        title: Some(String::from("Chrozone Help")),
        description: Some(String::from("List of supported commands and their arguments.")),
        fields: Vec::from([
            EmbedField { inline: false, name: String::from("`/help`"), value: String::from("Summon this help menu.") },
            EmbedField {
                inline: false,
                name: String::from("`/epoch timezone year [month] [day] [hour] [min] [sec] [preview]`"),
                value: String::from("Get the ISO-8601 timestamp (in seconds) for some date and timezone."),
            },
        ]),
        ..super::embed::base()
    }
}

pub fn execute(mut data: CommandData) -> Option<InteractionResponseData> {
    use twilight_model::{
        application::interaction::application_command::{CommandDataOption, CommandOptionValue},
        channel::message::MessageFlags,
    };

    let get_embed = match data.options.pop() {
        Some(CommandDataOption { value: CommandOptionValue::String(val), .. }) => match val.as_str() {
            "epoch" => epoch,
            "help" => help,
            _ => return None,
        },
        _ => default,
    };

    Some(InteractionResponseData {
        embeds: Some(Vec::from([get_embed()])),
        flags: Some(MessageFlags::EPHEMERAL),
        ..Default::default()
    })
}
