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
            Self::AntiqProtectMeClearGuardsPerformer | Self::AntiqProtectMeClearGuardsTarget => ApplicationKind::Immediate,
            Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_) => ApplicationKind::Queue,
        }
    }

    const fn buffs(&self) -> &'static [Buff] {
        match self {
            Self::AntiqBlightBuff(1) => &[Buff::AntiqBlightBuff(1)],
            Self::AntiqBlightBuff(2) => &[Buff::AntiqBlightBuff(2)],
            Self::AntiqBlightBuff(3) => &[Buff::AntiqBlightBuff(3)],
            Self::AntiqBlightBuff(4) => &[Buff::AntiqBlightBuff(4)],
            Self::AntiqBlightBuff(5) => &[Buff::AntiqBlightBuff(5)],
            Self::AntiqBlightDebuff(1) => &[Buff::AntiqBlightDebuff(1)],
            Self::AntiqBlightDebuff(2) => &[Buff::AntiqBlightDebuff(2)],
            Self::AntiqBlightDebuff(3) => &[Buff::AntiqBlightDebuff(3)],
            Self::AntiqBlightDebuff(4) => &[Buff::AntiqBlightDebuff(4)],
            Self::AntiqBlightDebuff(5) => &[Buff::AntiqBlightDebuff(5)],
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
            Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget => &[],
        }
    }

    const fn chance(&self) -> u8 {
        match self {
            Self::AbyssalStun(5) => 150,
            Self::AbyssalStun(4) | Self::AntiqBlight(5) | Self::AntiqBlightDebuff(5) | Self::AntiqDistract(5) => 140,
            Self::AbyssalStun(3) | Self::AntiqBlight(4) | Self::AntiqBlightDebuff(4) | Self::AntiqDistract(4) => 130,
            Self::AbyssalStun(2) | Self::AntiqBlight(3) | Self::AntiqBlightDebuff(3) | Self::AntiqDistract(3) => 120,
            Self::AbyssalStun(1) | Self::AntiqBlight(2) | Self::AntiqBlightDebuff(2) | Self::AntiqDistract(2) => 110,
            Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(1)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(1)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(1)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
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
            Self::Adrenaline(5) => &[
                CombatStatBuff::AttackRatingAdd(9),
                CombatStatBuff::DamageHighMultiply(30),
                CombatStatBuff::DamageLowMultiply(30),
            ],
            Self::Adrenaline(4) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::DamageHighMultiply(26),
                CombatStatBuff::DamageLowMultiply(26),
            ],
            Self::Adrenaline(3) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::DamageHighMultiply(24),
                CombatStatBuff::DamageLowMultiply(24),
            ],
            Self::Adrenaline(2) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::DamageHighMultiply(22),
                CombatStatBuff::DamageLowMultiply(22),
            ],
            Self::Adrenaline(1) => &[
                CombatStatBuff::AttackRatingAdd(5),
                CombatStatBuff::DamageHighMultiply(20),
                CombatStatBuff::DamageLowMultiply(20),
            ],
            Self::AntiqDistract(1) => &[CombatStatBuff::AttackRatingAdd(-10)],
            Self::AntiqDistract(2) => &[CombatStatBuff::AttackRatingAdd(-11)],
            Self::AntiqDistract(3) => &[CombatStatBuff::AttackRatingAdd(-12)],
            Self::AntiqDistract(4) => &[CombatStatBuff::AttackRatingAdd(-14)],
            Self::AntiqDistract(5) => &[CombatStatBuff::AttackRatingAdd(-15)],
            Self::AbyssalKiller(5) => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
            Self::AbyssalKiller(4) => &[CombatStatBuff::DamageHighMultiply(22), CombatStatBuff::DamageLowMultiply(22)],
            Self::AbyssalKiller(3) => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
            Self::AbyssalKiller(2) => &[CombatStatBuff::DamageHighMultiply(17), CombatStatBuff::DamageLowMultiply(17)],
            Self::AbyssalKiller(1) => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            Self::AntiqCower(5) => &[CombatStatBuff::DefenseRatingAdd(25)],
            Self::AntiqCower(4) => &[CombatStatBuff::DefenseRatingAdd(22)],
            Self::AntiqCower(3) => &[CombatStatBuff::DefenseRatingAdd(20)],
            Self::AntiqCower(2) => &[CombatStatBuff::DefenseRatingAdd(18)],
            Self::AntiqCower(1) => &[CombatStatBuff::DefenseRatingAdd(15)],
            Self::AntiqDodge(5) => &[CombatStatBuff::DefenseRatingAdd(10)],
            Self::AntiqDodge(4) => &[CombatStatBuff::DefenseRatingAdd(9)],
            Self::AntiqDefBuff(5) => &[CombatStatBuff::DefenseRatingAdd(8), CombatStatBuff::ProtectionRatingAdd(20)],
            Self::AntiqDodge(3) => &[CombatStatBuff::DefenseRatingAdd(7)],
            Self::AntiqDefBuff(4) => &[CombatStatBuff::DefenseRatingAdd(7), CombatStatBuff::ProtectionRatingAdd(18)],
            Self::AntiqDefBuff(3) => &[CombatStatBuff::DefenseRatingAdd(6), CombatStatBuff::ProtectionRatingAdd(15)],
            Self::AntiqDodge(2) => &[CombatStatBuff::DefenseRatingAdd(5)],
            Self::AntiqDefBuff(2) => &[CombatStatBuff::DefenseRatingAdd(5), CombatStatBuff::ProtectionRatingAdd(13)],
            Self::AntiqDefBuff(1) => &[CombatStatBuff::DefenseRatingAdd(4), CombatStatBuff::ProtectionRatingAdd(10)],
            Self::AntiqDodge(1) => &[CombatStatBuff::DefenseRatingAdd(3)],
            Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget => &[],
        }
    }

    const fn condition(&self) -> Option<Condition> {
        match self {
            Self::AbyssalKiller(_) => Some(Condition::MonsterType(MonsterType::Eldritch)),
            Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget => None,
        }
    }

    const fn duration(&self) -> Option<Duration> {
        match self {
            Self::Adrenaline(_) | Self::AntiqBlightBuff(_) | Self::AntiqCower(_) | Self::AntiqDefBuff(_) => Some(Duration::Rounds(4)),
            Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqDodge(_)
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritStressResist => Some(Duration::Rounds(3)),
            Self::AntiqDistract(_) | Self::OnCritBleedChance | Self::OnCritBlightChance | Self::OnCritHealDone | Self::OnCritStressHealDone => {
                Some(Duration::Rounds(2))
            }
            Self::AbyssalStun(_) => Some(Duration::Rounds(1)),
            Self::AbyssalKiller(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::OnCritAcc
            | Self::OnCritDef
            | Self::OnCritProt
            | Self::OnCritSpeed => None,
        }
    }

    const fn on_hit(&self) -> bool {
        match self {
            Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
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
            Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget => true,
            Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqDistract(_)
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

    const fn status_effect(&self) -> &'static [StatusEffect] {
        match self {
            Self::AntiqProtectMeClearGuardsPerformer | Self::AntiqProtectMeClearGuardsTarget => {
                &[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding]
            }
            Self::Adrenaline(_) => &[StatusEffect::Cure],
            Self::AntiqBlight(1) => &[StatusEffect::DotPoison(1)],
            Self::AntiqBlight(2..=3) => &[StatusEffect::DotPoison(2)],
            Self::AntiqBlight(4) => &[StatusEffect::DotPoison(3)],
            Self::AntiqBlight(5) => &[StatusEffect::DotPoison(4)],
            Self::AbyssalStun(_) => &[StatusEffect::Stun],
            Self::AbyssalKiller(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_) => &[],
        }
    }

    const fn target(&self) -> Target {
        match self {
            Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqProtectMeClearGuardsPerformer
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
            Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsTarget => Target::Target,
        }
    }
}
