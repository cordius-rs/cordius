use cordius::api::{Endpoint, MessageFlags, ActivityTypes, GatewayIntents};
use std::time::{Instant};

fn main() {
  let st = Instant::now();
  println!("{}{}", Endpoint::BASE_URL, Endpoint::CURRENT_APPLICATION);
  println!("{}", MessageFlags::Ephemeral);
  println!("{}", MessageFlags::IS_COMPONENTS_V2 | MessageFlags::SUPPRESS_EMBEDS);
  println!("{}", ActivityTypes::Game); // or "GAME"
  println!("{}", GatewayIntents::Guilds);
  println!("{}", GatewayIntents::Guilds | GatewayIntents::GuildMembers);
  println!("{}", GatewayIntents::DIRECT_MESSAGE_POLLS);
  println!("{:.2?}", st.elapsed());
}
