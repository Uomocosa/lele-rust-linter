use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct Snapshot {
    pub owner: Entity,
}

#[derive(Deserialize)]
pub struct RoomRoster {
    pub members: Vec<Entity>,
}

#[derive(Serialize, Deserialize)]
pub enum Delta {
    Joined { owner: Option<Entity> },
    Left,
}
