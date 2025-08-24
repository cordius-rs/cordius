use serde::{Serialize, Deserialize};
use crate::types{Snowflake, AvatarDecorationData, Collectibles, UserPrimaryGuild};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    #[serde(default)]
    global_name: Option<String>,
    #[serde(default)]
    avatar: Option<String>,
    #[serde(default)]
    bot: bool,
    #[serde(default)]
    system: bool,
    #[serde(default)]
    mfa_enabled: bool,
    #[serde(default)]
    banner: Option<String>,
    #[serde(default)]
    accent_color: Option<u32>,
    #[serde(default)]
    locale: Option<String>,
    #[serde(default)]
    verified: bool,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    flags: u64,
    #[serde(default)]
    premium_type: Option<u8>,
    #[serde(default)]
    public_flags: u64,
    #[serde(default)]
    avatar_decoration_data: Option<AvatarDecorationData>,
    #[serde(default)]
    collectibles: Option<Collectibles>,
    #[serde(default)]
    primary_guild: Option<UserPrimaryGuild>,
}
