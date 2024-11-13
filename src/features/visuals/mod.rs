use serde::{Deserialize, Serialize};
use crate::features::visuals::box_esp::BoxEsp;
use crate::features::visuals::name_esp::NameEsp;

pub mod box_esp;
mod name_esp;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Visuals {
    pub box_esp: BoxEsp,
    pub name_esp: NameEsp,
}