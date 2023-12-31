use cgmath::{Vector3, Point3};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WorldGenSettings {
    Normal,
    PracticeRange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub name: String,
    pub delta_time: f32,
    pub is_remote: bool,
    pub player_count: u32,
    pub render_size: Vector3<u32>,
    pub spawn_location: Point3<f32>,
    pub max_loaded_chunks: u32,
    pub world_gen: WorldGenSettings,
    pub fixed_center: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lobby {
    pub name: String,
    pub lobby_id: u64,
    pub settings: GameSettings,
}