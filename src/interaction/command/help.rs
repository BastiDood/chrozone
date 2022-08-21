use super::error;
use twilight_model::{
    application::interaction::application_command::CommandData, http::interaction::InteractionResponseData,
};

fn home() -> InteractionResponseData {
    use alloc::{string::String, vec::Vec};
    use twilight_model::channel::{
        embed::{Embed, EmbedField},
        message::MessageFlags,
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

    InteractionResponseData {
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
    }
}

pub fn execute(data: CommandData) -> error::Result<InteractionResponseData> {
    Ok(home())
}
