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

struct Armour {
    def: Percent10,
    hp: u8,
}

impl Armour {
    const fn new(def: Percent10, hp: u8) -> Self {
        Self { def, hp }
    }
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
    weapons: [Weapon; 5],
    armours: [Armour; 5],
    combat_skills: [CombatSkill; 7],
    move_skill: CombatMoveSkill,
    riposte: Option<RiposteSkill>,
    controlled: Rank,
    combat_skills_max: u8,
    modes: &'static [Mode],
    extra_economy: Option<ExtraEconomy>,
}

impl HeroData {
    pub const fn new(
        class: HeroClass,
        resistances: Resistances,
        crit: Effect,
        weapons: [Weapon; 5],
        armours: [Armour; 5],
        combat_skills: [CombatSkill; 7],
        move_skill: CombatMoveSkill,
        controlled: Rank,
        combat_skills_max: u8,
    ) -> Self {
        Self {
            class,
            resistances,
            crit,
            weapons,
            armours,
            combat_skills,
            move_skill,
            riposte: None,
            controlled,
            combat_skills_max,
            modes: &[],
            extra_economy: None,
        }
    }

    pub const fn with_extra_economy(mut self, extra_economy: ExtraEconomy) -> Self {
        self.extra_economy = Some(extra_economy);
        self
    }

    pub const fn with_modes(mut self, modes: &'static [Mode]) -> Self {
        self.modes = modes;
        self
    }

    pub const fn with_riposte(mut self, riposte: RiposteSkill) -> Self {
        self.riposte = Some(riposte);
        self
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

pub struct Weapon {
    dmg: (u8, u8),
    crit: u8,
    spd: u8,
}

impl Weapon {
    pub const fn new(dmg: (u8, u8), crit: u8, spd: u8) -> Self {
        Self { dmg, crit, spd }
    }
}
