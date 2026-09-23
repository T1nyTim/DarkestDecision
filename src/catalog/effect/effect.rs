use crate::catalog::{
    effect::{ApplicationKind, Buff, CombatStatBuff, Condition, Duration, MonsterType, Status, StatusEffect, Target},
    heroes::Mode,
};

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
    HumanStressHealParty(u8),
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
        match self {
            Self::AbyssalKiller(_)
            | Self::Disorient(_)
            | Self::EldritchKiller(_)
            | Self::FlareClear
            | Self::GrAccBuff(_)
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
            | Self::OnCritStressResist
            | Self::Rakebuff(_)
            | Self::XformDamage(_) => ApplicationKind::ApplyOnce,
            Self::AbsolutionHealStress(_)
            | Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::ArbMarkDebuff(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastStressParty
            | Self::BellowCrit(_)
            | Self::BhMarkDebuff(_)
            | Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::Command(_)
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkMark
            | Self::CrusaderHealStress(_)
            | Self::Defender(_)
            | Self::FortifyResists(_)
            | Self::GrDodge(_)
            | Self::GrapeshotVulnerability(_)
            | Self::HeroStrongStun(_)
            | Self::HoundHowl(_)
            | Self::HoundProtect(_)
            | Self::HumanStressHealParty(_)
            | Self::InspiringTune(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperAcc
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::ManaclesStun(_)
            | Self::MarkSelf
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
            | Self::OccWeakenProt(_)
            | Self::PdDisorientingStun(_)
            | Self::ShadowBlood(_)
            | Self::Stun(_)
            | Self::TransformHealSelf(_)
            | Self::VestalHealSelf(_)
            | Self::VestalStun(_)
            | Self::WyrdBleed(_) => ApplicationKind::Queue,
            Self::AntiqSelfSpeed(_)
            | Self::ArbSelfSpeed(_)
            | Self::BhSelfSpeed(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::CrusaderBulwarkLight
            | Self::CrusaderLight(_)
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::FlareHealStress(_)
            | Self::FlareLight(_)
            | Self::GrSelfSpeed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::JesterSpotlight(_)
            | Self::LeperVulnerability
            | Self::SoloMarkSelf
            | Self::VestalLight(_) => ApplicationKind::QueueOnce,
            Self::AbomVomit(_)
            | Self::AbsolutionHeal(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::AntiqProtectMeGuard
            | Self::ArbMarkTarget
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BhDmgMarked
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::Bleed(_)
            | Self::BoloPush1(_)
            | Self::CaltropsPreyDebuff(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::ClearCorpses
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::Cure
            | Self::Destealth
            | Self::Disrupt(_)
            | Self::DodgeCurse(_)
            | Self::EmboldenTeam(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlight(_)
            | Self::GrBlightDebuff(_)
            | Self::GrDaggerDmgMarked(_)
            | Self::GrFadeAttack(_)
            | Self::HarryBleed(_)
            | Self::HellionHealSelf(_)
            | Self::HmDmgMarked(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundDebuff(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::HwPistolDmgMarked(_)
            | Self::HwyRiposte(_)
            | Self::LeperHealSelf(_)
            | Self::LeperIntimidate(_)
            | Self::LickWounds(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::ManKiller(_)
            | Self::MarkTarget
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::PdBlight(_)
            | Self::PdSingleBlight(_)
            | Self::PdVapoursBuff(_)
            | Self::PoisonKiller(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::SlamDebuff(_)
            | Self::SniperDamage(_)
            | Self::Stealth
            | Self::StealthSelf
            | Self::StrongBleed(_)
            | Self::StunKiller(_)
            | Self::Suppression(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TrackingBuff(_)
            | Self::UnholyKiller(_)
            | Self::VestalInspiration(_)
            | Self::VomitDebuff(_) => ApplicationKind::Stack,
        }
    }

    const fn apply_on_death(&self) -> bool {
        if matches!(
            self,
            Self::AbomVomit(_)
                | Self::AntiqBlight(_)
                | Self::Bleed(_)
                | Self::BoloPush1(_)
                | Self::ClearCorpses
                | Self::Disorient(_)
                | Self::GrBlight(_)
                | Self::HarryBleed(_)
                | Self::HoundBleed(_)
                | Self::PdBlight(_)
                | Self::PdSingleBlight(_)
                | Self::Pull2(_)
                | Self::Push1(_)
                | Self::Push2(_)
                | Self::Push3(_)
                | Self::StrongBleed(_)
                | Self::WyrdBleed(_)
        ) {
            true
        } else {
            false
        }
    }

    const fn buffs(&self) -> &'static [Buff] {
        match self {
            Self::LeperAcc => &[Buff::Acc(5)],
            Self::AntiqBlightBuff(5..) => &[Buff::AntiqBlightBuff(5)],
            Self::AntiqBlightBuff(4) => &[Buff::AntiqBlightBuff(4)],
            Self::AntiqBlightBuff(3) => &[Buff::AntiqBlightBuff(3)],
            Self::AntiqBlightBuff(2) => &[Buff::AntiqBlightBuff(2)],
            Self::AntiqBlightBuff(0..=1) => &[Buff::AntiqBlightBuff(1)],
            Self::AntiqBlightDebuff(5..) => &[Buff::AntiqBlightDebuff(5)],
            Self::AntiqBlightDebuff(4) => &[Buff::AntiqBlightDebuff(4)],
            Self::AntiqBlightDebuff(3) => &[Buff::AntiqBlightDebuff(3)],
            Self::AntiqBlightDebuff(2) => &[Buff::AntiqBlightDebuff(2)],
            Self::AntiqBlightDebuff(0..=1) => &[Buff::AntiqBlightDebuff(1)],
            Self::BeastBuff2(5..) => &[Buff::BeastBlightBuff(5)],
            Self::BeastBuff2(4) => &[Buff::BeastBlightBuff(4)],
            Self::BeastBuff2(3) => &[Buff::BeastBlightBuff(3)],
            Self::BeastBuff2(2) => &[Buff::BeastBlightBuff(2)],
            Self::BeastBuff2(0..=1) => &[Buff::BeastBlightBuff(1)],
            Self::GrBleedDebuff(5..) | Self::HwOpenVeinBleedDebuff(5..) => &[Buff::BleedDebuff(5)],
            Self::GrBleedDebuff(4) | Self::HwOpenVeinBleedDebuff(4) => &[Buff::BleedDebuff(4)],
            Self::GrBleedDebuff(3) | Self::HwOpenVeinBleedDebuff(3) => &[Buff::BleedDebuff(3)],
            Self::GrBleedDebuff(2) | Self::HwOpenVeinBleedDebuff(2) => &[Buff::BleedDebuff(2)],
            Self::GrBleedDebuff(0..=1) | Self::HwOpenVeinBleedDebuff(0..=1) => &[Buff::BleedDebuff(1)],
            Self::FortifyResists(5..) => &[Buff::BleedResist(15), Buff::BlightResist(15)],
            Self::FortifyResists(4) => &[Buff::BleedResist(14), Buff::BlightResist(14)],
            Self::FortifyResists(3) => &[Buff::BleedResist(12), Buff::BlightResist(12)],
            Self::FortifyResists(2) => &[Buff::BleedResist(11), Buff::BlightResist(11)],
            Self::FortifyResists(0..=1) => &[Buff::BleedResist(10), Buff::BlightResist(10)],
            Self::GrBlightDebuff(5..) => &[Buff::BlightDebuff(5)],
            Self::GrBlightDebuff(4) => &[Buff::BlightDebuff(4)],
            Self::GrBlightDebuff(3) => &[Buff::BlightDebuff(3)],
            Self::GrBlightDebuff(2) => &[Buff::BlightDebuff(2)],
            Self::GrBlightDebuff(0..=1) => &[Buff::BlightDebuff(1)],
            Self::BuildToFinale(2..) => &[Buff::BuildToFinaleDmgH(2), Buff::BuildToFinaleDmgL(2)],
            Self::BuildToFinale(0..=1) | Self::BuildToFinaleSong => &[Buff::BuildToFinaleDmgH(1), Buff::BuildToFinaleDmgL(1)],
            Self::CaltropsPreyDebuff(5..) => &[Buff::CaltropsDmgReceived(5)],
            Self::CaltropsPreyDebuff(4) => &[Buff::CaltropsDmgReceived(4)],
            Self::CaltropsPreyDebuff(3) => &[Buff::CaltropsDmgReceived(3)],
            Self::CaltropsPreyDebuff(2) => &[Buff::CaltropsDmgReceived(2)],
            Self::CaltropsPreyDebuff(0..=1) => &[Buff::CaltropsDmgReceived(1)],
            Self::HellionExhaust => &[Buff::HellionExDmgH, Buff::HellionExDmgL, Buff::HellionExSpd],
            Self::HellionExhaustSm => &[Buff::HellionExDmgHSm, Buff::HellionExDmgLSm, Buff::HellionExSpdSm],
            Self::JesterTuneBuff(5..) => &[Buff::JestTuneStressResistance(5)],
            Self::JesterTuneBuff(4) => &[Buff::JestTuneStressResistance(4)],
            Self::JesterTuneBuff(3) => &[Buff::JestTuneStressResistance(3)],
            Self::JesterTuneBuff(2) => &[Buff::JestTuneStressResistance(2)],
            Self::JesterTuneBuff(0..=1) => &[Buff::JestTuneStressResistance(1)],
            Self::LeperResistBuff => &[
                Buff::LeperBleedResist,
                Buff::LeperBlightResist,
                Buff::LeperDebuffResist,
                Buff::LeperMoveResist,
            ],
            Self::LeperVulnerability => &[Buff::LeperDefVuln, Buff::LeperDmgVuln],
            Self::BellowCrit(_) => &[Buff::MaaBellowCritReceived],
            Self::Command(5..) => &[
                Buff::MaaCommandAcc(5),
                Buff::MaaCommandCrit(5),
                Buff::MaaCommandGuardedDmgH(5),
                Buff::MaaCommandGuardedDmgL(5),
            ],
            Self::Command(4) => &[
                Buff::MaaCommandAcc(4),
                Buff::MaaCommandCrit(4),
                Buff::MaaCommandGuardedDmgH(4),
                Buff::MaaCommandGuardedDmgL(4),
            ],
            Self::Command(3) => &[
                Buff::MaaCommandAcc(3),
                Buff::MaaCommandCrit(3),
                Buff::MaaCommandGuardedDmgH(3),
                Buff::MaaCommandGuardedDmgL(3),
            ],
            Self::Command(2) => &[
                Buff::MaaCommandAcc(2),
                Buff::MaaCommandCrit(2),
                Buff::MaaCommandGuardedDmgH(2),
                Buff::MaaCommandGuardedDmgL(2),
            ],
            Self::Command(0..=1) => &[
                Buff::MaaCommandAcc(1),
                Buff::MaaCommandCrit(1),
                Buff::MaaCommandGuardedDmgH(1),
                Buff::MaaCommandGuardedDmgL(1),
            ],
            Self::MortalWeaknessStress => &[Buff::MortalWeaknessStress],
            Self::OnCritAcc => &[Buff::OnCritAcc],
            Self::OnCritBleedChance => &[Buff::OnCritBleedChance],
            Self::OnCritBlightChance => &[Buff::OnCritBlightChance],
            Self::OnCritDef => &[Buff::OnCritDef],
            Self::OnCritDmg => &[Buff::OnCritDmgH, Buff::OnCritDmgL],
            Self::OnCritDmgBleeding => &[Buff::OnCritDmgHBleeding, Buff::OnCritDmgLBleeding],
            Self::OnCritDmgMarked => &[Buff::OnCritDmgHMarked, Buff::OnCritDmgLMarked],
            Self::OnCritHealDone => &[Buff::OnCritHealBuff],
            Self::OnCritProt => &[Buff::OnCritProt],
            Self::OnCritSpeed => &[Buff::OnCritSpd],
            Self::OnCritStressHealDone => &[Buff::OnCritStressHealBuff],
            Self::OnCritStressResist => &[Buff::OnCritStressResist],
            Self::Rakebuff(5..) => &[Buff::RakeBuffH(5), Buff::RakeBuffL(5)],
            Self::Rakebuff(4) => &[Buff::RakeBuffH(4), Buff::RakeBuffL(4)],
            Self::Rakebuff(3) => &[Buff::RakeBuffH(3), Buff::RakeBuffL(3)],
            Self::Rakebuff(2) => &[Buff::RakeBuffH(2), Buff::RakeBuffL(2)],
            Self::Rakebuff(0..=1) => &[Buff::RakeBuffH(1), Buff::RakeBuffL(1)],
            Self::HwOpenVeinSpdDebuff(5..) => &[Buff::Spd(-3)],
            Self::HwOpenVeinSpdDebuff(3..=4) => &[Buff::Spd(-2)],
            Self::HwOpenVeinSpdDebuff(0..=2) => &[Buff::Spd(-1)],
            Self::BolsterStressBuff(5..) => &[Buff::StressDmg(-20)],
            Self::BolsterStressBuff(3..=4) => &[Buff::StressDmg(-15)],
            Self::BolsterStressBuff(0..=2) => &[Buff::StressDmg(-10)],
            Self::VomitDebuff(5..) => &[Buff::VomitBlightResistDebuff(5)],
            Self::VomitDebuff(4) => &[Buff::VomitBlightResistDebuff(4)],
            Self::VomitDebuff(3) => &[Buff::VomitBlightResistDebuff(3)],
            Self::VomitDebuff(2) => &[Buff::VomitBlightResistDebuff(2)],
            Self::VomitDebuff(0..=1) => &[Buff::VomitBlightResistDebuff(1)],
            Self::AbomVomit(_)
            | Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
            | Self::AbyssalKiller(_)
            | Self::AbyssalStun(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::AntiqProtectMeGuard
            | Self::AntiqSelfSpeed(_)
            | Self::ArbMarkDebuff(_)
            | Self::ArbMarkTarget
            | Self::ArbSelfSpeed(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastBuff(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BeastStressParty
            | Self::BhDmgMarked
            | Self::BhMarkDebuff(_)
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::BhSelfSpeed(_)
            | Self::Bleed(_)
            | Self::BoloPush1(_)
            | Self::Bolster(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::ClearCorpses
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkLight
            | Self::CrusaderBulwarkMark
            | Self::CrusaderHealStress(_)
            | Self::CrusaderLight(_)
            | Self::Cure
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::Defender(_)
            | Self::Destealth
            | Self::Disorient(_)
            | Self::Disrupt(_)
            | Self::DodgeCurse(_)
            | Self::EldritchKiller(_)
            | Self::EmboldenTeam(_)
            | Self::FlareClear
            | Self::FlareHealStress(_)
            | Self::FlareLight(_)
            | Self::GrAccBuff(_)
            | Self::GrBlight(_)
            | Self::GrDaggerDmgMarked(_)
            | Self::GrDodge(_)
            | Self::GrFadeAttack(_)
            | Self::GrSelfSpeed(_)
            | Self::GrapeshotVulnerability(_)
            | Self::HarryBleed(_)
            | Self::HellionHealSelf(_)
            | Self::HeroStrongStun(_)
            | Self::HmDmgMarked(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundDebuff(_)
            | Self::HoundHowl(_)
            | Self::HoundProtect(_)
            | Self::HumanStressHealParty(_)
            | Self::HwPistolDmgMarked(_)
            | Self::HwyRiposte(_)
            | Self::InspiringTune(_)
            | Self::JesterSpotlight(_)
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LeperIntimidate(_)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperStrength(_)
            | Self::LickWounds(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::ManKiller(_)
            | Self::ManaclesStun(_)
            | Self::MarkSelf
            | Self::MarkTarget
            | Self::MortalWeakness
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::OccWeakenProt(_)
            | Self::PdBlight(_)
            | Self::PdDisorientingStun(_)
            | Self::PdSingleBlight(_)
            | Self::PdVapoursBuff(_)
            | Self::PoisonKiller(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::ShadowBlood(_)
            | Self::SlamDebuff(_)
            | Self::SniperDamage(_)
            | Self::SoloMarkSelf
            | Self::Stealth
            | Self::StealthSelf
            | Self::StrongBleed(_)
            | Self::Stun(_)
            | Self::StunKiller(_)
            | Self::Suppression(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TrackingBuff(_)
            | Self::TransformHealSelf(_)
            | Self::UnholyKiller(_)
            | Self::VestalHealSelf(_)
            | Self::VestalInspiration(_)
            | Self::VestalLight(_)
            | Self::VestalStun(_)
            | Self::WyrdBleed(_)
            | Self::XformDamage(_) => &[],
        }
    }

    const fn chance(&self) -> u8 {
        match self {
            Self::HoundDebuff(5..) => 170,
            Self::HoundDebuff(4) => 160,
            Self::AbyssalStun(5..) | Self::HarryBleed(5..) | Self::HeroStrongStun(5..) | Self::HoundDebuff(3) => 150,
            Self::AbomVomit(5..)
            | Self::AbyssalStun(4)
            | Self::AntiqBlight(5..)
            | Self::AntiqBlightDebuff(5..)
            | Self::AntiqDistract(5..)
            | Self::ArbMarkDebuff(5..)
            | Self::BellowCrit(5..)
            | Self::BhMarkDebuff(5..)
            | Self::Bleed(5..)
            | Self::CaltropsPreyDebuff(5..)
            | Self::CaltropsSpdDebuff(5..)
            | Self::Disorient(5..)
            | Self::Disrupt(5..)
            | Self::DodgeCurse(5..)
            | Self::GrapeshotVulnerability(5..)
            | Self::GrBleedDebuff(5..)
            | Self::GrBlight(5..)
            | Self::GrBlightDebuff(5..)
            | Self::HarryBleed(4)
            | Self::HeroStrongStun(4)
            | Self::HoundBleed(5..)
            | Self::HoundDebuff(2)
            | Self::HwOpenVeinBleedDebuff(5..)
            | Self::HwOpenVeinSpdDebuff(5..)
            | Self::LeperIntimidate(5..)
            | Self::NoxiousDebuff(5..)
            | Self::OccVulnerabilityCurse(5..)
            | Self::OccWeakeningCurse(5..)
            | Self::OccWeakenProt(5..)
            | Self::PdBlight(5..)
            | Self::PdDisorientingStun(5..)
            | Self::PdSingleBlight(5..)
            | Self::Pull2(5..)
            | Self::Push1(5..)
            | Self::Push2(5..)
            | Self::Push3(5..)
            | Self::SlamDebuff(5..)
            | Self::StrongBleed(5..)
            | Self::Stun(5..)
            | Self::Suppression(5..)
            | Self::VestalStun(5..)
            | Self::VomitDebuff(5..) => 140,
            Self::AbomVomit(4)
            | Self::AbyssalStun(3)
            | Self::AntiqBlight(4)
            | Self::AntiqBlightDebuff(4)
            | Self::AntiqDistract(4)
            | Self::ArbMarkDebuff(4)
            | Self::BellowCrit(4)
            | Self::BhMarkDebuff(4)
            | Self::Bleed(4)
            | Self::CaltropsPreyDebuff(4)
            | Self::CaltropsSpdDebuff(4)
            | Self::Disorient(4)
            | Self::Disrupt(4)
            | Self::DodgeCurse(4)
            | Self::GrapeshotVulnerability(4)
            | Self::GrBleedDebuff(4)
            | Self::GrBlight(4)
            | Self::GrBlightDebuff(4)
            | Self::HarryBleed(3)
            | Self::HeroStrongStun(3)
            | Self::HoundBleed(4)
            | Self::HoundDebuff(0..=1)
            | Self::HwOpenVeinBleedDebuff(4)
            | Self::HwOpenVeinSpdDebuff(4)
            | Self::LeperIntimidate(4)
            | Self::ManaclesStun(5..)
            | Self::NoxiousDebuff(4)
            | Self::OccVulnerabilityCurse(4)
            | Self::OccWeakeningCurse(4)
            | Self::OccWeakenProt(4)
            | Self::PdBlight(4)
            | Self::PdDisorientingStun(4)
            | Self::PdSingleBlight(4)
            | Self::Pull2(4)
            | Self::Push1(4)
            | Self::Push2(4)
            | Self::Push3(4)
            | Self::SlamDebuff(4)
            | Self::StrongBleed(4)
            | Self::Stun(4)
            | Self::Suppression(4)
            | Self::VestalStun(4)
            | Self::VomitDebuff(4) => 130,
            Self::AbomVomit(3)
            | Self::AbyssalStun(2)
            | Self::AntiqBlight(3)
            | Self::AntiqBlightDebuff(3)
            | Self::AntiqDistract(3)
            | Self::ArbMarkDebuff(3)
            | Self::BellowCrit(3)
            | Self::BhMarkDebuff(3)
            | Self::Bleed(3)
            | Self::CaltropsPreyDebuff(3)
            | Self::CaltropsSpdDebuff(3)
            | Self::Disorient(3)
            | Self::Disrupt(3)
            | Self::DodgeCurse(3)
            | Self::GrapeshotVulnerability(3)
            | Self::GrBleedDebuff(3)
            | Self::GrBlight(3)
            | Self::GrBlightDebuff(3)
            | Self::HarryBleed(2)
            | Self::HeroStrongStun(2)
            | Self::HoundBleed(3)
            | Self::HwOpenVeinBleedDebuff(3)
            | Self::HwOpenVeinSpdDebuff(3)
            | Self::LeperIntimidate(3)
            | Self::ManaclesStun(4)
            | Self::NoxiousDebuff(3)
            | Self::OccVulnerabilityCurse(3)
            | Self::OccWeakeningCurse(3)
            | Self::OccWeakenProt(3)
            | Self::PdBlight(3)
            | Self::PdDisorientingStun(3)
            | Self::PdSingleBlight(3)
            | Self::Pull2(3)
            | Self::Push1(3)
            | Self::Push2(3)
            | Self::Push3(3)
            | Self::SlamDebuff(3)
            | Self::StrongBleed(3)
            | Self::Stun(3)
            | Self::Suppression(3)
            | Self::VestalStun(3)
            | Self::VomitDebuff(3) => 120,
            Self::BoloPush1(5..) => 115,
            Self::AbomVomit(2)
            | Self::AbyssalStun(0..=1)
            | Self::AntiqBlight(2)
            | Self::AntiqBlightDebuff(2)
            | Self::AntiqDistract(2)
            | Self::ArbMarkDebuff(2)
            | Self::BellowCrit(2)
            | Self::BhMarkDebuff(2)
            | Self::Bleed(2)
            | Self::CaltropsPreyDebuff(2)
            | Self::CaltropsSpdDebuff(2)
            | Self::Disorient(2)
            | Self::Disrupt(2)
            | Self::DodgeCurse(2)
            | Self::GrapeshotVulnerability(2)
            | Self::GrBleedDebuff(2)
            | Self::GrBlight(2)
            | Self::GrBlightDebuff(2)
            | Self::HarryBleed(0..=1)
            | Self::HeroStrongStun(0..=1)
            | Self::HoundBleed(2)
            | Self::HwOpenVeinBleedDebuff(2)
            | Self::HwOpenVeinSpdDebuff(2)
            | Self::LeperIntimidate(2)
            | Self::ManaclesStun(3)
            | Self::NoxiousDebuff(2)
            | Self::OccVulnerabilityCurse(2)
            | Self::OccWeakeningCurse(2)
            | Self::OccWeakenProt(2)
            | Self::PdBlight(2)
            | Self::PdDisorientingStun(2)
            | Self::PdSingleBlight(2)
            | Self::Pull2(2)
            | Self::Push1(2)
            | Self::Push2(2)
            | Self::Push3(2)
            | Self::SlamDebuff(2)
            | Self::StrongBleed(2)
            | Self::Stun(2)
            | Self::Suppression(2)
            | Self::VestalStun(2)
            | Self::VomitDebuff(2) => 110,
            Self::BoloPush1(4) => 105,
            Self::AbomVomit(0..=1)
            | Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
            | Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlight(0..=1)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(0..=1)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(0..=1)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::AntiqProtectMeGuard
            | Self::AntiqSelfSpeed(_)
            | Self::ArbMarkDebuff(0..=1)
            | Self::ArbMarkTarget
            | Self::ArbSelfSpeed(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BeastStressParty
            | Self::BellowCrit(0..=1)
            | Self::BhDmgMarked
            | Self::BhMarkDebuff(0..=1)
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::BhSelfSpeed(_)
            | Self::Bleed(0..=1)
            | Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::CaltropsPreyDebuff(0..=1)
            | Self::CaltropsSpdDebuff(0..=1)
            | Self::ClearCorpses
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::Command(_)
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkLight
            | Self::CrusaderBulwarkMark
            | Self::CrusaderHealStress(_)
            | Self::CrusaderLight(_)
            | Self::Cure
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::Defender(_)
            | Self::Destealth
            | Self::Disorient(0..=1)
            | Self::Disrupt(0..=1)
            | Self::DodgeCurse(0..=1)
            | Self::EldritchKiller(_)
            | Self::EmboldenTeam(_)
            | Self::FlareClear
            | Self::FlareLight(_)
            | Self::FortifyResists(_)
            | Self::GrAccBuff(_)
            | Self::GrapeshotVulnerability(0..=1)
            | Self::GrBleedDebuff(0..=1)
            | Self::GrBlight(0..=1)
            | Self::GrBlightDebuff(0..=1)
            | Self::GrDaggerDmgMarked(_)
            | Self::GrDodge(_)
            | Self::GrFadeAttack(_)
            | Self::GrSelfSpeed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HellionHealSelf(_)
            | Self::HmDmgMarked(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(0..=1)
            | Self::HoundProtect(_)
            | Self::HumanStressHealParty(_)
            | Self::HwOpenVeinBleedDebuff(0..=1)
            | Self::HwOpenVeinSpdDebuff(0..=1)
            | Self::HwPistolDmgMarked(_)
            | Self::HwyRiposte(_)
            | Self::InspiringTune(_)
            | Self::JesterSpotlight(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperAcc
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LeperIntimidate(0..=1)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::LeperVulnerability
            | Self::LickWounds(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::ManKiller(_)
            | Self::ManaclesStun(2)
            | Self::MarkSelf
            | Self::MarkTarget
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
            | Self::NoxiousDebuff(0..=1)
            | Self::OccVulnerabilityCurse(0..=1)
            | Self::OccWeakeningCurse(0..=1)
            | Self::OccWeakenProt(0..=1)
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
            | Self::OnCritStressResist
            | Self::PdBlight(0..=1)
            | Self::PdDisorientingStun(0..=1)
            | Self::PdSingleBlight(0..=1)
            | Self::PdVapoursBuff(_)
            | Self::PoisonKiller(_)
            | Self::Pull2(0..=1)
            | Self::Push1(0..=1)
            | Self::Push2(0..=1)
            | Self::Push3(0..=1)
            | Self::Rakebuff(_)
            | Self::ShadowBlood(_)
            | Self::SlamDebuff(0..=1)
            | Self::SniperDamage(_)
            | Self::SoloMarkSelf
            | Self::Stealth
            | Self::StealthSelf
            | Self::StrongBleed(0..=1)
            | Self::Stun(0..=1)
            | Self::StunKiller(_)
            | Self::Suppression(0..=1)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TrackingBuff(_)
            | Self::TransformHealSelf(_)
            | Self::UnholyKiller(_)
            | Self::VestalHealSelf(_)
            | Self::VestalInspiration(_)
            | Self::VestalLight(_)
            | Self::VestalStun(0..=1)
            | Self::VomitDebuff(0..=1)
            | Self::XformDamage(_) => 100,
            Self::BoloPush1(3) => 95,
            Self::ManaclesStun(0..=1) => 90,
            Self::BoloPush1(2) | Self::WyrdBleed(5..) => 85,
            Self::WyrdBleed(4) => 80,
            Self::BoloPush1(0..=1) => 75,
            Self::HoundHowl(5..) => 74,
            Self::HoundHowl(4) => 72,
            Self::HoundHowl(3) | Self::WyrdBleed(2..=3) => 70,
            Self::HoundHowl(2) => 68,
            Self::FlareHealStress(3 | 5..) => 67,
            Self::HoundHowl(0..=1) => 66,
            Self::FlareHealStress(0..=2 | 4) | Self::WyrdBleed(0..=1) => 60,
        }
    }

    const fn combat_stat_buff(&self) -> &'static [CombatStatBuff] {
        match self {
            Self::LeperStrength(5..) => &[
                CombatStatBuff::AttackRatingAdd(15),
                CombatStatBuff::CritChanceAdd(11),
                CombatStatBuff::DamageHighMultiply(35),
                CombatStatBuff::DamageLowMultiply(35),
            ],
            Self::LeperStrength(4) => &[
                CombatStatBuff::AttackRatingAdd(13),
                CombatStatBuff::CritChanceAdd(10),
                CombatStatBuff::DamageHighMultiply(32),
                CombatStatBuff::DamageLowMultiply(32),
            ],
            Self::LeperStrength(3) => &[
                CombatStatBuff::AttackRatingAdd(12),
                CombatStatBuff::CritChanceAdd(9),
                CombatStatBuff::DamageHighMultiply(30),
                CombatStatBuff::DamageLowMultiply(30),
            ],
            Self::LeperStrength(2) => &[
                CombatStatBuff::AttackRatingAdd(11),
                CombatStatBuff::CritChanceAdd(8),
                CombatStatBuff::DamageHighMultiply(27),
                CombatStatBuff::DamageLowMultiply(27),
            ],
            Self::LeperStrength(0..=1) => &[
                CombatStatBuff::AttackRatingAdd(10),
                CombatStatBuff::CritChanceAdd(7),
                CombatStatBuff::DamageHighMultiply(25),
                CombatStatBuff::DamageLowMultiply(25),
            ],
            Self::GrAccBuff(5..) => &[CombatStatBuff::AttackRatingAdd(10)],
            Self::TrackingBuff(5..) => &[
                CombatStatBuff::AttackRatingAdd(10),
                CombatStatBuff::CritChanceAdd(8),
                CombatStatBuff::DamageHighMultiply(20),
                CombatStatBuff::DamageLowMultiply(20),
            ],
            Self::EmboldenTeam(5..) => &[
                CombatStatBuff::AttackRatingAdd(10),
                CombatStatBuff::CritChanceAdd(6),
                CombatStatBuff::SpeedRatingAdd(4),
            ],
            Self::VestalInspiration(5..) => &[
                CombatStatBuff::AttackRatingAdd(10),
                CombatStatBuff::DamageHighMultiply(35),
                CombatStatBuff::DamageLowMultiply(35),
            ],
            Self::Adrenaline(5..) => &[
                CombatStatBuff::AttackRatingAdd(10),
                CombatStatBuff::DamageHighMultiply(30),
                CombatStatBuff::DamageLowMultiply(30),
            ],
            Self::TrackingBuff(4) => &[
                CombatStatBuff::AttackRatingAdd(9),
                CombatStatBuff::CritChanceAdd(7),
                CombatStatBuff::DamageHighMultiply(18),
                CombatStatBuff::DamageLowMultiply(18),
            ],
            Self::VestalInspiration(4) => &[
                CombatStatBuff::AttackRatingAdd(9),
                CombatStatBuff::DamageHighMultiply(33),
                CombatStatBuff::DamageLowMultiply(33),
            ],
            Self::GrAccBuff(4) => &[CombatStatBuff::AttackRatingAdd(8)],
            Self::TrackingBuff(3) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::CritChanceAdd(6),
                CombatStatBuff::DamageHighMultiply(16),
                CombatStatBuff::DamageLowMultiply(16),
            ],
            Self::EmboldenTeam(4) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::CritChanceAdd(5),
                CombatStatBuff::SpeedRatingAdd(3),
            ],
            Self::VestalInspiration(3) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::DamageHighMultiply(30),
                CombatStatBuff::DamageLowMultiply(30),
            ],
            Self::Adrenaline(4) => &[
                CombatStatBuff::AttackRatingAdd(8),
                CombatStatBuff::DamageHighMultiply(26),
                CombatStatBuff::DamageLowMultiply(26),
            ],
            Self::GrAccBuff(3) => &[CombatStatBuff::AttackRatingAdd(7)],
            Self::TrackingBuff(2) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::CritChanceAdd(5),
                CombatStatBuff::DamageHighMultiply(14),
                CombatStatBuff::DamageLowMultiply(14),
            ],
            Self::EmboldenTeam(3) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::CritChanceAdd(4),
                CombatStatBuff::SpeedRatingAdd(3),
            ],
            Self::VestalInspiration(2) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::DamageHighMultiply(27),
                CombatStatBuff::DamageLowMultiply(27),
            ],
            Self::Adrenaline(3) => &[
                CombatStatBuff::AttackRatingAdd(7),
                CombatStatBuff::DamageHighMultiply(24),
                CombatStatBuff::DamageLowMultiply(24),
            ],
            Self::GrAccBuff(2) => &[CombatStatBuff::AttackRatingAdd(6)],
            Self::TrackingBuff(0..=1) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::CritChanceAdd(4),
                CombatStatBuff::DamageHighMultiply(12),
                CombatStatBuff::DamageLowMultiply(12),
            ],
            Self::EmboldenTeam(2) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::CritChanceAdd(3),
                CombatStatBuff::SpeedRatingAdd(2),
            ],
            Self::VestalInspiration(0..=1) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::DamageHighMultiply(25),
                CombatStatBuff::DamageLowMultiply(25),
            ],
            Self::Adrenaline(2) => &[
                CombatStatBuff::AttackRatingAdd(6),
                CombatStatBuff::DamageHighMultiply(22),
                CombatStatBuff::DamageLowMultiply(22),
            ],
            Self::GrAccBuff(0..=1) => &[CombatStatBuff::AttackRatingAdd(5)],
            Self::EmboldenTeam(0..=1) => &[
                CombatStatBuff::AttackRatingAdd(5),
                CombatStatBuff::CritChanceAdd(2),
                CombatStatBuff::SpeedRatingAdd(2),
            ],
            Self::Adrenaline(0..=1) => &[
                CombatStatBuff::AttackRatingAdd(5),
                CombatStatBuff::DamageHighMultiply(20),
                CombatStatBuff::DamageLowMultiply(20),
            ],
            Self::NoxiousDebuff(0..=2) => &[CombatStatBuff::AttackRatingAdd(-5)],
            Self::NoxiousDebuff(3..=4) => &[CombatStatBuff::AttackRatingAdd(-6)],
            Self::NoxiousDebuff(5..) => &[CombatStatBuff::AttackRatingAdd(-7)],
            Self::AntiqDistract(0..=1) => &[CombatStatBuff::AttackRatingAdd(-10)],
            Self::AntiqDistract(2) => &[CombatStatBuff::AttackRatingAdd(-11)],
            Self::AntiqDistract(3) => &[CombatStatBuff::AttackRatingAdd(-12)],
            Self::AntiqDistract(4) => &[CombatStatBuff::AttackRatingAdd(-14)],
            Self::AntiqDistract(5..) => &[CombatStatBuff::AttackRatingAdd(-15)],
            Self::Suppression(0..=1) => &[CombatStatBuff::AttackRatingAdd(-15), CombatStatBuff::CritChanceAdd(-15)],
            Self::Suppression(2) => &[CombatStatBuff::AttackRatingAdd(-16), CombatStatBuff::CritChanceAdd(-16)],
            Self::Suppression(3) => &[CombatStatBuff::AttackRatingAdd(-17), CombatStatBuff::CritChanceAdd(-17)],
            Self::Suppression(4) => &[CombatStatBuff::AttackRatingAdd(-18), CombatStatBuff::CritChanceAdd(-18)],
            Self::Suppression(5..) => &[CombatStatBuff::AttackRatingAdd(-20), CombatStatBuff::CritChanceAdd(-19)],
            Self::SniperDamage(5..) => &[
                CombatStatBuff::CritChanceAdd(13),
                CombatStatBuff::DamageHighMultiply(100),
                CombatStatBuff::DamageLowMultiply(100),
            ],
            Self::SniperDamage(4) => &[
                CombatStatBuff::CritChanceAdd(12),
                CombatStatBuff::DamageHighMultiply(80),
                CombatStatBuff::DamageLowMultiply(80),
            ],
            Self::SniperDamage(3) => &[
                CombatStatBuff::CritChanceAdd(11),
                CombatStatBuff::DamageHighMultiply(70),
                CombatStatBuff::DamageLowMultiply(70),
            ],
            Self::SniperDamage(2) => &[
                CombatStatBuff::CritChanceAdd(10),
                CombatStatBuff::DamageHighMultiply(60),
                CombatStatBuff::DamageLowMultiply(60),
            ],
            Self::SniperDamage(0..=1) => &[
                CombatStatBuff::CritChanceAdd(9),
                CombatStatBuff::DamageHighMultiply(50),
                CombatStatBuff::DamageLowMultiply(50),
            ],
            Self::GrFadeAttack(5..) => &[
                CombatStatBuff::CritChanceAdd(8),
                CombatStatBuff::DamageHighMultiply(100),
                CombatStatBuff::DamageLowMultiply(100),
            ],
            Self::GrFadeAttack(4) => &[
                CombatStatBuff::CritChanceAdd(7),
                CombatStatBuff::DamageHighMultiply(95),
                CombatStatBuff::DamageLowMultiply(95),
            ],
            Self::GrFadeAttack(3) => &[
                CombatStatBuff::CritChanceAdd(6),
                CombatStatBuff::DamageHighMultiply(90),
                CombatStatBuff::DamageLowMultiply(90),
            ],
            Self::GrFadeAttack(2) => &[
                CombatStatBuff::CritChanceAdd(5),
                CombatStatBuff::DamageHighMultiply(85),
                CombatStatBuff::DamageLowMultiply(85),
            ],
            Self::GrFadeAttack(0..=1) => &[
                CombatStatBuff::CritChanceAdd(4),
                CombatStatBuff::DamageHighMultiply(80),
                CombatStatBuff::DamageLowMultiply(80),
            ],
            Self::GrapeshotVulnerability(5..) => &[CombatStatBuff::CritReceivedChance(8)],
            Self::GrapeshotVulnerability(4) => &[CombatStatBuff::CritReceivedChance(7)],
            Self::GrapeshotVulnerability(3) => &[CombatStatBuff::CritReceivedChance(6)],
            Self::GrapeshotVulnerability(2) => &[CombatStatBuff::CritReceivedChance(5)],
            Self::GrapeshotVulnerability(0..=1) => &[CombatStatBuff::CritReceivedChance(4)],
            Self::HmDmgMarked(5..) => &[CombatStatBuff::DamageHighMultiply(100), CombatStatBuff::DamageLowMultiply(100)],
            Self::BhDmgMarked | Self::HmDmgMarked(4) => &[CombatStatBuff::DamageHighMultiply(90), CombatStatBuff::DamageLowMultiply(90)],
            Self::HmDmgMarked(3) => &[CombatStatBuff::DamageHighMultiply(80), CombatStatBuff::DamageLowMultiply(80)],
            Self::HmDmgMarked(2) => &[CombatStatBuff::DamageHighMultiply(70), CombatStatBuff::DamageLowMultiply(70)],
            Self::HmDmgMarked(0..=1) | Self::StunKiller(5..) => &[CombatStatBuff::DamageHighMultiply(60), CombatStatBuff::DamageLowMultiply(60)],
            Self::HwPistolDmgMarked(5..) | Self::StunKiller(4) => &[CombatStatBuff::DamageHighMultiply(50), CombatStatBuff::DamageLowMultiply(50)],
            Self::GrDaggerDmgMarked(5..) | Self::HwPistolDmgMarked(4) | Self::StunKiller(3) => {
                &[CombatStatBuff::DamageHighMultiply(40), CombatStatBuff::DamageLowMultiply(40)]
            }
            Self::GrDaggerDmgMarked(4) => &[CombatStatBuff::DamageHighMultiply(36), CombatStatBuff::DamageLowMultiply(36)],
            Self::BeastKiller(5..) | Self::EldritchKiller(5..) | Self::HwPistolDmgMarked(3) | Self::ManKiller(5..) | Self::UnholyKiller(5..) => {
                &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)]
            }
            Self::PoisonKiller(5..) | Self::StunKiller(2) => &[CombatStatBuff::DamageHighMultiply(33), CombatStatBuff::DamageLowMultiply(33)],
            Self::GrDaggerDmgMarked(3) => &[CombatStatBuff::DamageHighMultiply(32), CombatStatBuff::DamageLowMultiply(32)],
            Self::BeastKiller(4) | Self::EldritchKiller(4) | Self::HwPistolDmgMarked(2) | Self::ManKiller(4) | Self::UnholyKiller(4) => {
                &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)]
            }
            Self::PoisonKiller(4) => &[CombatStatBuff::DamageHighMultiply(29), CombatStatBuff::DamageLowMultiply(29)],
            Self::GrDaggerDmgMarked(2) => &[CombatStatBuff::DamageHighMultiply(28), CombatStatBuff::DamageLowMultiply(28)],
            Self::PoisonKiller(3) => &[CombatStatBuff::DamageHighMultiply(26), CombatStatBuff::DamageLowMultiply(26)],
            Self::AbyssalKiller(5..)
            | Self::BeastKiller(3)
            | Self::EldritchKiller(3)
            | Self::GrDaggerDmgMarked(0..=1)
            | Self::HwPistolDmgMarked(0..=1)
            | Self::ManKiller(3)
            | Self::StunKiller(0..=1)
            | Self::UnholyKiller(3)
            | Self::XformDamage(5..) => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
            Self::PdVapoursBuff(5..) => &[
                CombatStatBuff::DamageHighMultiply(25),
                CombatStatBuff::DamageLowMultiply(25),
                CombatStatBuff::SpeedRatingAdd(5),
            ],
            Self::PoisonKiller(2) => &[CombatStatBuff::DamageHighMultiply(23), CombatStatBuff::DamageLowMultiply(23)],
            Self::PdVapoursBuff(4) => &[
                CombatStatBuff::DamageHighMultiply(23),
                CombatStatBuff::DamageLowMultiply(23),
                CombatStatBuff::SpeedRatingAdd(4),
            ],
            Self::AbyssalKiller(4) => &[CombatStatBuff::DamageHighMultiply(22), CombatStatBuff::DamageLowMultiply(22)],
            Self::PdVapoursBuff(3) => &[
                CombatStatBuff::DamageHighMultiply(22),
                CombatStatBuff::DamageLowMultiply(22),
                CombatStatBuff::SpeedRatingAdd(4),
            ],
            Self::XformDamage(4) => &[CombatStatBuff::DamageHighMultiply(21), CombatStatBuff::DamageLowMultiply(21)],
            Self::PdVapoursBuff(2) => &[
                CombatStatBuff::DamageHighMultiply(21),
                CombatStatBuff::DamageLowMultiply(21),
                CombatStatBuff::SpeedRatingAdd(3),
            ],
            Self::AbyssalKiller(3)
            | Self::BeastKiller(2)
            | Self::EldritchKiller(2)
            | Self::ManKiller(2)
            | Self::PoisonKiller(0..=1)
            | Self::UnholyKiller(2) => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
            Self::PdVapoursBuff(0..=1) => &[
                CombatStatBuff::DamageHighMultiply(20),
                CombatStatBuff::DamageLowMultiply(20),
                CombatStatBuff::SpeedRatingAdd(3),
            ],
            Self::AbyssalKiller(2) | Self::XformDamage(3) => &[CombatStatBuff::DamageHighMultiply(17), CombatStatBuff::DamageLowMultiply(17)],
            Self::AbyssalKiller(0..=1)
            | Self::BeastKiller(0..=1)
            | Self::EldritchKiller(0..=1)
            | Self::ManKiller(0..=1)
            | Self::UnholyKiller(0..=1) => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            Self::XformDamage(2) => &[CombatStatBuff::DamageHighMultiply(14), CombatStatBuff::DamageLowMultiply(14)],
            Self::XformDamage(0..=1) => &[CombatStatBuff::DamageHighMultiply(10), CombatStatBuff::DamageLowMultiply(10)],
            Self::OccWeakeningCurse(0..=1) => &[CombatStatBuff::DamageHighMultiply(-10), CombatStatBuff::DamageLowMultiply(-10)],
            Self::OccWeakeningCurse(2) => &[CombatStatBuff::DamageHighMultiply(-12), CombatStatBuff::DamageLowMultiply(-12)],
            Self::OccWeakeningCurse(3) => &[CombatStatBuff::DamageHighMultiply(-15), CombatStatBuff::DamageLowMultiply(-15)],
            Self::OccWeakeningCurse(4) => &[CombatStatBuff::DamageHighMultiply(-17), CombatStatBuff::DamageLowMultiply(-17)],
            Self::OccWeakeningCurse(5..) => &[CombatStatBuff::DamageHighMultiply(-20), CombatStatBuff::DamageLowMultiply(-20)],
            Self::LeperIntimidate(0..=1) => &[
                CombatStatBuff::DamageHighMultiply(-20),
                CombatStatBuff::DamageLowMultiply(-20),
                CombatStatBuff::SpeedRatingAdd(-3),
            ],
            Self::LeperIntimidate(2) => &[
                CombatStatBuff::DamageHighMultiply(-23),
                CombatStatBuff::DamageLowMultiply(-23),
                CombatStatBuff::SpeedRatingAdd(-3),
            ],
            Self::LeperIntimidate(3) => &[
                CombatStatBuff::DamageHighMultiply(-26),
                CombatStatBuff::DamageLowMultiply(-26),
                CombatStatBuff::SpeedRatingAdd(-4),
            ],
            Self::LeperIntimidate(4) => &[
                CombatStatBuff::DamageHighMultiply(-29),
                CombatStatBuff::DamageLowMultiply(-29),
                CombatStatBuff::SpeedRatingAdd(-4),
            ],
            Self::LeperIntimidate(5..) => &[
                CombatStatBuff::DamageHighMultiply(-33),
                CombatStatBuff::DamageLowMultiply(-33),
                CombatStatBuff::SpeedRatingAdd(-5),
            ],
            Self::JesterSpotlight(5..) => &[CombatStatBuff::DefenseRatingAdd(30)],
            Self::JesterSpotlight(4) => &[CombatStatBuff::DefenseRatingAdd(27)],
            Self::AntiqCower(5..) | Self::JesterSpotlight(3) => &[CombatStatBuff::DefenseRatingAdd(25)],
            Self::AntiqCower(4) | Self::JesterSpotlight(2) => &[CombatStatBuff::DefenseRatingAdd(22)],
            Self::AntiqCower(3) | Self::HoundProtect(5..) | Self::JesterSpotlight(0..=1) => &[CombatStatBuff::DefenseRatingAdd(20)],
            Self::AntiqCower(2) => &[CombatStatBuff::DefenseRatingAdd(18)],
            Self::HoundProtect(4) => &[CombatStatBuff::DefenseRatingAdd(17)],
            Self::AntiqCower(0..=1) | Self::GrDodge(5..) => &[CombatStatBuff::DefenseRatingAdd(15)],
            Self::HoundProtect(3) => &[CombatStatBuff::DefenseRatingAdd(14)],
            Self::GrDodge(4) | Self::ShadowBlood(5..) => &[CombatStatBuff::DefenseRatingAdd(13)],
            Self::GrDodge(3) | Self::HoundProtect(2) | Self::ShadowBlood(4) => &[CombatStatBuff::DefenseRatingAdd(12)],
            Self::GrDodge(2) | Self::ShadowBlood(3) => &[CombatStatBuff::DefenseRatingAdd(11)],
            Self::AntiqDodge(5..) | Self::Bolster(5..) | Self::GrDodge(0..=1) | Self::HoundProtect(0..=1) | Self::ShadowBlood(2) => {
                &[CombatStatBuff::DefenseRatingAdd(10)]
            }
            Self::AntiqDodge(4) | Self::ShadowBlood(0..=1) => &[CombatStatBuff::DefenseRatingAdd(9)],
            Self::Bolster(4) => &[CombatStatBuff::DefenseRatingAdd(8)],
            Self::AntiqDefBuff(5..) => &[CombatStatBuff::DefenseRatingAdd(8), CombatStatBuff::ProtectionRatingAdd(200)],
            Self::AntiqDodge(3) | Self::Bolster(3) => &[CombatStatBuff::DefenseRatingAdd(7)],
            Self::AntiqDefBuff(4) => &[CombatStatBuff::DefenseRatingAdd(7), CombatStatBuff::ProtectionRatingAdd(180)],
            Self::Bolster(2) => &[CombatStatBuff::DefenseRatingAdd(6)],
            Self::AntiqDefBuff(3) => &[CombatStatBuff::DefenseRatingAdd(6), CombatStatBuff::ProtectionRatingAdd(150)],
            Self::AntiqDodge(2) | Self::Bolster(0..=1) => &[CombatStatBuff::DefenseRatingAdd(5)],
            Self::AntiqDefBuff(2) => &[CombatStatBuff::DefenseRatingAdd(5), CombatStatBuff::ProtectionRatingAdd(130)],
            Self::AntiqDefBuff(0..=1) => &[CombatStatBuff::DefenseRatingAdd(4), CombatStatBuff::ProtectionRatingAdd(100)],
            Self::AntiqDodge(0..=1) => &[CombatStatBuff::DefenseRatingAdd(3)],
            Self::Disrupt(0..=1) => &[CombatStatBuff::DefenseRatingAdd(-5), CombatStatBuff::SpeedRatingAdd(-5)],
            Self::Disrupt(2) => &[CombatStatBuff::DefenseRatingAdd(-6), CombatStatBuff::SpeedRatingAdd(-5)],
            Self::Disrupt(3) => &[CombatStatBuff::DefenseRatingAdd(-7), CombatStatBuff::SpeedRatingAdd(-6)],
            Self::Disrupt(4) => &[CombatStatBuff::DefenseRatingAdd(-8), CombatStatBuff::SpeedRatingAdd(-6)],
            Self::Disrupt(5..) => &[CombatStatBuff::DefenseRatingAdd(-10), CombatStatBuff::SpeedRatingAdd(-7)],
            Self::SlamDebuff(0..=1) => &[CombatStatBuff::DefenseRatingAdd(-10), CombatStatBuff::SpeedRatingAdd(-2)],
            Self::SlamDebuff(2) => &[CombatStatBuff::DefenseRatingAdd(-12), CombatStatBuff::SpeedRatingAdd(-3)],
            Self::OccVulnerabilityCurse(0..=1) => &[CombatStatBuff::DefenseRatingAdd(-15)],
            Self::SlamDebuff(3) => &[CombatStatBuff::DefenseRatingAdd(-15), CombatStatBuff::SpeedRatingAdd(-4)],
            Self::OccVulnerabilityCurse(2) => &[CombatStatBuff::DefenseRatingAdd(-16)],
            Self::OccVulnerabilityCurse(3) => &[CombatStatBuff::DefenseRatingAdd(-17)],
            Self::SlamDebuff(4) => &[CombatStatBuff::DefenseRatingAdd(-17), CombatStatBuff::SpeedRatingAdd(-5)],
            Self::OccVulnerabilityCurse(4) => &[CombatStatBuff::DefenseRatingAdd(-18)],
            Self::ArbMarkDebuff(0..=1) | Self::DodgeCurse(0..=1) | Self::OccVulnerabilityCurse(5..) => &[CombatStatBuff::DefenseRatingAdd(-20)],
            Self::SlamDebuff(5..) => &[CombatStatBuff::DefenseRatingAdd(-20), CombatStatBuff::SpeedRatingAdd(-6)],
            Self::ArbMarkDebuff(2) => &[CombatStatBuff::DefenseRatingAdd(-22)],
            Self::DodgeCurse(2) => &[CombatStatBuff::DefenseRatingAdd(-23)],
            Self::ArbMarkDebuff(3) | Self::DodgeCurse(3) => &[CombatStatBuff::DefenseRatingAdd(-25)],
            Self::MortalWeakness => &[CombatStatBuff::DefenseRatingAdd(-25), CombatStatBuff::SpeedRatingAdd(-3)],
            Self::ArbMarkDebuff(4) | Self::DodgeCurse(4) => &[CombatStatBuff::DefenseRatingAdd(-27)],
            Self::ArbMarkDebuff(5..) | Self::DodgeCurse(5..) => &[CombatStatBuff::DefenseRatingAdd(-30)],
            Self::ArbStackingHeal(5..) => &[CombatStatBuff::HpHealReceivedPercent(38)],
            Self::ArbStackingHeal(4) => &[CombatStatBuff::HpHealReceivedPercent(33)],
            Self::ArbStackingHeal(3) => &[CombatStatBuff::HpHealReceivedPercent(28)],
            Self::ArbStackingHeal(2) => &[CombatStatBuff::HpHealReceivedPercent(24)],
            Self::ArbStackingHeal(0..=1) => &[CombatStatBuff::HpHealReceivedPercent(20)],
            Self::CrusaderBulwark(5..) | Self::Defender(5..) | Self::LeperProtect(5..) => &[CombatStatBuff::ProtectionRatingAdd(300)],
            Self::CrusaderBulwark(4) => &[CombatStatBuff::ProtectionRatingAdd(280)],
            Self::LeperProtect(4) => &[CombatStatBuff::ProtectionRatingAdd(270)],
            Self::Defender(4) => &[CombatStatBuff::ProtectionRatingAdd(260)],
            Self::CrusaderBulwark(3) | Self::LeperProtect(3) => &[CombatStatBuff::ProtectionRatingAdd(250)],
            Self::CrusaderBulwark(2) | Self::Defender(3) | Self::LeperProtect(2) => &[CombatStatBuff::ProtectionRatingAdd(220)],
            Self::CrusaderBulwark(0..=1) | Self::LeperProtect(0..=1) => &[CombatStatBuff::ProtectionRatingAdd(200)],
            Self::Defender(2) => &[CombatStatBuff::ProtectionRatingAdd(180)],
            Self::Defender(0..=1) => &[CombatStatBuff::ProtectionRatingAdd(150)],
            Self::BhMarkDebuff(0..=1) | Self::OccWeakenProt(0..=1) => &[CombatStatBuff::ProtectionRatingAdd(-100)],
            Self::BhMarkDebuff(2) => &[CombatStatBuff::ProtectionRatingAdd(-125)],
            Self::OccWeakenProt(2) => &[CombatStatBuff::ProtectionRatingAdd(-130)],
            Self::BhMarkDebuff(3) | Self::OccWeakenProt(3) => &[CombatStatBuff::ProtectionRatingAdd(-150)],
            Self::OccWeakenProt(4) => &[CombatStatBuff::ProtectionRatingAdd(-170)],
            Self::BhMarkDebuff(4) => &[CombatStatBuff::ProtectionRatingAdd(-175)],
            Self::BhMarkDebuff(5..) | Self::HoundDebuff(0..=1) | Self::OccWeakenProt(5..) => &[CombatStatBuff::ProtectionRatingAdd(-200)],
            Self::HoundDebuff(2) => &[CombatStatBuff::ProtectionRatingAdd(-220)],
            Self::HoundDebuff(3) => &[CombatStatBuff::ProtectionRatingAdd(-250)],
            Self::HoundDebuff(4) => &[CombatStatBuff::ProtectionRatingAdd(-270)],
            Self::HoundDebuff(5..) => &[CombatStatBuff::ProtectionRatingAdd(-300)],
            Self::AntiqSelfSpeed(5..) | Self::ArbSelfSpeed(5..) | Self::BeastBuff(5..) | Self::BhSelfSpeed(5..) => {
                &[CombatStatBuff::SpeedRatingAdd(5)]
            }
            Self::AntiqSelfSpeed(4)
            | Self::ArbSelfSpeed(3..=4)
            | Self::BeastBuff(4)
            | Self::BhSelfSpeed(3..=4)
            | Self::GrSelfSpeed(5..)
            | Self::LeperHype(5..) => &[CombatStatBuff::SpeedRatingAdd(4)],
            Self::AntiqSelfSpeed(3)
            | Self::ArbSelfSpeed(0..=2)
            | Self::BeastBuff(3)
            | Self::BhSelfSpeed(0..=2)
            | Self::GrSelfSpeed(3..=4)
            | Self::LeperHype(3..=4) => &[CombatStatBuff::SpeedRatingAdd(3)],
            Self::AntiqSelfSpeed(2) | Self::BeastBuff(2) | Self::GrSelfSpeed(0..=2) | Self::LeperHype(0..=2) => &[CombatStatBuff::SpeedRatingAdd(2)],
            Self::AntiqSelfSpeed(0..=1) | Self::BeastBuff(0..=1) => &[CombatStatBuff::SpeedRatingAdd(1)],
            Self::BeastDebuff(4) => &[CombatStatBuff::SpeedRatingAdd(-1)],
            Self::BeastDebuff(3) => &[CombatStatBuff::SpeedRatingAdd(-2)],
            Self::BeastDebuff(2) => &[CombatStatBuff::SpeedRatingAdd(-3)],
            Self::BeastDebuff(0..=1) | Self::CaltropsSpdDebuff(0..=1) => &[CombatStatBuff::SpeedRatingAdd(-4)],
            Self::CaltropsSpdDebuff(2) => &[CombatStatBuff::SpeedRatingAdd(-5)],
            Self::CaltropsSpdDebuff(3) => &[CombatStatBuff::SpeedRatingAdd(-6)],
            Self::CaltropsSpdDebuff(4) => &[CombatStatBuff::SpeedRatingAdd(-7)],
            Self::CaltropsSpdDebuff(5..) => &[CombatStatBuff::SpeedRatingAdd(-8)],
            Self::AbomVomit(_)
            | Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
            | Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::AntiqProtectMeGuard
            | Self::ArbMarkTarget
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastStressParty
            | Self::BellowCrit(_)
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::Bleed(_)
            | Self::BoloPush1(_)
            | Self::BolsterStressBuff(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::CaltropsPreyDebuff(_)
            | Self::ClearCorpses
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::Command(_)
            | Self::CrusaderBulwarkLight
            | Self::CrusaderBulwarkMark
            | Self::CrusaderHealStress(_)
            | Self::CrusaderLight(_)
            | Self::Cure
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::Destealth
            | Self::Disorient(_)
            | Self::FlareClear
            | Self::FlareHealStress(_)
            | Self::FlareLight(_)
            | Self::FortifyResists(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlight(_)
            | Self::GrBlightDebuff(_)
            | Self::HarryBleed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HellionHealSelf(_)
            | Self::HeroStrongStun(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundHowl(_)
            | Self::HumanStressHealParty(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::HwyRiposte(_)
            | Self::InspiringTune(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperAcc
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperResistBuff
            | Self::LeperVulnerability
            | Self::LickWounds(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::ManaclesStun(_)
            | Self::MarkSelf
            | Self::MarkTarget
            | Self::MortalWeaknessStress
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
            | Self::OnCritStressResist
            | Self::PdBlight(_)
            | Self::PdDisorientingStun(_)
            | Self::PdSingleBlight(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::Rakebuff(_)
            | Self::SoloMarkSelf
            | Self::Stealth
            | Self::StealthSelf
            | Self::StrongBleed(_)
            | Self::Stun(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TransformHealSelf(_)
            | Self::VestalHealSelf(_)
            | Self::VestalLight(_)
            | Self::VestalStun(_)
            | Self::VomitDebuff(_)
            | Self::WyrdBleed(_) => &[],
        }
    }

    const fn condition(&self) -> Option<Condition> {
        match self {
            Self::BeastKiller(_) => Some(Condition::MonsterType(MonsterType::Beast)),
            Self::ClearCorpses => Some(Condition::MonsterType(MonsterType::Corpse)),
            Self::AbyssalKiller(_) | Self::EldritchKiller(_) => Some(Condition::MonsterType(MonsterType::Eldritch)),
            Self::ManKiller(_) => Some(Condition::MonsterType(MonsterType::Man)),
            Self::UnholyKiller(_) => Some(Condition::MonsterType(MonsterType::Unholy)),
            Self::PoisonKiller(_) => Some(Condition::TargetStatus(Status::Poisoned)),
            Self::StunKiller(_) => Some(Condition::TargetStatus(Status::Stunned)),
            Self::BhDmgMarked | Self::GrDaggerDmgMarked(_) | Self::HmDmgMarked(_) | Self::HwPistolDmgMarked(_) | Self::SniperDamage(_) => {
                Some(Condition::TargetStatus(Status::Tagged))
            }
            Self::AbomVomit(_)
            | Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
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
            | Self::AntiqProtectMeGuard
            | Self::AntiqSelfSpeed(_)
            | Self::ArbMarkDebuff(_)
            | Self::ArbMarkTarget
            | Self::ArbSelfSpeed(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastStressParty
            | Self::BellowCrit(_)
            | Self::BhMarkDebuff(_)
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::BhSelfSpeed(_)
            | Self::Bleed(_)
            | Self::BoloPush1(_)
            | Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::CaltropsPreyDebuff(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::Command(_)
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkLight
            | Self::CrusaderBulwarkMark
            | Self::CrusaderHealStress(_)
            | Self::CrusaderLight(_)
            | Self::Cure
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::Defender(_)
            | Self::Destealth
            | Self::Disorient(_)
            | Self::Disrupt(_)
            | Self::DodgeCurse(_)
            | Self::EmboldenTeam(_)
            | Self::FlareClear
            | Self::FlareHealStress(_)
            | Self::FlareLight(_)
            | Self::FortifyResists(_)
            | Self::GrAccBuff(_)
            | Self::GrapeshotVulnerability(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlight(_)
            | Self::GrBlightDebuff(_)
            | Self::GrDodge(_)
            | Self::GrFadeAttack(_)
            | Self::GrSelfSpeed(_)
            | Self::HarryBleed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HellionHealSelf(_)
            | Self::HeroStrongStun(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundDebuff(_)
            | Self::HoundHowl(_)
            | Self::HoundProtect(_)
            | Self::HumanStressHealParty(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::HwyRiposte(_)
            | Self::InspiringTune(_)
            | Self::JesterSpotlight(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperAcc
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LeperIntimidate(_)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::LeperVulnerability
            | Self::LickWounds(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::ManaclesStun(_)
            | Self::MarkSelf
            | Self::MarkTarget
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::OccWeakenProt(_)
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
            | Self::OnCritStressResist
            | Self::PdBlight(_)
            | Self::PdDisorientingStun(_)
            | Self::PdSingleBlight(_)
            | Self::PdVapoursBuff(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::Rakebuff(_)
            | Self::ShadowBlood(_)
            | Self::SlamDebuff(_)
            | Self::SoloMarkSelf
            | Self::Stealth
            | Self::StealthSelf
            | Self::StrongBleed(_)
            | Self::Stun(_)
            | Self::Suppression(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TrackingBuff(_)
            | Self::TransformHealSelf(_)
            | Self::VestalHealSelf(_)
            | Self::VestalInspiration(_)
            | Self::VestalLight(_)
            | Self::VestalStun(_)
            | Self::VomitDebuff(_)
            | Self::WyrdBleed(_)
            | Self::XformDamage(_) => None,
        }
    }

    const fn duration(&self) -> Option<Duration> {
        match self {
            Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkMark
            | Self::GrSelfSpeed(_)
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::LeperVulnerability
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
            | Self::PdVapoursBuff(_)
            | Self::ShadowBlood(_)
            | Self::TrackingBuff(_) => Some(Duration::Combat),
            Self::BuildToFinale(_) | Self::BuildToFinaleSong => Some(Duration::Rounds(8)),
            Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqSelfSpeed(_)
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::Defender(_)
            | Self::GrBlight(_)
            | Self::GrDodge(_)
            | Self::JesterSpotlight(_)
            | Self::LeperAcc
            | Self::LeperIntimidateMark
            | Self::Rakebuff(_) => Some(Duration::Rounds(4)),
            Self::AbomVomit(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqDodge(_)
            | Self::ArbMarkTarget
            | Self::BellowCrit(_)
            | Self::BhMarkDebuff(_)
            | Self::BhMarkTarget
            | Self::Bleed(_)
            | Self::CaltropsPreyDebuff(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::Disrupt(_)
            | Self::FortifyResists(_)
            | Self::GrapeshotVulnerability(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlightDebuff(_)
            | Self::HarryBleed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundProtect(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::HwyRiposte(_)
            | Self::LeperIntimidate(_)
            | Self::MaaGuard
            | Self::MaaRiposte(_)
            | Self::MarkTarget
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::OccWeakenProt(_)
            | Self::OnCritDmg
            | Self::OnCritDmgBleeding
            | Self::OnCritDmgMarked
            | Self::OnCritStressResist
            | Self::PdBlight(_)
            | Self::PdSingleBlight(_)
            | Self::SlamDebuff(_)
            | Self::SniperDamage(_)
            | Self::SoloMarkSelf
            | Self::StrongBleed(_)
            | Self::WyrdBleed(_)
            | Self::XformDamage(_) => Some(Duration::Rounds(3)),
            Self::AntiqDistract(_)
            | Self::AntiqProtectMeGuard
            | Self::ArbMarkDebuff(_)
            | Self::BhMinorMark
            | Self::BhSelfSpeed(_)
            | Self::GrFadeAttack(_)
            | Self::HmGuard
            | Self::MarkSelf
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritHealDone
            | Self::OnCritStressHealDone
            | Self::StealthSelf
            | Self::Suppression(_) => Some(Duration::Rounds(2)),
            Self::AbyssalStun(_)
            | Self::HeroStrongStun(_)
            | Self::ManaclesStun(_)
            | Self::PdDisorientingStun(_)
            | Self::Stealth
            | Self::Stun(_)
            | Self::VestalStun(_) => Some(Duration::Rounds(1)),
            Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
            | Self::AbyssalKiller(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::ArbSelfSpeed(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BeastStressParty
            | Self::BhDmgMarked
            | Self::BoloPush1(_)
            | Self::ClearCorpses
            | Self::ClearGuardPerformer
            | Self::ClearGuardTarget
            | Self::Command(_)
            | Self::CrusaderBulwarkLight
            | Self::CrusaderHealStress(_)
            | Self::CrusaderLight(_)
            | Self::Cure
            | Self::CureSelf
            | Self::Darkness
            | Self::DazzlingLight
            | Self::Destealth
            | Self::Disorient(_)
            | Self::DodgeCurse(_)
            | Self::EldritchKiller(_)
            | Self::EmboldenTeam(_)
            | Self::FlareClear
            | Self::FlareHealStress(_)
            | Self::FlareLight(_)
            | Self::GrAccBuff(_)
            | Self::GrDaggerDmgMarked(_)
            | Self::HellionHealSelf(_)
            | Self::HmDmgMarked(_)
            | Self::HoundDebuff(_)
            | Self::HoundHowl(_)
            | Self::HumanStressHealParty(_)
            | Self::HwPistolDmgMarked(_)
            | Self::InspiringTune(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LickWounds(_)
            | Self::ManKiller(_)
            | Self::OnCritAcc
            | Self::OnCritDef
            | Self::OnCritProt
            | Self::OnCritSpeed
            | Self::PoisonKiller(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::StunKiller(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TransformHealSelf(_)
            | Self::UnholyKiller(_)
            | Self::VestalHealSelf(_)
            | Self::VestalInspiration(_)
            | Self::VestalLight(_)
            | Self::VomitDebuff(_) => None,
        }
    }

    const fn on_miss(&self) -> bool {
        if matches!(
            self,
            Self::Adrenaline(_)
                | Self::AntiqBlightBuff(_)
                | Self::AntiqCower(_)
                | Self::AntiqDefBuff(_)
                | Self::AntiqDodge(_)
                | Self::AntiqProtectMeClearGuardsPerformer
                | Self::AntiqProtectMeClearGuardsTarget
                | Self::AntiqProtectMeGuard
                | Self::AntiqSelfSpeed(_)
                | Self::ArbSelfSpeed(_)
                | Self::ArbStackingHeal(_)
                | Self::Bolster(_)
                | Self::BolsterStressBuff(_)
                | Self::ClearGuardPerformer
                | Self::ClearGuardTarget
                | Self::Command(_)
                | Self::CrusaderBulwark(_)
                | Self::CrusaderBulwarkLight
                | Self::CrusaderBulwarkMark
                | Self::CrusaderLight(_)
                | Self::DazzlingLight
                | Self::Defender(_)
                | Self::EmboldenTeam(_)
                | Self::FlareClear
                | Self::FlareHealStress(_)
                | Self::FlareLight(_)
                | Self::FortifyResists(_)
                | Self::GrDodge(_)
                | Self::HellionExhaust
                | Self::HellionExhaustSm
                | Self::HmGuard
                | Self::HwyRiposte(_)
                | Self::JesterSpotlight(_)
                | Self::LeperAcc
                | Self::LeperHype(_)
                | Self::LeperMarkSelf
                | Self::LeperProtect(_)
                | Self::LeperStrength(_)
                | Self::LeperVulnerability
                | Self::MaaGuard
                | Self::MaaRiposte(_)
                | Self::MarkSelf
                | Self::MortalWeakness
                | Self::MortalWeaknessStress
                | Self::Rakebuff(_)
                | Self::ShadowBlood(_)
                | Self::SoloMarkSelf
                | Self::TrackingBuff(_)
                | Self::VestalInspiration(_)
                | Self::VestalLight(_)
        ) {
            true
        } else {
            false
        }
    }

    const fn status_effect(&self) -> &'static [StatusEffect] {
        match self {
            Self::AntiqProtectMeClearGuardsPerformer | Self::AntiqProtectMeClearGuardsTarget => {
                &[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding]
            }
            Self::ClearGuardPerformer | Self::ClearGuardTarget => &[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding],
            Self::Adrenaline(_) | Self::Cure | Self::CureSelf | Self::ShadowBlood(_) => &[StatusEffect::Cure],
            Self::StrongBleed(5..) => &[StatusEffect::DotBleed(5)],
            Self::Bleed(5..) | Self::StrongBleed(3..=4) => &[StatusEffect::DotBleed(4)],
            Self::Bleed(3..=4) | Self::HarryBleed(5..) | Self::StrongBleed(0..=2) | Self::WyrdBleed(5..) => &[StatusEffect::DotBleed(3)],
            Self::Bleed(0..=2) | Self::HarryBleed(3..=4) | Self::HoundBleed(4..) | Self::WyrdBleed(3..=4) => &[StatusEffect::DotBleed(2)],
            Self::HarryBleed(0..=2) | Self::HoundBleed(0..=3) | Self::WyrdBleed(0..=2) => &[StatusEffect::DotBleed(1)],
            Self::PdSingleBlight(5..) => &[StatusEffect::DotPoison(7)],
            Self::PdBlight(5..) | Self::PdSingleBlight(3..=4) => &[StatusEffect::DotPoison(6)],
            Self::AbomVomit(5..) | Self::PdBlight(3..=4) | Self::PdSingleBlight(0..=2) => &[StatusEffect::DotPoison(5)],
            Self::AbomVomit(4) | Self::AntiqBlight(5..) | Self::GrBlight(5..) | Self::PdBlight(0..=2) => &[StatusEffect::DotPoison(4)],
            Self::AbomVomit(2..=3) | Self::AntiqBlight(4) | Self::GrBlight(2..=4) => &[StatusEffect::DotPoison(3)],
            Self::AbomVomit(0..=1) | Self::AntiqBlight(2..=3) | Self::GrBlight(0..=1) => &[StatusEffect::DotPoison(2)],
            Self::AntiqBlight(0..=1) => &[StatusEffect::DotPoison(1)],
            Self::AntiqProtectMeGuard | Self::HmGuard | Self::MaaGuard => &[StatusEffect::Guard],
            Self::LeperHealSelf(5..) => &[StatusEffect::Heal { amount: 10, is_skill: true }],
            Self::TransformHealSelf(5..) => &[StatusEffect::Heal { amount: 10, is_skill: false }],
            Self::LeperHealSelf(4) => &[StatusEffect::Heal { amount: 9, is_skill: true }],
            Self::LeperHealSelf(3) | Self::LickWounds(5..) | Self::TransformHealSelf(4) => &[StatusEffect::Heal { amount: 8, is_skill: true }],
            Self::LeperHealSelf(2) | Self::LickWounds(4) | Self::TransformHealSelf(3) => &[StatusEffect::Heal { amount: 7, is_skill: true }],
            Self::LeperHealSelf(0..=1) | Self::LickWounds(3) | Self::TransformHealSelf(2) => &[StatusEffect::Heal { amount: 6, is_skill: true }],
            Self::AbsolutionHeal(5..) | Self::LickWounds(2) | Self::TransformHealSelf(0..=1) | Self::VestalHealSelf(3..) => {
                &[StatusEffect::Heal { amount: 5, is_skill: true }]
            }
            Self::AbsolutionHeal(3..=4) | Self::HellionHealSelf(5..) | Self::LickWounds(0..=1) | Self::VestalHealSelf(2) => {
                &[StatusEffect::Heal { amount: 4, is_skill: true }]
            }
            Self::AbsolutionHeal(0..=2) | Self::HellionHealSelf(4) | Self::VestalHealSelf(0..=1) => {
                &[StatusEffect::Heal { amount: 3, is_skill: true }]
            }
            Self::HellionHealSelf(2..=3) => &[StatusEffect::Heal { amount: 2, is_skill: true }],
            Self::HellionHealSelf(0..=1) => &[StatusEffect::Heal { amount: 1, is_skill: true }],
            Self::InspiringTune(5..) => &[StatusEffect::HealStress(12)],
            Self::InspiringTune(4) => &[StatusEffect::HealStress(11)],
            Self::AbsolutionHealStress(5..) | Self::InspiringTune(3) => &[StatusEffect::HealStress(10)],
            Self::AbsolutionHealStress(4) | Self::InspiringTune(2) => &[StatusEffect::HealStress(9)],
            Self::AbsolutionHealStress(2..=3) | Self::CrusaderHealStress(5..) | Self::InspiringTune(0..=1) => &[StatusEffect::HealStress(8)],
            Self::AbsolutionHealStress(0..=1) | Self::CrusaderHealStress(4) | Self::LeperHealSelfStress(5..) => &[StatusEffect::HealStress(7)],
            Self::CrusaderHealStress(3) | Self::HoundHowl(5..) | Self::LeperHealSelfStress(3..=4) => &[StatusEffect::HealStress(6)],
            Self::CrusaderHealStress(0..=2) | Self::HoundHowl(4) | Self::LeperHealSelfStress(0..=2) => &[StatusEffect::HealStress(5)],
            Self::HoundHowl(3) => &[StatusEffect::HealStress(4)],
            Self::FlareHealStress(4..) | Self::HoundHowl(2) => &[StatusEffect::HealStress(3)],
            Self::FlareHealStress(2..=3) | Self::HoundHowl(0..=1) | Self::HumanStressHealParty(_) => &[StatusEffect::HealStress(2)],
            Self::FlareHealStress(0..=1) => &[StatusEffect::HealStress(1)],
            Self::ClearCorpses => &[StatusEffect::Kill],
            Self::Pull2(_) => &[StatusEffect::Pull(2)],
            Self::BoloPush1(_) | Self::Push1(_) => &[StatusEffect::Push(1)],
            Self::Push2(_) => &[StatusEffect::Push(2)],
            Self::Push3(_) => &[StatusEffect::Push(3)],
            Self::HwyRiposte(5..) => &[StatusEffect::Riposte { damage: -15, crit: 5 }],
            Self::HwyRiposte(4) | Self::MaaRiposte(5..) => &[StatusEffect::Riposte { damage: -20, crit: 4 }],
            Self::HwyRiposte(3) | Self::MaaRiposte(4) => &[StatusEffect::Riposte { damage: -25, crit: 3 }],
            Self::MaaRiposte(3) => &[StatusEffect::Riposte { damage: -30, crit: 2 }],
            Self::HwyRiposte(2) => &[StatusEffect::Riposte { damage: -33, crit: 2 }],
            Self::MaaRiposte(2) => &[StatusEffect::Riposte { damage: -35, crit: 1 }],
            Self::HwyRiposte(0..=1) | Self::MaaRiposte(0..=1) => &[StatusEffect::Riposte { damage: -40, crit: 0 }],
            Self::SwitchModeBeastSelf => &[StatusEffect::SetMode(Mode::Beast)],
            Self::SwitchModeHumanSelf => &[StatusEffect::SetMode(Mode::Human)],
            Self::Disorient(_) => &[StatusEffect::Shuffle],
            Self::Stealth | Self::StealthSelf => &[StatusEffect::Stealth],
            Self::BeastStressParty => &[StatusEffect::Stress(8)],
            Self::AbyssalStun(_)
            | Self::HeroStrongStun(_)
            | Self::ManaclesStun(_)
            | Self::PdDisorientingStun(_)
            | Self::Stun(_)
            | Self::VestalStun(_) => &[StatusEffect::Stun],
            Self::ArbMarkTarget
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::CrusaderBulwarkMark
            | Self::HmMarkTarget
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::MarkSelf
            | Self::MarkTarget
            | Self::SoloMarkSelf => &[StatusEffect::Tag],
            Self::CrusaderBulwarkLight => &[StatusEffect::Torch(24)],
            Self::CrusaderLight(5..) | Self::VestalLight(5..) => &[StatusEffect::Torch(10)],
            Self::CrusaderLight(4) | Self::VestalLight(4) => &[StatusEffect::Torch(8)],
            Self::CrusaderLight(3) | Self::FlareLight(5..) | Self::VestalLight(3) => &[StatusEffect::Torch(7)],
            Self::CrusaderLight(2) | Self::DazzlingLight | Self::FlareLight(4) | Self::VestalLight(2) => &[StatusEffect::Torch(6)],
            Self::CrusaderLight(0..=1) | Self::FlareLight(3) | Self::VestalLight(0..=1) => &[StatusEffect::Torch(5)],
            Self::FlareLight(2) => &[StatusEffect::Torch(4)],
            Self::FlareLight(0..=1) => &[StatusEffect::Torch(3)],
            Self::Darkness => &[StatusEffect::Torch(-5)],
            Self::Destealth => &[StatusEffect::Unstealth],
            Self::FlareClear => &[StatusEffect::Unstun, StatusEffect::Untag],
            Self::AbyssalKiller(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqSelfSpeed(_)
            | Self::ArbMarkDebuff(_)
            | Self::ArbSelfSpeed(_)
            | Self::ArbStackingHeal(_)
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BellowCrit(_)
            | Self::BhDmgMarked
            | Self::BhMarkDebuff(_)
            | Self::BhSelfSpeed(_)
            | Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::CaltropsPreyDebuff(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::Command(_)
            | Self::CrusaderBulwark(_)
            | Self::Defender(_)
            | Self::Disrupt(_)
            | Self::DodgeCurse(_)
            | Self::EldritchKiller(_)
            | Self::EmboldenTeam(_)
            | Self::FortifyResists(_)
            | Self::GrAccBuff(_)
            | Self::GrapeshotVulnerability(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlightDebuff(_)
            | Self::GrDaggerDmgMarked(_)
            | Self::GrDodge(_)
            | Self::GrFadeAttack(_)
            | Self::GrSelfSpeed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HmDmgMarked(_)
            | Self::HoundDebuff(_)
            | Self::HoundProtect(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::HwPistolDmgMarked(_)
            | Self::JesterSpotlight(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperAcc
            | Self::LeperHype(_)
            | Self::LeperIntimidate(_)
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::LeperVulnerability
            | Self::ManKiller(_)
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::OccWeakenProt(_)
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
            | Self::OnCritStressResist
            | Self::PdVapoursBuff(_)
            | Self::PoisonKiller(_)
            | Self::Rakebuff(_)
            | Self::SlamDebuff(_)
            | Self::SniperDamage(_)
            | Self::StunKiller(_)
            | Self::Suppression(_)
            | Self::TrackingBuff(_)
            | Self::UnholyKiller(_)
            | Self::VestalInspiration(_)
            | Self::VomitDebuff(_)
            | Self::XformDamage(_) => &[],
        }
    }

    const fn swap_source_and_target(&self) -> bool {
        if matches!(self, Self::AntiqProtectMeGuard) { true } else { false }
    }

    const fn target(&self) -> Target {
        match self {
            Self::CrusaderBulwarkLight
            | Self::CrusaderLight(_)
            | Self::Darkness
            | Self::DazzlingLight
            | Self::FlareLight(_)
            | Self::VestalLight(_) => Target::Global,
            Self::AbsolutionHeal(_)
            | Self::AbsolutionHealStress(_)
            | Self::AbyssalKiller(_)
            | Self::Adrenaline(_)
            | Self::AntiqBlightBuff(_)
            | Self::AntiqCower(_)
            | Self::AntiqProtectMeClearGuardsPerformer
            | Self::AntiqSelfSpeed(_)
            | Self::ArbSelfSpeed(_)
            | Self::BeastBuff(_)
            | Self::BeastBuff2(_)
            | Self::BeastDebuff(_)
            | Self::BeastKiller(_)
            | Self::BhDmgMarked
            | Self::BhSelfSpeed(_)
            | Self::BuildToFinale(_)
            | Self::BuildToFinaleSong
            | Self::ClearGuardPerformer
            | Self::CrusaderBulwark(_)
            | Self::CrusaderBulwarkMark
            | Self::CureSelf
            | Self::Defender(_)
            | Self::EldritchKiller(_)
            | Self::GrAccBuff(_)
            | Self::GrDaggerDmgMarked(_)
            | Self::GrDodge(_)
            | Self::GrFadeAttack(_)
            | Self::GrSelfSpeed(_)
            | Self::HellionExhaust
            | Self::HellionExhaustSm
            | Self::HellionHealSelf(_)
            | Self::HmDmgMarked(_)
            | Self::HoundProtect(_)
            | Self::HwPistolDmgMarked(_)
            | Self::HwyRiposte(_)
            | Self::JesterSpotlight(_)
            | Self::LeperAcc
            | Self::LeperHealSelf(_)
            | Self::LeperHealSelfStress(_)
            | Self::LeperHype(_)
            | Self::LeperIntimidateMark
            | Self::LeperMarkSelf
            | Self::LeperProtect(_)
            | Self::LeperResistBuff
            | Self::LeperStrength(_)
            | Self::LeperVulnerability
            | Self::LickWounds(_)
            | Self::MaaRiposte(_)
            | Self::ManKiller(_)
            | Self::MarkSelf
            | Self::MortalWeakness
            | Self::MortalWeaknessStress
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
            | Self::OnCritStressResist
            | Self::PoisonKiller(_)
            | Self::Rakebuff(_)
            | Self::ShadowBlood(_)
            | Self::SniperDamage(_)
            | Self::SoloMarkSelf
            | Self::StealthSelf
            | Self::StunKiller(_)
            | Self::SwitchModeBeastSelf
            | Self::SwitchModeHumanSelf
            | Self::TrackingBuff(_)
            | Self::TransformHealSelf(_)
            | Self::UnholyKiller(_)
            | Self::VestalHealSelf(_)
            | Self::VestalInspiration(_)
            | Self::XformDamage(_) => Target::Performer,
            Self::BeastStressParty | Self::FlareClear | Self::FlareHealStress(_) | Self::HumanStressHealParty(_) => Target::PerformerGroupOther,
            Self::AbomVomit(_)
            | Self::AbyssalStun(_)
            | Self::AntiqBlight(_)
            | Self::AntiqBlightDebuff(_)
            | Self::AntiqDefBuff(_)
            | Self::AntiqDistract(_)
            | Self::AntiqDodge(_)
            | Self::AntiqProtectMeClearGuardsTarget
            | Self::AntiqProtectMeGuard
            | Self::ArbMarkDebuff(_)
            | Self::ArbMarkTarget
            | Self::ArbStackingHeal(_)
            | Self::BellowCrit(_)
            | Self::BhMarkDebuff(_)
            | Self::BhMarkTarget
            | Self::BhMinorMark
            | Self::Bleed(_)
            | Self::BoloPush1(_)
            | Self::Bolster(_)
            | Self::BolsterStressBuff(_)
            | Self::CaltropsPreyDebuff(_)
            | Self::CaltropsSpdDebuff(_)
            | Self::ClearGuardTarget
            | Self::Command(_)
            | Self::CrusaderHealStress(_)
            | Self::Cure
            | Self::Destealth
            | Self::Disorient(_)
            | Self::Disrupt(_)
            | Self::DodgeCurse(_)
            | Self::EmboldenTeam(_)
            | Self::FortifyResists(_)
            | Self::GrapeshotVulnerability(_)
            | Self::GrBleedDebuff(_)
            | Self::GrBlight(_)
            | Self::GrBlightDebuff(_)
            | Self::HarryBleed(_)
            | Self::HeroStrongStun(_)
            | Self::HmGuard
            | Self::HmMarkTarget
            | Self::HoundBleed(_)
            | Self::HoundDebuff(_)
            | Self::HoundHowl(_)
            | Self::HwOpenVeinBleedDebuff(_)
            | Self::HwOpenVeinSpdDebuff(_)
            | Self::InspiringTune(_)
            | Self::JesterTuneBuff(_)
            | Self::LeperIntimidate(_)
            | Self::MaaGuard
            | Self::ManaclesStun(_)
            | Self::MarkTarget
            | Self::NoxiousDebuff(_)
            | Self::OccVulnerabilityCurse(_)
            | Self::OccWeakeningCurse(_)
            | Self::OccWeakenProt(_)
            | Self::PdBlight(_)
            | Self::PdDisorientingStun(_)
            | Self::PdSingleBlight(_)
            | Self::PdVapoursBuff(_)
            | Self::Pull2(_)
            | Self::Push1(_)
            | Self::Push2(_)
            | Self::Push3(_)
            | Self::SlamDebuff(_)
            | Self::Stealth
            | Self::StrongBleed(_)
            | Self::Stun(_)
            | Self::Suppression(_)
            | Self::VestalStun(_)
            | Self::VomitDebuff(_)
            | Self::WyrdBleed(_) => Target::Target,
            Self::ClearCorpses => Target::TargetGroup,
        }
    }
}
