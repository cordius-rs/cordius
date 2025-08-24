use serde::{Serialize, Deserialize};
use crate::types::Nameplate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collectibles {
    #[serde(default)]
    nameplate: Option<Nameplate>,
}
