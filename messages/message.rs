use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::types::{Snowflake, User, Role, Channel, Attachment, Embed, Reaction, Nonce, MessageActivity, Application, MessageReference, MessageSnapshot, MessageInteractionMetadata, MessageInteraction, MessageComponents, MessageStickerItem, Sticker, RoleSubscriptionData, Resolved, Poll, MessageCall};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    author: User,
    #[serde(default)]
    content: String,
    timestamp: DateTime<Utc>,
    edited_timestamp: Option<DateTime<Utc>>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    mention_channels: Option<Vec<Channel>>,
    attachments: Vec<Attachment>,
    #[serde(default)]
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<Nonce>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    #[serde(rename="type")]
    type_: u8,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<Snowflake>,
    flags: Option<u64>,
    message_reference: Option<MessageReference>,
    message_snapshots: Option<Vec<MessageSnapshot>>,
    #[serde(default)]
    referenced_message: Option<Box<Message>>,
    interaction_metadata: Option<MessageInteractionMetadata>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    #[serde(default)]
    components: Option<Vec<MessageComponents>>,
    sticker_items: Option<Vec<MessageStickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<u64>,
    role_subscription_data: Option<RoleSubscriptionData>,
    resolved: Option<Resolved>,
    #[serde(default)]
    poll: Option<Poll>,
    call: Option<MessageCall>,
}
