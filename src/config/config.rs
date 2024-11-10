use serde::{Deserialize, Serialize};
use crate::features;

#[derive(Serialize, Deserialize, Default)]
// Example config
pub struct Config {
    features: FEATURES
}