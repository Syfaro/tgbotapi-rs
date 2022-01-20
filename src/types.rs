use serde::{Deserialize, Serialize};

use crate::error::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    /// If the request was successful. If true, the result is available.
    /// If false, error contains information about what happened.
    pub ok: bool,
    #[serde(flatten)]
    pub error: TelegramError,

    /// The response data.
    pub result: Option<T>,
}

impl<T> From<Response<T>> for Result<T, TelegramError> {
    fn from(resp: Response<T>) -> Self {
        match resp.result {
            Some(result) if resp.ok => Ok(result),
            _ => Err(resp.error),
        }
    }
}

impl<T> From<Response<T>> for Result<T, Error> {
    fn from(resp: Response<T>) -> Self {
        match resp.result {
            Some(result) if resp.ok => Ok(result),
            _ => Err(Error::Telegram(resp.error)),
        }
    }
}

/// An update from Telegram. Each update contains exactly one item.
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::Private
    }
}

impl ChatType {
    pub fn is_group(&self) -> bool {
        *self == Self::Group || *self == Self::Supergroup
    }
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i32>,
    pub message_auto_delete_time: Option<i32>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
}

/// An entity within a message's text or caption.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub entity_type: MessageEntityType,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
}

/// The type of an entity within a message's text or caption.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    #[serde(rename = "url")]
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
}

/// A sent message.
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Message {
    /// Unique identifier for this message in this chat.
    pub message_id: i32,
    /// Sender of message. Will be empty in channels.
    pub from: Option<User>,
    /// Date the message was sent, as a unix timestasmp.
    pub date: i64,
    /// Chat the message was sent to.
    pub chat: Chat,
    /// Original sender of a message, if it was forwarded.
    pub forward_from: Option<User>,
    /// Original chat of a message, if it was forwarded from a channel.
    pub forward_from_chat: Option<Chat>,
    /// Original message ID, if it was forwarded from a channel.
    pub forward_from_message_id: Option<i32>,
    /// Signature for a message forwarded from a channel with signatures enabled.
    pub forward_signature: Option<String>,
    /// Original sender's name when user has disallowed linking to their account.
    pub forward_sender_name: Option<String>,
    /// Original message's sent date, as a unix timestamp.
    pub forward_date: Option<i64>,
    /// The message this message is in reply to. Can only be one level deep.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent.
    pub via_bot: Option<User>,
    /// Date this message was last edited, as a unix timestamp.
    pub edit_date: Option<i64>,
    /// Unique identifier of the media group the message belongs to.
    pub media_group_id: Option<String>,
    /// Author's signature, if posted in a channel with signatures enabled.
    pub author_signature: Option<String>,
    /// Text of the message, may be 0-4096 characters.
    pub text: Option<String>,
    /// Entities contained within the text.
    pub entities: Option<Vec<MessageEntity>>,
    /// Entities contained within the caption.
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// If the message was audio, the audio information.
    pub audio: Option<Audio>,
    /// If the message was a document, the document information.
    pub document: Option<Document>,
    /// If the message was an animation, the animation information.
    ///
    /// For backwards compatibility, document is also set.
    pub animation: Option<Animation>,
    /// If the message was a game, the game information.
    pub game: Option<Game>,
    /// If the message was a photo, a vec of sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,
    /// If the message was a sticker, the sticker information.
    pub sticker: Option<Sticker>,
    /// If the message was a video, the video information.
    pub video: Option<Video>,
    /// If the message was a voice recording, the voice information.
    pub voice: Option<Voice>,
    /// If the message was a video note, the video note information.
    pub video_note: Option<VideoNote>,
    /// The caption, may be set if this message contained an animation, audio,
    /// document, photo, video, or voice. Up to 1024 characters.
    pub caption: Option<String>,
    /// If the message was a contact, the contact information.
    pub contact: Option<Contact>,
    /// If the message was a location, the location information.
    pub location: Option<Location>,
    /// If the message was a venue, the venue information.
    pub venue: Option<Venue>,
    /// If the message was a poll, the poll information.
    pub poll: Option<Poll>,
    /// If new members were added, information about those users.
    ///
    /// This includes the bot when it is added to a group.
    pub new_chat_members: Option<Vec<User>>,
    /// If a member left or was removed, the user information.
    ///
    /// This includes the bot.
    pub left_chat_member: Option<User>,
    /// If the chat title was changed, the new title.
    pub new_chat_title: Option<String>,
    /// If the chat photo changed, a vec of sizes of the new photo.
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// If the chat photo was deleted, true.
    pub delete_chat_photo: Option<bool>,
    /// If the group was newly created, true.
    pub group_chat_created: Option<bool>,
    /// If the supergroup was newly created, true.
    ///
    /// Bots will never get this directly, only through replies.
    pub supergroup_chat_created: Option<bool>,
    /// If the group has been migrated to a new supergroup, the new ID.
    pub migrate_to_chat_id: Option<i64>,
    /// If the group was migrated to a new supergroup, the new ID.
    pub migrate_from_chat_id: Option<i64>,
    /// If a message was pinned, the pinned message.
    pub pinned_message: Option<Box<Message>>,
    // TODO: this is missing invoice, successful_payment
    /// If the user logged in, the domain name of the website.
    pub connected_website: Option<String>,
    // TODO: this is missing passport_data
    /// If the message had an inline keyboard, that inline keyboard data.
    ///
    /// `login_url` buttons will be represented as regular `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Clone, Debug, Serialize)]
/// Command represents information obtained from the BotCommand MessageEntity.
pub struct Command {
    /// The name of the command, eg. `/start`.
    ///
    /// If the command was specified in the `/command@username` format, the
    /// username will be removed.
    pub name: String,
    /// The entity responsible for this command.
    pub entity: MessageEntity,
    /// The username of the bot, if the command was provided in the
    /// `/command@username` format.
    pub username: Option<String>,
}

impl Message {
    /// Extracts the command from a given message.
    ///
    /// Returns None if there is not a MessageEntity of type BotCommand
    /// starting at offset 0.
    pub fn get_command(&self) -> Option<Command> {
        let entities = self.entities.as_ref()?;
        let text = self.text.as_ref()?;
        let entity = entities.iter().find(|entity| {
            entity.offset == 0 && entity.entity_type == MessageEntityType::BotCommand
        })?;

        let command_text: String = text
            .chars()
            .skip(entity.offset as usize)
            .take(entity.length as usize)
            .collect();

        let mut command_parts = command_text.split('@');

        let command = command_parts.next().unwrap().to_string();
        let username = command_parts.next().map(|part| part.to_string());

        Some(Command {
            name: command,
            entity: entity.clone(),
            username,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

/// A callback query is data from an inline keyboard.
///
/// Exactly one of `data` or `game_short_name` will be set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallbackQuery {
    /// Unique ID of this callback query.
    pub id: String,
    /// Sender of the callback query.
    pub from: User,
    /// Message that originated the query. May be omitted if too old.
    pub message: Option<Box<Message>>,
    /// Identifier of the message that sent via the bot in inline mode.
    pub inline_message_id: Option<String>,
    /// Global identifier for the chat, useful for game high scores.
    pub chat_instance: Option<String>,
    /// Data associated with the callback button.
    pub data: Option<String>,
    /// Short name of the game to be returned.
    pub game_short_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct File {
    /// The ID for this file, specific to this bot.
    pub file_id: String,
    /// The size of the file, if known.
    pub file_size: Option<usize>,
    /// A path which is required to download the file. It is unclear
    /// when this would ever be `None`.
    pub file_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LoginUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub expire_date: Option<i32>,
    pub member_limit: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Restricted,
    Left,
    Kicked,
}

impl Default for ChatMemberStatus {
    fn default() -> Self {
        Self::Member
    }
}

impl ChatMemberStatus {
    pub fn is_admin(&self) -> bool {
        *self == Self::Creator || *self == Self::Administrator
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ChatMember {
    pub user: User,
    pub status: ChatMemberStatus,
    pub custom_title: Option<String>,
    pub until_date: Option<i32>,
    pub can_be_edited: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub is_member: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i32,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
}

/// The part of the face where the mask should be placed as a part of a mask
/// position in a sticker.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

impl Default for MaskPositionPoint {
    fn default() -> Self {
        MaskPositionPoint::Forehead
    }
}

/// The position on faces where a mask should be placed.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MaskPosition {
    /// The part of the face where the mask belongs.
    pub point: MaskPositionPoint,
    /// Shift x-axis measured by widths of masks scaled to face size.
    pub x_shift: f64,
    /// Shift y-axis measured by widths of masks scaled to face size.
    pub y_shift: f64,
    /// Mask scaling amount.
    pub scale: f64,
}

/// Information about a sticker.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Sticker {
    /// Identifier for the file, which may be used to download or reuse it.
    pub file_id: String,
    /// Unique identifier for this file which is reused between bots.
    /// May **not** be used to download or reuse the file.
    pub file_unique_id: String,
    /// Width of the sticker.
    pub width: i32,
    /// Height of the sticker.
    pub height: i32,
    /// If the sticker is animated.
    pub is_animated: bool,
    /// Thumbnail for the sticker, may be in webp or jpg format.
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// Name of the associated sticker set.
    pub set_name: Option<String>,
    /// For mask stickers, where the mask should be placed.
    pub mask_position: Option<MaskPosition>,
    /// File size of the sticker.
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub forsquare_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub poll_type: PollType,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollType {
    Regular,
    Quiz,
}

impl Default for PollType {
    fn default() -> Self {
        PollType::Regular
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i32>,
}
