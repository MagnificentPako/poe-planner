use std::{collections::HashMap, f32::consts::PI};

use egui::{pos2, Pos2};
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

pub const CLASS_ART: [&str; 8] = [
    "centerscion",
    "centermarauder",
    "centerranger",
    "centerwitch",
    "centerduelist",
    "centertemplar",
    "centershadow",
    "PSStartNodeBackgroundInactive",
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

#[derive(Serialize, Deserialize, Copy, Clone)]
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
    #[serde(rename = "0.2972")]
    pub worse_sprites: Sprite,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    pub normal_active: Spritesheet,
    pub group_background: Spritesheet,
    pub frame: Spritesheet,
    pub mastery_inactive: Spritesheet,
    pub start_node: Spritesheet,
    pub notable_active: Spritesheet,
    pub keystone_active: Spritesheet,
    pub ascendancy_background: Spritesheet,
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
    #[serde(default)]
    pub is_mastery: bool,
    #[serde(default)]
    pub inactive_icon: String,
    #[serde(default)]
    pub is_keystone: bool,
    #[serde(default)]
    pub out: Vec<String>,
    pub class_start_index: Option<usize>,
    pub ascendancy_name: Option<String>,
    #[serde(default)]
    pub is_ascendancy_start: bool,
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

pub enum FrameType {
    Normal,
    Notable,
    Keystone,
    None,
}

fn orbit_position(node: &Node, group: &Group) -> (f32, f32) {
    let radius = ORBIT_RADII[node.orbit.unwrap_or(0)] as f32;
    let skills_on_orbit = ORBIT_NODES[node.orbit.unwrap_or(0)];
    let orbit_index = node.orbit_index.unwrap_or(0);
    let two_pi = PI * 2.0;

    let angle = match skills_on_orbit {
        16 => (ORBIT_ANGLES_16[orbit_index] as f32).to_radians(),
        40 => (ORBIT_ANGLES_40[orbit_index] as f32).to_radians(),
        an => two_pi / an as f32 * orbit_index as f32,
    };

    let x = group.x + radius * angle.sin();
    let y = group.y - radius * angle.cos();

    (x, y)
}

impl TreeExport {
    pub fn new() -> Option<TreeExport> {
        match serde_json::from_str(&String::from_utf8_lossy(TREE_DATA)) {
            Ok(te) => Some(te),
            Err(e) => panic!("{}", e),
        }
    }

    pub fn node_position(&self, node: &Node) -> (f32, f32) {
        if let Some(group_num) = node.group {
            let group_str = format!("{group_num}");
            if let Some(group) = self.groups.get(&group_str) {
                return orbit_position(node, group);
            }
        }
        (0.0, 0.0)
    }

    pub fn generate_lines(&self) -> Vec<(Pos2, Pos2)> {
        let mut lines = vec![];
        for node in self.nodes.values() {
            let node_pos = self.node_position(node);
            let out_nodes: Vec<&Node> = node
                .out
                .iter()
                .map(|x| self.nodes.get(x).unwrap())
                .collect();
            for other in out_nodes {
                if !node.is_mastery
                    && !other.is_mastery
                    && !node.is_proxy.unwrap_or(false)
                    && !other.is_proxy.unwrap_or(false)
                    && node.class_start_index == other.class_start_index
                    && node.ascendancy_name == other.ascendancy_name
                {
                    let other_pos = self.node_position(other);
                    lines.push((pos2(node_pos.0, node_pos.1), pos2(other_pos.0, other_pos.1)));
                }
            }
        }

        lines
    }

    pub fn get_ascendancy_starts(&self) -> Vec<(Pos2, String)> {
        let start_nodes = self
            .nodes
            .values()
            .filter(|x| x.is_ascendancy_start)
            .collect::<Vec<&Node>>();
        start_nodes
            .into_iter()
            .map(|node| {
                let (_, group) = self
                    .groups
                    .iter()
                    .find(|group| group.0.parse::<usize>().unwrap() == node.group.unwrap())
                    .unwrap();
                (
                    pos2(group.x, group.y),
                    format!("Classes{}", &node.ascendancy_name.as_ref().unwrap()),
                )
            })
            .collect()
    }
}

impl Node {
    pub fn frame_type(&self) -> FrameType {
        if self.is_notable.unwrap_or(false) {
            return FrameType::Notable;
        }

        if self.is_keystone {
            return FrameType::Keystone;
        }

        if !self.is_proxy.unwrap_or(false) {
            return FrameType::Normal;
        }

        FrameType::None
    }
}
