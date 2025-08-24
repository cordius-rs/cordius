use crate::types::Snowflake;

pub struct UserPrimaryGuild {
    #[serde(default)]
    identity_guild_id: Option<Snowflake>,
    #[serde(default)]
    identity_enabled: bool,
    #[serde(default)]
    tag: Option<String>,
    #[serde(default)]
    badge: Option<String>,
}
