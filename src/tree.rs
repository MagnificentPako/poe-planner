use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const TREE_DATA: &[u8; 5996378] = include_bytes!("../resources/data.json");
pub const ORBIT_ANGLES_16: [i32; 16] = [
    0, 30, 45, 60, 90, 120, 135, 150, 180, 210, 225, 240, 270, 300, 315, 330,
];
pub const ORBIT_NODES: [i32; 7] = [1, 6, 16, 16, 40, 72, 72];
pub const ORBIT_RADII: [i32; 7] = [0, 82, 162, 335, 493, 662, 846];
pub const ORBIT_ANGLES_40: [i32; 40] = [
    0, 10, 20, 30, 40, 45, 50, 60, 70, 80, 90, 100, 110, 120, 130, 135, 140, 150, 160, 170, 180,
    190, 200, 210, 220, 225, 230, 240, 250, 260, 270, 280, 290, 300, 310, 315, 320, 330, 340, 350,
];

#[derive(Serialize, Deserialize, Default)]
pub struct Class {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupBackground {
    pub image: String,
    pub is_half_image: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Group {
    pub x: f32,
    pub y: f32,
    pub orbits: Vec<usize>,
    pub background: Option<GroupBackground>,
    pub nodes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SpriteCoords {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Sprite {
    pub filename: String,
    pub w: usize,
    pub h: usize,
    pub coords: HashMap<String, SpriteCoords>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Spritesheet {
    #[serde(rename = "0.3835")]
    pub sprites: Sprite,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    pub normal_active: Spritesheet,
    pub group_background: Spritesheet,
    pub frame: Spritesheet,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub skill: Option<usize>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub is_notable: Option<bool>,
    pub group: Option<usize>,
    pub orbit: Option<usize>,
    pub orbit_index: Option<usize>,
    pub is_proxy: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Constants {
    pub orbit_radii: Vec<usize>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TreeExport {
    pub tree: String,
    pub classes: Vec<Class>,
    pub groups: HashMap<String, Group>,
    pub nodes: HashMap<String, Node>,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub sprites: Sprites,
    pub constants: Constants,
}

impl TreeExport {
    pub fn new() -> Option<TreeExport> {
        match serde_json::from_str(&String::from_utf8_lossy(TREE_DATA)) {
            Ok(te) => Some(te),
            Err(e) => panic!("{}", e),
        }
    }
}
