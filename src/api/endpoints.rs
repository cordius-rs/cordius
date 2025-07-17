use std::fmt;

// Current DAPI version this project is using: 10

#[allow(non_camel_case_types)]
pub enum Endpoint {
  // Not endpoints but
  BASE_URL,
  WSS,
  // Application
  // See more https://discord.com/developers/docs/resources/application
  CURRENT_APPLICATION,
  APPLICATION_ACTIVITY_INSTANCE(u64, String),
  // Audit Logs
  // See more https://discord.com/developers/docs/resources/audit-log
  GUILD_AUDIT_LOG(u64),
  // Auto Moderation
  // See more https://discord.com/developers/docs/resources/application
  AUTO_MOD_RULES(u64),
  AUTO_MOD_RULE(u64, u64),
  // Channel
  // See more https://discord.com/developers/docs/resources/channel
  CHANNEL(u64),
  CHANNEL_PERMS(u64, u64),
  CHANNEL_INV(u64),
  FOLLOW_ANNOUNCEMENT_CHANNEL(u64),
  TRIGGER_TYPING_INDICATOR(u64),
  GROUP_RECIPIENT(u64, u64),
  START_THREAD_FROM_MSG(u64, u64),
  START_THREAD(u64),
  THREAD(u64),
  THREAD_MEMBER(u64, u64),
  LIST_THREAD_MEMBERS(u64),
  LIST_ARCHIVED_THREADS(u64),
  LIST_PRIVATE_ARCHIVED_THREADS(u64),
  LIST_JOINED_PRIVATE_ARCHIVED_THREADS(u64),
  // Emoji
  // See more https://discord.com/developers/docs/resources/emoji
  GUILD_EMOJIS(u64),
  GUILD_EMOJIS_MANAGE(u64, u64),
  APPLICATION_EMOJIS(u64),
  APPLICATION_EMOJIS_MANAGE(u64, u64),
  // Entitlements
  // See more https://discord.com/developers/docs/resources/entitlement
}

impl fmt::Display for Endpoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let end = match self {
      Endpoint::BASE_URL => String::from("https://discord.com/api/v10"),
      Endpoint::WSS => String::from("wss://gateway.discord.gg/?v=10&encoding=json"),
      Endpoint::CURRENT_APPLICATION => String::from("/applications/@me"),
      Endpoint::APPLICATION_ACTIVITY_INSTANCE(application_id, instance_id) => format!("/applications/{application_id}/activity-instances/{instance_id}"),
      Endpoint::GUILD_AUDIT_LOG(guild_id) => format!("/guilds/{guild_id}/audit-logs"),
      Endpoint::AUTO_MOD_RULES(guild_id) => format!("/guilds/{guild_id}/auto-moderation/rules"),
      Endpoint::AUTO_MOD_RULE(guild_id, auto_mod_rule_id) => format!("/guilds/{guild_id}/auto-moderation/rules/{auto_mod_rule_id}"),
      Endpoint::CHANNEL(channel_id) => format!("/channels/{channel_id}"),
      Endpoint::CHANNEL_PERMS(channel_id, overwrite_id) => format!("/channels/{channel_id}/permissions/{overwrite_id}"),
      Endpoint::CHANNEL_INV(channel_id) => format!("/channels/{channel_id}/invites"),
      Endpoint::FOLLOW_ANNOUNCEMENT_CHANNEL(channel_id) => format!("/channels/{channel_id}/followers"),
      Endpoint::TRIGGER_TYPING_INDICATOR(channel_id) => format!("/channels/{channel_id}/typing"),
      Endpoint::GROUP_RECIPIENT(channel_id, user_id) => format!("/channels/{channel_id}/recipients/{user_id}"),
      Endpoint::START_THREAD_FROM_MSG(channel_id, message_id) => format!("/channels/{channel_id}/messages/{message_id}/threads"),
      Endpoint::START_THREAD(channel_id) => format!("/channels/{channel_id}/threads"),
      Endpoint::THREAD(channel_id) => format!("/channels/{channel_id}/thread-members/@me"),
      Endpoint::THREAD_MEMBER(channel_id, user_id) => format!("/channels/{channel_id}/thread-members/{user_id}"),
      Endpoint::LIST_THREAD_MEMBERS(channel_id) => format!("/channels/{channel_id}/thread-members"),
      Endpoint::LIST_ARCHIVED_THREADS(channel_id) => format!("/channels/{channel_id}/threads/archived/public"),
      Endpoint::LIST_PRIVATE_ARCHIVED_THREADS(channel_id) => format!("/channels/{channel_id}/threads/archived/private"),
      Endpoint::LIST_JOINED_PRIVATE_ARCHIVED_THREADS(channel_id) => format!("/channels/{channel_id}/users/@me/threads/archived/private"),
      Endpoint::GUILD_EMOJIS(guild_id) => format!("/guilds/{guild_id}/emojis"),
      Endpoint::GUILD_EMOJIS_MANAGE(guild_id, emoji_id) => format!("/guilds/{guild_id}/emojis/{emoji_id}"),
      Endpoint::APPLICATION_EMOJIS(app_id) => format!("/applications/{app_id}/emojis"),
      Endpoint::APPLICATION_EMOJIS_MANAGE(app_id, emoji_id) => format!("/applications/{app_id}/emojis/{emoji_id}"),
    };
    write!(f, "{end}")
  }
}
