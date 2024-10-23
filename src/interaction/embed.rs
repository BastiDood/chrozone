use twilight_model::channel::message::embed::{Embed, EmbedAuthor, EmbedFooter};

/// Base embed. Typically, only the `title`, `description`,
/// and `fields` properties are overwritten.
pub fn base() -> Embed {
    Embed {
        author: Some(EmbedAuthor {
            name: String::from("Chrozone"),
            url: Some(String::from("https://github.com/BastiDood/chrozone")),
            icon_url: Some(String::from(
                "https://cdn.discordapp.com/app-icons/1008989318901137459/777734d2d2a26c8d5f675931d97c3f85.png",
            )),
            proxy_icon_url: None,
        }),
        color: Some(0xE5AE16),
        description: None,
        fields: Vec::new(),
        footer: Some(EmbedFooter {
            text: String::from("By BastiDood"),
            icon_url: Some(String::from("https://avatars.githubusercontent.com/u/39114273")),
            proxy_icon_url: None,
        }),
        image: None,
        kind: String::from("rich"),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: None,
        url: None,
        video: None,
    }
}
