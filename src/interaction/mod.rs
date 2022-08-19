mod error;

use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponse};

pub fn try_respond(interaction: Interaction) -> error::Result<InteractionResponse> {
    use twilight_model::{
        application::interaction::InteractionType::{ApplicationCommand, Ping},
        http::interaction::InteractionResponseType::Pong,
    };

    match interaction.kind {
        Ping => return Ok(InteractionResponse { kind: Pong, data: None }),
        ApplicationCommand => (),
        _ => return Err(error::Error::UnsupportedInteractionType),
    }

    todo!()
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
