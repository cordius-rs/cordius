use std::fmt;

pub enum ActivityTypes {
  Game,
  GAME,
  Streaming,
  STREAMING,
  Listening,
  LISTENING,
  Watching,
  WATCHING,
  Custom,
  CUSTOM,
  Competing,
  COMPETING,
}

impl fmt::Display for ActivityTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let act = match self {
      ActivityTypes::Game | ActivityTypes::GAME => 0,
      ActivityTypes::Streaming | ActivityTypes::STREAMING => 1,
      ActivityTypes::Listening | ActivityTypes::LISTENING => 2,
      ActivityTypes::Watching | ActivityTypes::WATCHING => 3,
      ActivityTypes::Custom | ActivityTypes::CUSTOM => 4,
      ActivityTypes::Competing | ActivityTypes::COMPETING => 5,
    };
    write!(f, "{act}")
  }
}
