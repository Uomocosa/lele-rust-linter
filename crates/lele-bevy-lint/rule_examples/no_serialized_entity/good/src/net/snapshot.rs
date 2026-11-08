use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct Snapshot {
    pub owner: PeerId,
}

#[derive(Deserialize)]
pub struct RoomRoster {
    pub members: Vec<PeerId>,
}

#[derive(Serialize, Deserialize)]
pub enum Delta {
    Joined { owner: Option<PeerId> },
    Left,
}
