mod effect;
mod encounter;
pub mod encounter_id;
pub mod enemies;
pub mod heroes;

use serde::Deserialize;

use crate::catalog::{encounter::ENCOUNTERS, encounter_id::EncounterId, enemies::EnemyKind};

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Area {
    Hall,
    Room,
}

impl Area {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Hall => "Hall",
            Self::Room => "Room",
        }
    }

    pub const fn value(&self) -> &'static str {
        match self {
            Self::Hall => "hall",
            Self::Room => "room",
        }
    }

    pub fn filter(location: &Location, difficulty: &Difficulty) -> Vec<Self> {
        Self::ALL
            .iter()
            .filter(|&area| {
                ENCOUNTERS
                    .iter()
                    .any(|encounter| encounter.location == *location && encounter.difficulty == *difficulty && encounter.area == *area)
            })
            .cloned()
            .collect()
    }

    const ALL: &'static [Self] = &[Self::Hall, Self::Room];
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Apprentice,
    Veteran,
    Champion,
    Darkest,
}

impl Difficulty {
    pub const ALL: &'static [Self] = &[Self::Apprentice, Self::Veteran, Self::Champion, Self::Darkest];

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Apprentice => "Apprentice (Lvl 1)",
            Self::Veteran => "Veteran (Lvl 3)",
            Self::Champion => "Champion (Lvl 5)",
            Self::Darkest => "Darkest (Lvl 6)",
        }
    }

    pub const fn value(&self) -> &'static str {
        match self {
            Self::Apprentice => "apprentice",
            Self::Veteran => "veteran",
            Self::Champion => "champion",
            Self::Darkest => "darkest",
        }
    }

    pub fn filter(location: &Location) -> Vec<Self> {
        Self::ALL
            .iter()
            .filter(|&difficulty| {
                ENCOUNTERS
                    .iter()
                    .any(|encounter| encounter.location == *location && encounter.difficulty == *difficulty)
            })
            .cloned()
            .collect()
    }
}

#[derive(Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Location {
    Tutorial,
    Cove,
    #[serde(rename = "ruins")]
    Crypts,
    Warrens,
    Weald,
    Darkest,
}

impl Location {
    pub const ALL: &'static [Self] = &[Self::Tutorial, Self::Cove, Self::Crypts, Self::Warrens, Self::Weald, Self::Darkest];

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Tutorial => "Tutorial",
            Self::Cove => "Cove",
            Self::Crypts => "Ruins",
            Self::Warrens => "Warrens",
            Self::Weald => "Weald",
            Self::Darkest => "Darkest",
        }
    }

    pub const fn value(&self) -> &'static str {
        match self {
            Self::Tutorial => "tutorial",
            Self::Cove => "cove",
            Self::Crypts => "ruins",
            Self::Warrens => "warrens",
            Self::Weald => "weald",
            Self::Darkest => "darkest",
        }
    }
}

pub enum Loot {
    Antiq,
}

pub enum Rank {
    One,
    Two,
    Three,
    Four,
}

pub struct EncounterRecord {
    location: Location,
    difficulty: Difficulty,
    area: Area,
    pub id: EncounterId,
    pub enemies: &'static [EnemyKind],
}

impl EncounterRecord {
    const fn new(location: Location, difficulty: Difficulty, area: Area, id: EncounterId, enemies: &'static [EnemyKind]) -> Self {
        Self {
            location,
            difficulty,
            area,
            id,
            enemies,
        }
    }

    pub fn filter(location: &Location, difficulty: &Difficulty, area: &Area) -> Vec<&'static Self> {
        ENCOUNTERS
            .iter()
            .filter(|encounter| encounter.location == *location && encounter.difficulty == *difficulty && encounter.area == *area)
            .collect()
    }

    pub fn find(location: &Location, difficulty: &Difficulty, area: &Area, encounter_id: &EncounterId) -> Option<&'static Self> {
        ENCOUNTERS.iter().find(|encounter| {
            &encounter.location == location && &encounter.difficulty == difficulty && &encounter.area == area && encounter.id == *encounter_id
        })
    }
    pub fn label(&self) -> String {
        self.enemies.iter().map(EnemyKind::label).collect::<Vec<_>>().join(" / ")
    }
}

pub struct Resistances {
    stun: u8,
    poison: u8,
    bleed: u8,
    disease: u8,
    move_res: u8,
    debuff: u8,
    death_blow: u8,
    trap: u8,
}

impl Resistances {
    pub const fn new(stun: u8, poison: u8, bleed: u8, disease: u8, move_res: u8, debuff: u8, death_blow: u8, trap: u8) -> Self {
        Self {
            stun,
            poison,
            bleed,
            disease,
            move_res,
            debuff,
            death_blow,
            trap,
        }
    }
}
