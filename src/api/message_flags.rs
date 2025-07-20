use std::fmt;
use bitflags::bitflags;

bitflags! {
  #[derive(Debug)]
  pub struct MessageFlags: u32 {
    const Crossposted = 1 << 0;
    const CROSSPOSTED = 1 << 0;
    const IsCrosspost = 1 << 1;
    const ISCROSSPOST = 1 << 1;
    const Is_Crosspost = 1 << 1;
    const IS_CROSSPOST = 1 << 1;
    const SuppressEmbeds = 1 << 2;
    const SUPPRESSEMBEDS = 1 << 2;
    const Suppress_Embeds = 1 << 2;
    const SUPPRESS_EMBEDS = 1 << 2;
    const SourceMessageDeleted = 1 << 3;
    const SOURCEMESSAGEDELETED = 1 << 3;
    const Source_Message_Deleted = 1 << 3;
    const SOURCE_MESSAGE_DELETED = 1 << 3;
    const HasThread = 1 << 5;
    const HASTHREAD = 1 << 5;
    const Has_Thread = 1 << 5;
    const HAS_THREAD = 1 << 5;
    const Ephemeral = 1 << 6;
    const EPHEMERAL = 1 << 6;
    const Loading = 1 << 7;
    const LOADING = 1 << 7;
    const FailedToMentionSomeRolesInThread = 1 << 8;
    const FAILEDTOMENTIONSOMEROLESINTHREAD = 1 << 8;
    const Failed_To_Mention_Some_Roles_In_Thread = 1 << 8;
    const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
    const SuppressNotifications = 1 << 12;
    const SUPPRESSNOTIFICATIONS = 1 << 12;
    const Suppress_Notifications = 1 << 12;
    const SUPPRESS_NOTIFICATIONS = 1 << 12;
    const IsVoiceMessage = 1 << 13;
    const ISVOICEMESSAGE = 1 << 13;
    const Is_Voice_Message = 1 << 13;
    const IS_VOICE_MESSAGE = 1 << 13;
    const HasSnapshot = 1 << 14;
    const HASSNAPSHOT = 1 << 14;
    const Has_Snapshot = 1 << 14;
    const HAS_SNAPSHOT = 1 << 14;
    const IsComponentsV2 = 1 << 15;
    const ISCOMPONENTSV2 = 1 << 15;
    const Is_Components_V2 = 1 << 15;
    const IS_COMPONENTS_V2 = 1 << 15;
  }
}

impl fmt::Display for MessageFlags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.bits())
  }
}
