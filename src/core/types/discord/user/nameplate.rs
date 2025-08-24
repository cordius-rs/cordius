use serde::{Serialize, Deserialize};
use crate::types::Snowflake;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nameplate {
    sku_id: Snowflake,
    asset: String,
    label: String
    palette: String,
}
