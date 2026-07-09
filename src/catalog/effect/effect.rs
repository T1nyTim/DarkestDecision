use crate::catalog::effect::{ApplicationKind, Buff, CombatStatBuff, Condition, Duration, MonsterType, StatusEffect, Target};

pub enum Effect {
    AbyssalKiller(u8),
    AbyssalStun(u8),
    Adrenaline(u8),
    AntiqBlight(u8),
    AntiqBlightBuff(u8),
    AntiqBlightDebuff(u8),
    AntiqCower(u8),
    AntiqDefBuff(u8),
    AntiqDistract(u8),
    AntiqDodge(u8),
    AntiqProtectMeClearGuardsPerformer,
    AntiqProtectMeClearGuardsTarget,
    AntiqProtectMeGuard,
    AntiqSelfSpeed(u8),
    ArbMarkDebuff(u8),
    ArbMarkTarget,
    ArbSelfSpeed(u8),
    ArbStackingHeal(u8),
    BeastKiller(u8),
    BellowCrit(u8),
    BhDmgMarked,
    BhMarkDebuff(u8),
    BhMarkTarget,
    BhMinorMark,
    BhSelfSpeed(u8),
    Bleed(u8),
    BoloPush1(u8),
    Bolster(u8),
    BolsterStressBuff(u8),
    BuildToFinale(u8),
    BuildToFinaleSong,
    CaltropsPreyDebuff(u8),
    CaltropsSpdDebuff(u8),
    ClearCorpses,
    ClearGuardPerformer,
    ClearGuardTarget,
    Command(u8),
    CrusaderBulwark(u8),
    CrusaderBulwarkLight,
    CrusaderBulwarkMark,
    CrusaderHealStress(u8),
    CrusaderLight(u8),
    Cure,
    Cureself,
    Darkness,
    DazzlingLight,
    Defender(u8),
    Destealth,
    Disorient(u8),
    Disrupt(u8),
    DodgeCurse(u8),
    EldritchKiller(u8),
    EmboldenTeam(u8),
    FlareClear,
    FlareHealStress(u8),
    FlareLight(u8),
    FortifyResists(u8),
    GrAccBuff(u8),
    GrapeshotVulnerability(u8),
    GrBleedDebuff(u8),
    GrBlight(u8),
    GrBlightDebuff(u8),
    GrDaggerDmgMarked(u8),
    GrDodge(u8),
    GrFadeAttack(u8),
    GrSelfSpeed(u8),
    HarryBleed(u8),
    HellionExhaust,
    HellionExhaustSm,
    HellionHealSelf(u8),
    HeroStrongStun(u8),
    HmDmgMarked(u8),
    HmGuard,
    HmMarkTarget,
    HoundBleed(u8),
    HoundDebuff(u8),
    HoundHowl(u8),
    HoundProtect(u8),
    HwOpenVeinBleedDebuff(u8),
    HwOpenVeinSpdDebuff(u8),
    HwPistolDmgMarked(u8),
    HwyRiposte(u8),
    InspiringTune(u8),
    JesterSpotlight(u8),
    JesterTuneBuff(u8),
    LeperAcc,
    LeperHealSelf(u8),
    LeperHealSelfStress(u8),
    LeperHype(u8),
    LeperIntimidate(u8),
    LeperIntimidateMark,
    LeperMarkSelf,
    LeperProtect(u8),
    LeperResistBuff,
    LeperStrength(u8),
    LeperVulnerability,
    LickWounds(u8),
    MaaGuard,
    MaaRiposte(u8),
    ManKiller(u8),
    MarkSelf,
    MarkTarget,
    MortalWeakness,
    MortalWeaknessStress,
    NoxiousDebuff(u8),
    OccVulnerabilityCurse(u8),
    OccWeakeningCurse(u8),
    OccWeakenProt(u8),
    OnCritAcc,
    OnCritBleedChance,
    OnCritBlightChance,
    OnCritDef,
    OnCritDmg,
    OnCritDmgBleeding,
    OnCritDmgMarked,
    OnCritHealDone,
    OnCritProt,
    OnCritSpeed,
    OnCritStressHealDone,
    OnCritStressResist,
    PdBlight(u8),
    PdDisorientingStun(u8),
    PdSingleBlight(u8),
    PdVapoursBuff(u8),
    PoisonKiller(u8),
    Pull2(u8),
    Push1(u8),
    Push2(u8),
    Push3(u8),
    ShadowBlood(u8),
    SniperDamage(u8),
    SoloMarkSelf,
    Stealth,
    StealthSelf,
    StrongBleed(u8),
    Stun(u8),
    StunKiller(u8),
    Suppression(u8),
    TakeAim,
    TrackingBuff(u8),
    UnholyKiller(u8),
    VestalHealSelf(u8),
    VestalInspiration(u8),
    VestalLight(u8),
    VestalStun(u8),
    WyrdBleed(u8),
}

impl Effect {
    const fn application_kind(&self) -> ApplicationKind {
        match self {
            Self::AbyssalKiller(_)
            | Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritHealDone
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::OnCritStressHealDone
            | Self::OnCritStressResist => ApplicationKind::ApplyOnce,
            Self::AbyssalStun(_) | Self::Adrenaline(_) | Self::AntiqBlight(_) | Self::AntiqBlightBuff(_) => ApplicationKind::Queue,
        }
    }

    const fn buffs(&self) -> &'static [Buff] {
        match self {
            Self::AntiqBlightBuff(lv) => &[Buff::AntiqBlightBuff(lv)],
            Self::OnCritAcc => &[Buff::OnCritAcc],
            Self::OnCritBleedChance => &[Buff::OnCritBleedChance],
            Self::OnCritBlightChance => &[Buff::OnCritBlightChance],
            Self::OnCritDef => &[Buff::OnCritDef],
            Self::OnCritDmg => &[Buff::OnCritDmgH, Buff::OnCritDmgL],
            Self::OnCritDmgBleeding => &[Buff::OnCritDmgHBleeding, Buff::OnCritDmgLBleeding],
            Self::OnCritDmgMarked => &[Buff::OnCritDmgHMarked, Buff::OnCritDmgHMarked],
            Self::OnCritHealDone => &[Buff::OnCritHealBuff],
            Self::OnCritProt => &[Buff::OnCritProt],
            Self::OnCritSpeed => &[Buff::OnCritSpd],
            Self::OnCritStressHealDone => &[Buff::OnCritStressHealBuff],
            Self::OnCritStressResist => &[Buff::OnCritStressResist],
            Self::AbyssalKiller(_) | Self::AbyssalStun(_) | Self::Adrenaline(_) | Self::AntiqBlight(_) => &[],
        }
    }

    const fn chance(&self) -> u8 {
        match self {
            Self::AbyssalStun(lv) => 10 * lv + 100,
            Self::AntiqBlight(lv) => 10 * lv + 90,
            Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritHealDone
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::OnCritStressHealDone
            | Self::OnCritStressResist => 100,
        }
    }

    const fn combat_stat_buff(&self) -> &'static [CombatStatBuff] {
        match self {
            Self::Adrenaline(1) => &[
                CombatStatBuff::AttackRatingAdd(5),
                CombatStatBuff::DamageLowMultiply(20),
                CombatStatBuff::DamageHighMultiply(20),
            ],
            Self::Adrenaline(2) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::DamageLowMultiply(22),
                CombatStatBuff::DamageHighMultiply(22),
            ],
            Self::Adrenaline(3) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::DamageLowMultiply(24),
                CombatStatBuff::DamageHighMultiply(24),
            ],
            Self::Adrenaline(4) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::DamageLowMultiply(26),
                CombatStatBuff::DamageHighMultiply(26),
            ],
            Self::Adrenaline(5) => &[
                CombatStatBuff::AttackRatingAdd(9),
                CombatStatBuff::DamageLowMultiply(30),
                CombatStatBuff::DamageHighMultiply(30),
            ],
            Self::AbyssalKiller(1) => &[CombatStatBuff::DamageLowMultiply(15), CombatStatBuff::DamageHighMultiply(15)],
            Self::AbyssalKiller(2) => &[CombatStatBuff::DamageLowMultiply(17), CombatStatBuff::DamageHighMultiply(17)],
            Self::AbyssalKiller(3) => &[CombatStatBuff::DamageLowMultiply(20), CombatStatBuff::DamageHighMultiply(20)],
            Self::AbyssalKiller(4) => &[CombatStatBuff::DamageLowMultiply(22), CombatStatBuff::DamageHighMultiply(22)],
            Self::AbyssalKiller(5) => &[CombatStatBuff::DamageLowMultiply(25), CombatStatBuff::DamageHighMultiply(25)],
            Self::AbyssalStun(_) | Self::AntiqBlight(_) | Self::AntiqBlightBuff(_) => &[],
        }
    }

    const fn condition(&self) -> Option<Condition> {
        match self {
            Self::AbyssalKiller(_) => Some(Condition::MonsterType(MonsterType::Eldritch)),
            Self::AbyssalStun(_) | Self::Adrenaline(_) | Self::AntiqBlight(_) | Self::AntiqBlightBuff(_) => None,
        }
    }

    const fn duration(&self) -> Option<Duration> {
        match self {
            Self::AbyssalStun(_) => Some(Duration::Rounds(1)),
            Self::OnCritBleedChance | Self::OnCritBlightChance | Self::OnCritHealDone | Self::OnCritStressHealDone => Some(Duration::Rounds(2)),
            Self::AntiqBlight(_) | Self::OnCritDmg | Self::OnCritDmgBleeding | Self::OnCritDmgMarked | Self::OnCritStressResist => {
                Some(Duration::Rounds(3))
            }
            Self::Adrenaline(_) | Self::AntiqBlightBuff(_) => Some(Duration::Rounds(4)),
            Self::AbyssalKiller(_) | Self::OnCritAcc | Self::OnCritDef | Self::OnCritProt | Self::OnCritSpeed => None,
        }
    }

    const fn on_hit(&self) -> bool {
        match self {
            Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritHealDone
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::OnCritStressHealDone
            | Self::OnCritStressResist => true,
        }
    }

    const fn on_miss(&self) -> bool {
        match self {
            Self::Adrenaline(_) | Self::AntiqBlightBuff(_) => true,
            Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritHealDone
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::OnCritStressHealDone
            | Self::OnCritStressResist => false,
        }
    }

    const fn status_effect(&self) -> Option<StatusEffect> {
        match self {
            Self::Adrenaline(_) => Some(StatusEffect::Cure),
            Self::AntiqBlight(1) => Some(StatusEffect::DotPoison(1)),
            Self::AntiqBlight(2..=3) => Some(StatusEffect::DotPoison(2)),
            Self::AntiqBlight(4) => Some(StatusEffect::DotPoison(3)),
            Self::AntiqBlight(5) => Some(StatusEffect::DotPoison(4)),
            Self::AbyssalStun(_) => Some(StatusEffect::Stun),
            Self::AbyssalKiller(_) | Self::AntiqBlightBuff(_) => None,
        }
    }

    const fn target(&self) -> Target {
        match self {
            Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritHealDone
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::OnCritStressHealDone
            | Self::OnCritStressResist => Target::Performer,
            Self::AbyssalStun(_) | Self::AntiqBlight(_) => Target::Target,
        }
    }
}
