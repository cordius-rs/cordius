use std::fmt;
use bitflags::bitflags;

bitflags! {
  pub struct GatewayIntents: u32 {
    const Guilds = 1 << 0;
    const GUILDS = 1 << 0;
    const GuildMembers = 1 << 1;  
    const GUILDMEMBERS = 1 << 1;  
    const Guild_Members = 1 << 1;  
    const GUILD_MEMBERS = 1 << 1;  
    const GuildModeration = 1 << 2;  
    const GUILDMODERATION = 1 << 2;  
    const Guild_Moderation = 1 << 2;  
    const GUILD_MODERATION = 1 << 2;  
    const GuildExpressions = 1 << 3;  
    const GUILDEXPRESSIONS = 1 << 3;  
    const Guild_Expressions = 1 << 3;  
    const GUILD_EXPRESSIONS = 1 << 3;  
    const GuildIntegrations = 1 << 4;  
    const GUILDINTEGRATIONS = 1 << 4;  
    const Guild_Integrations = 1 << 4;  
    const GUILD_INTEGRATIONS = 1 << 4;  
    const GuildWebhooks = 1 << 5;  
    const GUILDWEBHOOKS = 1 << 5;  
    const Guild_Webhooks = 1 << 5;  
    const GUILD_WEBHOOKS = 1 << 5;  
    const GuildInvites = 1 << 6;
    const GUILDINVITES = 1 << 6;
    const Guild_Invites = 1 << 6;
    const GUILD_INVITES = 1 << 6;
    const GuildVoiceStates = 1 << 7;
    const GUILDVOICESTATES = 1 << 7;
    const Guild_Voice_States = 1 << 7;
    const GUILD_VOICE_STATES = 1 << 7;
    const GuildPresences = 1 << 8;
    const GUILDPRESENCES = 1 << 8;
    const Guild_Presences = 1 << 8;
    const GUILD_PRESENCES = 1 << 8;
    const GuildMessages = 1 << 9;
    const GUILDMESSAGES = 1 << 9;
    const Guild_Messages = 1 << 9;
    const GUILD_MESSAGES = 1 << 9;
    const GuildMessageReactions = 1 << 10;
    const GUILDMESSAGEREACTIONS = 1 << 10;
    const Guild_Message_Reactions = 1 << 10;
    const GUILD_MESSAGE_REACTIONS = 1 << 10;
    const GuildMessageTyping = 1 << 11;
    const GUILDMESSAGETYPING = 1 << 11;
    const Guild_Message_Typing = 1 << 11;
    const GUILD_MESSAGE_TYPING = 1 << 11;
    const DirectMessages = 1 << 12;
    const DIRECTMESSAGES = 1 << 12;
    const Direct_Messages = 1 << 12;
    const DIRECT_MESSAGES = 1 << 12;
    const DirectMessageReactions = 1 << 13;
    const DIRECTMESSAGEREACTIONS = 1 << 13;
    const Direct_Message_Reactions = 1 << 13;
    const DIRECT_MESSAGE_REACTIONS = 1 << 13;
    const DirectMessageTyping = 1 << 14;
    const DIRECTMESSAGETYPING = 1 << 14;
    const Direct_Message_Typing = 1 << 14;
    const DIRECT_MESSAGE_TYPING = 1 << 14;
    const MessageContent = 1 << 15;
    const MESSAGECONTENT = 1 << 15;
    const Message_Content = 1 << 15;
    const MESSAGE_CONTENT = 1 << 15;
    const GuildScheduledEvents = 1 << 16;
    const GUILDSCHEDULEDEVENTS = 1 << 16;
    const Guild_Scheduled_Events = 1 << 16;
    const GUILD_SCHEDULED_EVENTS = 1 << 16;
    const AutoModerationConfiguration = 1 << 20;
    const AUTOMODERATIONCONFIGURATION = 1 << 20;
    const Auto_Moderation_Configuration = 1 << 20;
    const AUTO_MODERATION_CONFIGURATION = 1 << 20;
    const AutoModerationExecution = 1 << 21;
    const AUTOMODERATIONEXECUTION = 1 << 21;
    const Auto_Moderation_Execution = 1 << 21;
    const AUTO_MODERATION_EXECUTION = 1 << 21;
    const GuildMessagePolls = 1 << 24;
    const GUILDMESSAGEPOLLS = 1 << 24;
    const Guild_Message_Polls = 1 << 24;
    const GUILD_MESSAGE_POLLS = 1 << 24;
    const DirectMessagePolls = 1 << 25;
    const DIRECTMESSAGEPOLLS = 1 << 25;
    const Direct_Message_Polls = 1 << 25;
    const DIRECT_MESSAGE_POLLS = 1 << 25;
  }
}

impl fmt::Display for GatewayIntents {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.bits())
  }
}
