use crate::catalog::{
    Rank,
    effect::effect::Effect,
    heroes::{AttackData, CombatSkillData, Mode, Percent10, SkillData, TargetMod, combat_skills::COMBAT_SKILLS},
};

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum CombatSkill {
    Absolution,
    AbyssalArtillery,
    AdrenalineRush,
    BarbaricYawp,
    BattleBallad,
    BattlefieldBandage,
    BattlefieldMedicine,
    BattleHeal,
    Bellow,
    Blackjack,
    BleedOut,
    Blindfire,
    BlindingGas,
    Bloodlet,
    Bola,
    Bolster,
    Breakthru,
    BulwarkOfFaith,
    Chop,
    Crush,
    CollectBounty,
    ComeHither,
    Command,
    Cower,
    DaemonsPull,
    DazzlingLight,
    Defender,
    DirkStab,
    DisorientingBlast,
    DisruptiveCurse,
    DivineGrace,
    DuelistAdvance,
    EmboldeningVapours,
    FesteringVapours,
    FinishHim,
    Flare,
    Flashbang,
    FlashingDaggers,
    Flashpowder,
    Focus,
    FortifyingVapours,
    GodsComfort,
    GodsHand,
    GodsIllumination,
    GrapeShotBlast,
    GuardDog,
    HandsFromAbyss,
    Harvest,
    HeroicEnd,
    Hew,
    HolyLance,
    HookAndSlice,
    HoundsHarry,
    HoundsRush,
    Howl,
    IfItBleeds,
    Incision,
    InspiringCry,
    InspiringTune,
    Intimidate,
    InvigoratingVapours,
    IronSwan,
    Judgement,
    KrisStab,
    LickWounds,
    Lunge,
    MaceBash,
    Manacles,
    NoxiousBlast,
    OpenedVein,
    Pick,
    PistolShot,
    PlagueGrenade,
    PointBlankShot,
    PoisonDart,
    ProtectMe,
    Rage,
    Rake,
    Rampart,
    Retribution,
    Revenge,
    ShadowFade,
    Slam,
    SliceOff,
    Smite,
    SniperMark,
    SniperShot,
    Solemnity,
    Solo,
    StunningBlow,
    SuppressingFire,
    TargetTag,
    TakeAim,
    ThrownDagger,
    ToxinTrickery,
    Transform,
    Uppercut,
    Vomit,
    WeakeningCurse,
    Whistle,
    WickedHack,
    WickedSlice,
    Withstand,
    WyrdReconstruction,
    ZealousAccusation,
}

impl CombatSkill {
    const fn atk(&self, lv: usize) -> u8 {
        self.attack().atk[lv]
    }

    const fn attack(&self) -> AttackData {
        match self.data().skill_data {
            SkillData::Melee(attack) | SkillData::Ranged(attack) => attack,
            SkillData::Heal(_) => panic!("attack data requested for a healing skill"),
        }
    }

    const fn battle_limit(&self) -> u8 {
        self.data().battle_limit
    }

    const fn crit(&self, lv: usize) -> Percent10 {
        self.attack().crit[lv]
    }

    const fn data(&self) -> &'static CombatSkillData {
        &COMBAT_SKILLS[*self as usize]
    }

    const fn dmg(&self) -> i8 {
        self.attack().dmg
    }

    const fn heal(&self, lv: usize) -> (u8, u8) {
        match self.data().skill_data {
            SkillData::Heal(heal) => heal[lv],
            SkillData::Melee(_) | SkillData::Ranged(_) => panic!("heal() called for a non-healing skill"),
        }
    }

    const fn ignore_guard(&self) -> bool {
        self.data().ignore_guard
    }

    const fn ignore_protection(&self) -> bool {
        self.data().ignore_protection
    }

    const fn ignore_stealth(&self) -> bool {
        self.data().ignore_stealth
    }

    const fn is_continue_turn(&self) -> bool {
        self.data().is_continue_turn
    }

    const fn is_crit_valid(&self) -> bool {
        self.attack().is_crit_valid
    }

    const fn is_stall_invalidating(&self) -> bool {
        self.data().is_stall_invalidating
    }

    const fn launch(&self) -> &'static [Rank] {
        self.data().launch
    }

    const fn movement(&self) -> (u8, u8) {
        self.data().movement
    }

    const fn skill_data(&self) -> SkillData {
        self.data().skill_data
    }

    const fn target(&self) -> TargetMod {
        self.data().target
    }

    const fn turn_limit(&self) -> u8 {
        self.data().turn_limit
    }

    fn effects(&self, lv: usize, mode: Option<Mode>) -> Option<&'static [Effect]> {
        let data = self.data();
        if let (Some(current_mode), Some(mode_effects)) = (mode, data.mode_effects.clone()) {
            for (stored_mode, effects) in mode_effects {
                if stored_mode == current_mode {
                    return Some(effects[lv]);
                }
            }
        }
        match data.effects {
            Some(effects) => Some(effects[lv]),
            None => None,
        }
    }
}
