use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, Embed, ReactionType,
    },
    http::interaction::InteractionResponseData,
};

pub fn execute() -> InteractionResponseData {
    InteractionResponseData {
        components: Some(vec![
            Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        style: ButtonStyle::Primary,
                        emoji: Some(ReactionType::Unicode { name: String::from("robot") }),
                        label: Some(String::from("Install App")),
                        url: Some(String::from("https://discord.com/oauth2/authorize?client_id=1008989318901137459")),
                        custom_id: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Danger,
                        emoji: Some(ReactionType::Unicode { name: String::from("bug") }),
                        label: Some(String::from("Report a Bug")),
                        url: Some(String::from("https://github.com/BastiDood/chrozone/issues/new")),
                        custom_id: None,
                        disabled: false,
                    }),
                    Component::Button(Button {
                        style: ButtonStyle::Secondary,
                        emoji: Some(ReactionType::Unicode { name: String::from("technologist") }),
                        label: Some(String::from("Fork the Code")),
                        url: Some(String::from("https://github.com/BastiDood/chrozone/fork")),
                        custom_id: None,
                        disabled: false,
                    }),
                ],
            }),
        ]),
        embeds: Some(vec![Embed {
            description: Some(String::from("Chrozone is an [open-source](https://github.com/BastiDood/chrozone) bot written in [Rust](https://www.rust-lang.org/) by [`@BastiDood`](https://github.com/BastiDood) for time zone utilities and timestamp formatting.")),
            ..super::embed::base()
        }]),
        ..Default::default()
    }
}
