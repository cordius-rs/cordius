use serde::{Serialize, Deserialize};
use crate::types::Snowflake;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarDecorationData {
    asset: String,
    sku_id: Snowflake,
}
