use twilight_model::channel::message::embed::{Embed, EmbedAuthor, EmbedFooter};

/// Base embed. Typically, only the `title`, `description`,
/// and `fields` properties are overwritten.
pub fn base() -> Embed {
    Embed {
        author: Some(EmbedAuthor {
            name: String::from("Chrozone"),
            url: Some(String::from("https://github.com/BastiDood/chrozone")),
            icon_url: Some(String::from(
                "https://raw.githubusercontent.com/BastiDood/chrozone/12d8f28767c27a84850f8c53a7fd7623419d23f6/docs/LOGO.png",
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
