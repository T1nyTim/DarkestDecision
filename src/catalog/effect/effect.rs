use crate::catalog::effect::{ApplicationKind, Buff, CombatStatBuff, Condition, Duration, EffectData, StatusEffect, Target, effects::effect_data};

pub enum Effect {
    AbomVomit(u8),
    AbsolutionHeal(u8),
    AbsolutionHealStress(u8),
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
    BeastBuff(u8),
    BeastBuff2(u8),
    BeastDebuff(u8),
    BeastKiller(u8),
    BeastStressParty,
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
    CureSelf,
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
    GrBleedDebuff(u8),
    GrBlight(u8),
    GrBlightDebuff(u8),
    GrDaggerDmgMarked(u8),
    GrDodge(u8),
    GrFadeAttack(u8),
    GrSelfSpeed(u8),
    GrapeshotVulnerability(u8),
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
    HumanStressHealParty,
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
    ManaclesStun(u8),
    MarkSelf,
    MarkTarget,
    MortalWeakness,
    MortalWeaknessStress,
    NoxiousDebuff(u8),
    OccVulnerabilityCurse(u8),
    OccWeakenProt(u8),
    OccWeakeningCurse(u8),
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
    Rakebuff(u8),
    ShadowBlood(u8),
    SlamDebuff(u8),
    SniperDamage(u8),
    SoloMarkSelf,
    Stealth,
    StealthSelf,
    StrongBleed(u8),
    Stun(u8),
    StunKiller(u8),
    Suppression(u8),
    SwitchModeBeastSelf,
    SwitchModeHumanSelf,
    TrackingBuff(u8),
    TransformHealSelf(u8),
    UnholyKiller(u8),
    VestalHealSelf(u8),
    VestalInspiration(u8),
    VestalLight(u8),
    VestalStun(u8),
    VomitDebuff(u8),
    WyrdBleed(u8),
    XformDamage(u8),
}

impl Effect {
    const fn application_kind(&self) -> ApplicationKind {
        self.data().application_kind
    }

    const fn apply_on_death(&self) -> bool {
        self.data().apply_on_death
    }

    const fn buffs(&self) -> &'static [Buff] {
        self.data().buffs
    }

    const fn chance(&self) -> u8 {
        self.data().chance
    }

    const fn combat_stat_buff(&self) -> &'static [CombatStatBuff] {
        self.data().combat_stat_buffs
    }

    const fn condition(&self) -> Option<Condition> {
        self.data().condition
    }

    const fn data(&self) -> EffectData {
        effect_data(self)
    }

    const fn duration(&self) -> Option<Duration> {
        self.data().duration
    }

    const fn on_miss(&self) -> bool {
        self.data().on_miss
    }

    const fn status_effects(&self) -> &'static [StatusEffect] {
        self.data().status_effects
    }

    const fn swap_source_and_target(&self) -> bool {
        self.data().swap_source_and_target
    }

    const fn target(&self) -> Target {
        self.data().target
    }
}
