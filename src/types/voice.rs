/// This object represents a voice note.
///
/// [The official docs](https://core.telegram.org/bots/api#voice).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Voice {
    /// Identifier for this file.
    pub file_id: String,

    /// Duration of the audio in seconds as defined by sender.
    pub duration: u32,

    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,

    /// File size.
    pub file_size: Option<u64>,
}
