use serde::{Deserialize, Serialize};

use crate::files::*;
use crate::types::*;
use crate::{RequestFiles, TelegramRequest};

/// ChatID represents a possible type of value for requests.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatID {
    /// A chat's numeric ID.
    Identifier(i64),
    /// A username for a channel.
    Username(String),
}

impl Message {
    pub fn chat_id(&self) -> ChatID {
        ChatID::Identifier(self.chat.id)
    }
}

impl From<i64> for ChatID {
    fn from(item: i64) -> Self {
        ChatID::Identifier(item)
    }
}

impl From<i32> for ChatID {
    fn from(item: i32) -> Self {
        ChatID::Identifier(item as i64)
    }
}

impl From<&str> for ChatID {
    fn from(item: &str) -> Self {
        ChatID::Username(item.into())
    }
}

impl Default for ChatID {
    fn default() -> Self {
        ChatID::Identifier(0)
    }
}

/// ForceReply allows you to default users to replying to your message.
#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    /// This must be set to `true` to operate correctly.
    pub force_reply: bool,
    /// If only the user you are mentioning or replying to should be defaulted
    /// to replying to your message, or if it should default to replying for all
    /// members of the chat.
    pub selective: bool,
}

impl ForceReply {
    /// Create a [ForceReply] with selectivity.
    pub fn selective() -> Self {
        Self {
            force_reply: true,
            selective: true,
        }
    }
}

impl Default for ForceReply {
    /// Create a [ForceReply] without selectivity.
    fn default() -> Self {
        Self {
            force_reply: true,
            selective: false,
        }
    }
}

/// ReplyMarkup is additional data sent with a [Message] to enhance the bot
/// user experience.
///
/// You may add one of the following:
/// * [InlineKeyboardMarkup]
/// * <s>ReplyKeyboardMarkup</s> // TODO
/// * <s>ReplyKeyboardRemove</s> // TODO
/// * [ForceReply]
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ForceReply(ForceReply),
}

/// Mode that Telegram uses to parse content from a [SendMessage].
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum ParseMode {
    Html,
    Markdown,
    MarkdownV2,
}

/// ChatAction is the action that the bot is indicating.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

/// Represents a photo to be sent.
#[derive(Debug, Serialize, Clone)]
pub struct InputMediaPhoto {
    /// The type of the result, must be `photo`. You may use the Default value
    /// to ensure it is set correctly.
    #[serde(rename = "type")]
    pub media_type: String,
    /// File to send. Telegram recommends using a file ID.
    pub media: FileType,
    /// Caption for the photo, may be 0-1024 characters after entity parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// If the photo should be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

impl Default for InputMediaPhoto {
    fn default() -> Self {
        Self {
            media_type: "photo".into(),
            media: Default::default(),
            caption: None,
            parse_mode: None,
            has_spoiler: None,
        }
    }
}

/// Represents a video to be sent.
#[derive(Debug, Serialize, Clone)]
pub struct InputMediaVideo {
    /// The type of the result, must be `photo`. You may use the Default value
    /// to ensure it is set correctly.
    #[serde(rename = "type")]
    pub media_type: String,
    /// File to send. Telegram recommends using a file ID.
    pub media: FileType,
    /// Optional thumbnail for the video. Should be in JPEG format and less than
    /// 200kB in size. It should not be larger than 320x320.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<FileType>,
    /// Caption for the video, may be 0-1024 characters after entity parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Optional video width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Optional video height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// Optional video duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// If the video is suitable for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// If the video should be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

impl Default for InputMediaVideo {
    fn default() -> Self {
        Self {
            media_type: "video".into(),
            media: Default::default(),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum InputMedia {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl InputMedia {
    /// Replaces the media within an InputMedia without caring about the type.
    pub fn update_media(&self, media: FileType) -> Self {
        match self {
            InputMedia::Photo(photo) => InputMedia::Photo(InputMediaPhoto {
                media,
                ..photo.clone()
            }),
            InputMedia::Video(video) => InputMedia::Video(InputMediaVideo {
                media,
                ..video.clone()
            }),
        }
    }

    /// Get the file out of an InputMedia value.
    pub fn get_file(&self) -> &FileType {
        match self {
            InputMedia::Photo(photo) => &photo.media,
            InputMedia::Video(video) => &video.media,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct InlineQueryResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(flatten)]
    pub content: InlineQueryType,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InlineQueryType {
    Article(InlineQueryResultArticle),
    Photo(InlineQueryResultPhoto),
    Gif(InlineQueryResultGIF),
    Video(InlineQueryResultVideo),
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InlineQueryResultArticle {
    pub title: String,
    #[serde(flatten)]
    pub input_message_content: InputMessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InlineQueryResultPhoto {
    pub photo_url: String,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InlineQueryResultGIF {
    pub gif_url: String,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InlineQueryResultVideo {
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

impl InlineQueryResult {
    pub fn article(id: String, title: String, text: String) -> InlineQueryResult {
        InlineQueryResult {
            result_type: "article".into(),
            id,
            reply_markup: None,
            content: InlineQueryType::Article(InlineQueryResultArticle {
                title,
                description: None,
                input_message_content: InputMessageType::Text(InputMessageText {
                    message_text: text,
                    parse_mode: None,
                }),
            }),
        }
    }

    pub fn photo(id: String, photo_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResult {
            result_type: "photo".into(),
            id,
            reply_markup: None,
            content: InlineQueryType::Photo(InlineQueryResultPhoto {
                photo_url,
                thumb_url,
                caption: None,
                ..Default::default()
            }),
        }
    }

    pub fn gif(id: String, gif_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResult {
            result_type: "gif".into(),
            id,
            reply_markup: None,
            content: InlineQueryType::Gif(InlineQueryResultGIF {
                gif_url,
                thumb_url,
                caption: None,
            }),
        }
    }

    pub fn video(
        id: String,
        video_url: String,
        mime_type: String,
        thumb_url: String,
        title: String,
    ) -> InlineQueryResult {
        InlineQueryResult {
            result_type: "video".into(),
            id,
            reply_markup: None,
            content: InlineQueryType::Video(InlineQueryResultVideo {
                video_url,
                mime_type,
                thumb_url,
                title,
                caption: None,
            }),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputMessageType {
    Text(InputMessageText),
}

impl Default for InputMessageType {
    fn default() -> Self {
        InputMessageType::Text(Default::default())
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct InputMessageText {
    pub message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}

/// GetMe is a request that returns [User] information for the current bot.
#[derive(Serialize, Debug, Clone)]
pub struct GetMe;

impl TelegramRequest for GetMe {
    type Response = User;

    fn endpoint(&self) -> &str {
        "getMe"
    }
}

/// GetUpdates is a request that returns any available [Updates](Update).
#[derive(Serialize, Default, Debug, Clone)]
pub struct GetUpdates {
    /// ID for the first update to return. This must be set to one higher
    /// than previous IDs in order to confirm previous updates and clear them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of [Updates](Update) to retrieve. May be set 1-100,
    /// defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of seconds for long polling. This should be set to a reasonable
    /// value in production to avoid unneeded requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Which update types to receive. May be set to any available types.
    /// * `message`
    /// * `edited_message`
    /// * `channel_post`
    /// * `edited_channel_post`
    /// * `inline_query`
    /// * `chosen_inline_result`
    /// * `callback_query`
    /// * `shipping_query`
    /// * `pre_checkout_query`
    /// * `poll`
    /// * `poll_answer`
    /// * `my_chat_member`
    /// * `chat_member`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl TelegramRequest for GetUpdates {
    type Response = Vec<Update>;

    fn endpoint(&self) -> &str {
        "getUpdates"
    }
}

/// SendMessage sends a message.
///
/// # Example
///
/// ```
/// # use tgbotapi::requests::{ChatID, SendMessage};
/// let send_message = SendMessage {
///     chat_id: ChatID::Identifier(12345),
///     text: "Hello, world!".into(),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Default, Debug, Clone)]
pub struct SendMessage {
    /// The ID of the chat to send a message to.
    pub chat_id: ChatID,
    /// The text of the message. May be 1-4096 characters after entity parsing.
    pub text: String,
    /// The mode used to parse the provided text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// If Telegram should not generate a web page preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// If the message should be sent silently (notification but no sound).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// The ID of the [Message] this Message is in reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Allow sending the message even if the reply message was not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// The [ReplyMarkup], if desired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendMessage {
    type Response = Message;

    fn endpoint(&self) -> &str {
        "sendMessage"
    }
}

/// SendChatAction allows you to indicate to users that the bot is performing
/// an action.
///
/// Actions last for 5 seconds or until a message is sent,
/// whichever comes first.
#[derive(Serialize, Debug, Clone)]
pub struct SendChatAction {
    /// The ID of the chat to send an action to.
    pub chat_id: ChatID,
    /// The action to indicate.
    pub action: ChatAction,
}

impl TelegramRequest for SendChatAction {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "sendChatAction"
    }
}

/// SendPhoto sends a photo.
#[derive(Serialize, Debug, Default, Clone)]
pub struct SendPhoto {
    /// The ID of the chat to send a photo to.
    pub chat_id: ChatID,
    /// The file that makes up this photo.
    #[serde(skip_serializing_if = "FileType::needs_upload")]
    pub photo: FileType,
    /// A caption for the photo, if desired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// If the photo should be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Allow sending the message even if the reply message was not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendPhoto {
    type Response = Message;

    fn endpoint(&self) -> &str {
        "sendPhoto"
    }

    fn files(&self) -> RequestFiles {
        // Check if the photo needs to be uploaded. If the photo does need to
        // be uploaded, we specify the field name and get the file. This unwrap
        // is safe because `needs_upload` only returns true when it exists.
        if self.photo.needs_upload() {
            Some(vec![("photo".into(), self.photo.file().unwrap())])
        } else {
            None
        }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SendDocument {
    /// The ID of the chat to send a photo to.
    pub chat_id: ChatID,
    /// The file that makes up this photo.
    #[serde(skip_serializing_if = "FileType::needs_upload")]
    pub document: FileType,
    /// A caption for the photo, if desired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendDocument {
    type Response = Message;

    fn endpoint(&self) -> &str {
        "sendDocument"
    }

    fn files(&self) -> RequestFiles {
        if self.document.needs_upload() {
            Some(vec![("document".into(), self.document.file().unwrap())])
        } else {
            None
        }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SendVideo {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "FileType::needs_upload")]
    pub video: FileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendVideo {
    type Response = Message;

    fn endpoint(&self) -> &str {
        "sendVideo"
    }

    fn files(&self) -> RequestFiles {
        if self.video.needs_upload() {
            Some(vec![("video".into(), self.video.file().unwrap())])
        } else {
            None
        }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SendAnimation {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "FileType::needs_upload")]
    pub animation: FileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendAnimation {
    type Response = Message;

    fn endpoint(&self) -> &str {
        "sendAnimation"
    }

    fn files(&self) -> RequestFiles {
        if self.animation.needs_upload() {
            Some(vec![("animation".into(), self.animation.file().unwrap())])
        } else {
            None
        }
    }
}

/// GetFile retrieves information about a file.
///
/// This will not download the file! It only returns a [File] containing
/// the path which is needed to download the file. This returned ID lasts at
/// least one hour.
#[derive(Serialize, Debug, Clone)]
pub struct GetFile {
    /// The ID of the file to fetch.
    pub file_id: String,
}

impl TelegramRequest for GetFile {
    type Response = File;

    fn endpoint(&self) -> &str {
        "getFile"
    }
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SendMediaGroup {
    pub chat_id: ChatID,
    #[serde(serialize_with = "clean_input_media")]
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
}

impl TelegramRequest for SendMediaGroup {
    type Response = Vec<Message>;

    fn endpoint(&self) -> &str {
        "sendMediaGroup"
    }

    fn files(&self) -> RequestFiles {
        if !self.media.iter().any(|item| item.get_file().needs_upload()) {
            return None;
        }

        let mut items = Vec::new();

        for item in &self.media {
            let file = item.get_file();

            let part = match file {
                FileType::Bytes(file_name, bytes) => {
                    let file = reqwest::multipart::Part::bytes(bytes.clone())
                        .file_name(file_name.to_string());

                    (file_name.to_string(), file)
                }
                _ => continue,
            };

            items.push(part);
        }

        Some(items)
    }
}

/// Responds to an inline query request.
#[derive(Debug, Serialize, Default, Clone)]
pub struct AnswerInlineQuery {
    /// ID of the inline query.
    pub inline_query_id: String,
    /// Results displayed to the user. May include up to 50 items.
    pub results: Vec<InlineQueryResult>,
    /// Maximum amount of time the server may cache these results for.
    /// Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// If the results are specific to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Offset for getting more results, may be up to 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// Provide a button with the specified text to start sending a message to
    /// the bot, used with `switch_pm_parameter`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// Deep-linking parameter when switching to PMs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

impl TelegramRequest for AnswerInlineQuery {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "answerInlineQuery"
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct SetWebhook {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl TelegramRequest for SetWebhook {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "setWebhook"
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DeleteWebhook;

impl TelegramRequest for DeleteWebhook {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "deleteWebhook"
    }
}

/// Answers a callback query sent from an inline keyboard.
///
/// The answer is displayed to the user as a notification.
#[derive(Clone, Default, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    /// ID of the query to answer.
    pub callback_query_id: String,
    /// The text of the notification, may be 0-200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If an alert should be shown to the user. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL opened by user's client.
    ///
    /// Must be a game URL if from a callback_game button, otherwise it may be
    /// a link to open your bot with a parameter (`t.me/your_bot?start=XXXX`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Maximum time to cache the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

impl TelegramRequest for AnswerCallbackQuery {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "answerCallbackQuery"
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Box<Message>),
    Bool(bool),
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct EditMessageText {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for EditMessageText {
    type Response = MessageOrBool;

    fn endpoint(&self) -> &str {
        "editMessageText"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct EditMessageCaption {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for EditMessageCaption {
    type Response = MessageOrBool;

    fn endpoint(&self) -> &str {
        "editMessageCaption"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct EditMessageReplyMarkup {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for EditMessageReplyMarkup {
    type Response = MessageOrBool;

    fn endpoint(&self) -> &str {
        "editMessageReplyMarkup"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct DeleteMessage {
    pub chat_id: ChatID,
    pub message_id: i32,
}

impl TelegramRequest for DeleteMessage {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "deleteMessage"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct GetChat {
    pub chat_id: ChatID,
}

impl TelegramRequest for GetChat {
    type Response = Chat;

    fn endpoint(&self) -> &str {
        "getChat"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct GetChatAdministrators {
    pub chat_id: ChatID,
}

impl TelegramRequest for GetChatAdministrators {
    type Response = Vec<ChatMember>;

    fn endpoint(&self) -> &str {
        "getChatAdministrators"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct GetChatMember {
    pub chat_id: ChatID,
    pub user_id: i64,
}

impl TelegramRequest for GetChatMember {
    type Response = ChatMember;

    fn endpoint(&self) -> &str {
        "getChatMember"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct SetMyDefaultAdministratorRights {
    #[serde(flatten)]
    pub rights: ChatAdministratorRights,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl TelegramRequest for SetMyDefaultAdministratorRights {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "setMyDefaultAdministratorRights"
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct GetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl TelegramRequest for GetMyDefaultAdministratorRights {
    type Response = bool;

    fn endpoint(&self) -> &str {
        "getMyDefaultAdministratorRights"
    }
}
