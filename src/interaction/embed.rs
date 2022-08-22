use alloc::{string::String, vec::Vec};
use twilight_model::channel::embed::Embed;

/// Base embed. Typically, only the `title`, `description`,
/// and `fields` properties are overwritten.
pub fn base() -> Embed {
    Embed {
        author: None,
        color: Some(0xE5AE16),
        description: None,
        fields: Vec::new(),
        footer: None,
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
