use egui::{util::History, Pos2, TextureId};

use crate::{camera::Camera, tree};

#[derive(Debug, PartialEq)]
pub enum CharacterClass {
    Scion,
    Marauder,
    Ranger,
    Witch,
    Duelist,
    Templar,
    Shadow,
}

impl CharacterClass {
    pub fn id(&self) -> i32 {
        match self {
            Self::Scion => 0,
            Self::Marauder => 1,
            Self::Ranger => 2,
            Self::Witch => 3,
            Self::Duelist => 4,
            Self::Templar => 5,
            Self::Shadow => 6,
        }
    }
}

pub enum View {
    PassiveTree,
    Note,
    NoteEdit,
}

pub struct TemplateApp {
    pub tree: tree::TreeExport,
    pub group_bg_tex: TextureId,
    pub bg_tex: TextureId,
    pub skills_tex: TextureId,
    pub inactive_skills_tex: TextureId,
    pub frame_tex: TextureId,
    pub mastery_tex: TextureId,
    pub ascendancy_tex: TextureId,
    pub frame_times: History<f32>,
    pub lines: Vec<(Pos2, Pos2)>,
    pub camera: Camera,
    pub selected_class: CharacterClass,
    pub selected_view: View,
    pub notes_buffer: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;
        let camera = Camera::new(0.0, 0.0, 1.0, 400.0, 300.0);
        let tree = tree::TreeExport::new().unwrap();
        Self {
            tree,
            group_bg_tex: TextureId::User(999),
            bg_tex: TextureId::User(999),
            skills_tex: TextureId::User(999),
            frame_tex: TextureId::User(999),
            inactive_skills_tex: TextureId::User(999),
            ascendancy_tex: TextureId::User(999),
            mastery_tex: Default::default(),
            lines: Default::default(),
            selected_class: CharacterClass::Scion,
            frame_times: History::new(0..max_len, max_age),
            selected_view: View::PassiveTree,
            notes_buffer: Default::default(),
            camera,
        }
    }
}
