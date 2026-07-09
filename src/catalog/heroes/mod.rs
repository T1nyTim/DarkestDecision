mod combat_skill;
mod heroes;

use serde::Deserialize;

use crate::catalog::{Loot, Rank, Resistances, effect::effect::Effect, heroes::combat_skill::CombatSkill};

type Percent10 = i16;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeroClass {
    Abomination,
    Antiquarian,
    Arbalest,
    BountyHunter,
    Crusader,
    GraveRobber,
    Hellion,
    Highwayman,
    Houndmaster,
    Jester,
    Leper,
    ManAtArms,
    Occultist,
    PlagueDoctor,
    Vestal,
}

impl HeroClass {
    pub const ALL: &'static [Self] = &[
        Self::Abomination,
        Self::Antiquarian,
        Self::Arbalest,
        Self::BountyHunter,
        Self::Crusader,
        Self::GraveRobber,
        Self::Hellion,
        Self::Highwayman,
        Self::Houndmaster,
        Self::Jester,
        Self::Leper,
        Self::ManAtArms,
        Self::Occultist,
        Self::PlagueDoctor,
        Self::Vestal,
    ];

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Abomination => "Abomination",
            Self::Antiquarian => "Antiquarian",
            Self::Arbalest => "Arbalest",
            Self::BountyHunter => "Bounty Hunter",
            Self::Crusader => "Crusader",
            Self::GraveRobber => "Grave Robber",
            Self::Hellion => "Hellion",
            Self::Highwayman => "Highwayman",
            Self::Houndmaster => "Houndmaster",
            Self::Jester => "Jester",
            Self::Leper => "Leper",
            Self::ManAtArms => "Man-at-Arms",
            Self::Occultist => "Occultist",
            Self::PlagueDoctor => "Plague Doctor",
            Self::Vestal => "Vestal",
        }
    }

    pub const fn value(&self) -> &'static str {
        match self {
            Self::Abomination => "abomination",
            Self::Antiquarian => "antiquarian",
            Self::Arbalest => "arbalest",
            Self::BountyHunter => "bounty_hunter",
            Self::Crusader => "crusader",
            Self::GraveRobber => "grave_robber",
            Self::Hellion => "hellion",
            Self::Highwayman => "highwayman",
            Self::Houndmaster => "houndmaster",
            Self::Jester => "jester",
            Self::Leper => "leper",
            Self::ManAtArms => "man_at_arms",
            Self::Occultist => "occultist",
            Self::PlagueDoctor => "plague_doctor",
            Self::Vestal => "vestal",
        }
    }

    const fn crit_by_weapon_lv(&self, lv: usize) -> u8 {
        let crit = match self {
            Self::Abomination | Self::ManAtArms | Self::PlagueDoctor => [2, 3, 4, 5, 6],
            Self::Antiquarian | Self::Leper | Self::Vestal => [1, 2, 3, 4, 5],
            Self::Arbalest | Self::GraveRobber | Self::Occultist => [6, 7, 8, 9, 10],
            Self::BountyHunter | Self::Houndmaster | Self::Jester => [4, 5, 6, 7, 8],
            Self::Crusader => [3, 4, 5, 6, 7],
            Self::Hellion | Self::Highwayman => [5, 6, 7, 8, 9],
        };
        crit[lv]
    }

    const fn def_by_armour_lv(&self, lv: usize) -> Percent10 {
        let def = match self {
            Self::Abomination => [75, 125, 175, 225, 275],
            Self::Antiquarian | Self::GraveRobber | Self::Hellion | Self::Highwayman | Self::Houndmaster | Self::Occultist => {
                [100, 150, 200, 250, 300]
            }
            Self::Arbalest | Self::Leper | Self::PlagueDoctor | Self::Vestal => [0, 50, 100, 150, 200],
            Self::BountyHunter | Self::Crusader | Self::ManAtArms => [50, 100, 150, 200, 250],
            Self::Jester => [150, 200, 250, 300, 350],
        };
        def[lv]
    }

    const fn dmg_by_weapon_lv(&self, lv: usize) -> (u8, u8) {
        let dmg = match self {
            Self::Abomination => [(6, 11), (7, 13), (8, 15), (10, 18), (11, 20)],
            Self::Antiquarian => [(3, 5), (4, 6), (4, 7), (5, 8), (5, 9)],
            Self::Arbalest | Self::GraveRobber | Self::Vestal => [(4, 8), (5, 10), (6, 11), (6, 13), (7, 14)],
            Self::BountyHunter => [(5, 10), (6, 12), (7, 13), (7, 15), (8, 16)],
            Self::Crusader | Self::Hellion => [(6, 12), (7, 14), (8, 16), (9, 17), (10, 19)],
            Self::Highwayman => [(5, 10), (6, 12), (7, 13), (8, 15), (9, 16)],
            Self::Houndmaster | Self::Jester | Self::Occultist | Self::PlagueDoctor => [(4, 7), (5, 8), (6, 10), (6, 11), (7, 13)],
            Self::Leper => [(8, 16), (9, 18), (10, 21), (12, 23), (13, 26)],
            Self::ManAtArms => [(5, 9), (6, 10), (7, 12), (7, 13), (8, 14)],
        };
        dmg[lv]
    }

    const fn hp_by_armour_lv(&self, lv: usize) -> u8 {
        let hp = match self {
            Self::Abomination | Self::Hellion => [26, 31, 36, 41, 46],
            Self::Antiquarian => [17, 20, 23, 26, 29],
            Self::Arbalest => [27, 32, 37, 42, 47],
            Self::BountyHunter => [25, 30, 35, 40, 45],
            Self::Crusader => [33, 40, 47, 54, 61],
            Self::GraveRobber => [20, 24, 28, 32, 36],
            Self::Highwayman => [23, 28, 33, 38, 43],
            Self::Houndmaster => [21, 25, 29, 33, 37],
            Self::Jester | Self::Occultist => [19, 23, 27, 31, 35],
            Self::Leper => [35, 42, 49, 56, 63],
            Self::ManAtArms => [31, 37, 43, 49, 55],
            Self::PlagueDoctor => [22, 26, 30, 34, 38],
            Self::Vestal => [24, 29, 34, 39, 44],
        };
        hp[lv]
    }

    const fn spd_by_weapon_lv(&self, lv: usize) -> u8 {
        let spd = match self {
            Self::Abomination | Self::Jester | Self::PlagueDoctor => [7, 7, 8, 8, 9],
            Self::Antiquarian | Self::BountyHunter | Self::Highwayman | Self::Houndmaster => [5, 5, 6, 6, 7],
            Self::Arbalest | Self::ManAtArms => [3, 3, 4, 4, 5],
            Self::Crusader => [1, 1, 2, 2, 3],
            Self::GraveRobber => [8, 8, 9, 9, 10],
            Self::Hellion | Self::Vestal => [4, 4, 5, 5, 6],
            Self::Leper => [2, 2, 3, 3, 4],
            Self::Occultist => [6, 6, 7, 7, 8],
        };
        spd[lv]
    }
}

pub enum Mode {
    Human,
    Beast,
}

pub enum SkillType {
    Heal,
    Melee,
    Ranged,
}

pub enum TargetMod {
    Single,
    Multi,
    Allies,
    Performer,
    AlliesMulti,
    OtherAllies,
    Random,
}

pub struct CombatMoveSkill {
    back: u8,
    forward: u8,
}

impl CombatMoveSkill {
    pub const fn new(back: u8, forward: u8) -> Self {
        Self { back, forward }
    }
}

pub struct ExtraEconomy {
    battle_loot: Option<Loot>,
    curio_loot: Option<Loot>,
    stack_limit: u16,
    shard_bonus: u8,
}

impl ExtraEconomy {
    const fn new(battle_loot: Option<Loot>, curio_loot: Option<Loot>, stack_limit: u16, shard_bonus: u8) -> Self {
        Self {
            battle_loot,
            curio_loot,
            stack_limit,
            shard_bonus,
        }
    }
}

pub struct HeroData {
    class: HeroClass,
    resistances: Resistances,
    crit: Effect,
    combat_skills: [CombatSkill; 7],
    move_skill: CombatMoveSkill,
    riposte: RiposteSkill,
    controlled: Rank,
    combat_skills_max: u8,
    modes: &'static [Mode],
    extra_economy: ExtraEconomy,
}

impl HeroData {
    pub const fn new(
        class: HeroClass,
        resistances: Resistances,
        crit: Effect,
        combat_skills: [CombatSkill; 7],
        move_skill: CombatMoveSkill,
        riposte: RiposteSkill,
        controlled: Rank,
        combat_skills_max: u8,
        modes: &'static [Mode],
        extra_economy: ExtraEconomy,
    ) -> Self {
        Self {
            class,
            resistances,
            crit,
            combat_skills,
            move_skill,
            riposte,
            controlled,
            combat_skills_max,
            modes,
            extra_economy,
        }
    }
}

pub struct RiposteSkill {
    atk: u8,
    crit: Percent10,
}

impl RiposteSkill {
    pub const fn new(atk: u8, crit: Percent10) -> Self {
        Self { atk, crit }
    }
}
