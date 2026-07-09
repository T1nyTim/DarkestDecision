use crate::catalog::effect::effect::Effect;

pub const ABYSSAL_KILLER: [&[Effect]; 5] = [
    &[Effect::AbyssalKiller(1)],
    &[Effect::AbyssalKiller(2)],
    &[Effect::AbyssalKiller(3)],
    &[Effect::AbyssalKiller(4)],
    &[Effect::AbyssalKiller(5)],
];
pub const ADRENALINE_RUSH_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Adrenaline(1), Effect::HellionHealSelf(1)],
    &[Effect::Adrenaline(2), Effect::HellionHealSelf(2)],
    &[Effect::Adrenaline(3), Effect::HellionHealSelf(3)],
    &[Effect::Adrenaline(4), Effect::HellionHealSelf(4)],
    &[Effect::Adrenaline(5), Effect::HellionHealSelf(5)],
];
pub const ANTIQ_DODGE: [&[Effect]; 5] = [
    &[Effect::AntiqDodge(1)],
    &[Effect::AntiqDodge(2)],
    &[Effect::AntiqDodge(3)],
    &[Effect::AntiqDodge(4)],
    &[Effect::AntiqDodge(5)],
];
pub const ARB_SELF_SPEED: [&[Effect]; 5] = [
    &[Effect::ArbSelfSpeed(1)],
    &[Effect::ArbSelfSpeed(2)],
    &[Effect::ArbSelfSpeed(3)],
    &[Effect::ArbSelfSpeed(4)],
    &[Effect::ArbSelfSpeed(5)],
];
pub const ARB_STACKING_HEAL: [&[Effect]; 5] = [
    &[Effect::ArbStackingHeal(1)],
    &[Effect::ArbStackingHeal(2)],
    &[Effect::ArbStackingHeal(3)],
    &[Effect::ArbStackingHeal(4)],
    &[Effect::ArbStackingHeal(5)],
];
pub const BARBARIC_YAWP_EFFECTS: [&[Effect]; 5] = [
    &[Effect::HeroStrongStun(1), Effect::HellionExhaust],
    &[Effect::HeroStrongStun(2), Effect::HellionExhaust],
    &[Effect::HeroStrongStun(3), Effect::HellionExhaust],
    &[Effect::HeroStrongStun(4), Effect::HellionExhaust],
    &[Effect::HeroStrongStun(5), Effect::HellionExhaust],
];
pub const BATTLE_BALLAD_EFFECTS: [&[Effect]; 5] = [
    &[Effect::EmboldenTeam(1), Effect::BuildToFinaleSong],
    &[Effect::EmboldenTeam(2), Effect::BuildToFinaleSong],
    &[Effect::EmboldenTeam(3), Effect::BuildToFinaleSong],
    &[Effect::EmboldenTeam(4), Effect::BuildToFinaleSong],
    &[Effect::EmboldenTeam(5), Effect::BuildToFinaleSong],
];
pub const BELLOW_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Disrupt(1), Effect::BellowCrit(1)],
    &[Effect::Disrupt(2), Effect::BellowCrit(2)],
    &[Effect::Disrupt(3), Effect::BellowCrit(3)],
    &[Effect::Disrupt(4), Effect::BellowCrit(4)],
    &[Effect::Disrupt(5), Effect::BellowCrit(5)],
];
pub const BLEED: [&[Effect]; 5] = [
    &[Effect::Bleed(1)],
    &[Effect::Bleed(2)],
    &[Effect::Bleed(3)],
    &[Effect::Bleed(4)],
    &[Effect::Bleed(5)],
];
pub const BLEED_OUT_EFFECTS: [&[Effect]; 5] = [
    &[Effect::StrongBleed(1), Effect::HellionExhaust],
    &[Effect::StrongBleed(2), Effect::HellionExhaust],
    &[Effect::StrongBleed(3), Effect::HellionExhaust],
    &[Effect::StrongBleed(4), Effect::HellionExhaust],
    &[Effect::StrongBleed(5), Effect::HellionExhaust],
];
pub const BOLO_PUSH_1: [&[Effect]; 5] = [
    &[Effect::BoloPush1(1)],
    &[Effect::BoloPush1(2)],
    &[Effect::BoloPush1(3)],
    &[Effect::BoloPush1(4)],
    &[Effect::BoloPush1(5)],
];
pub const BOLSTER_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Bolster(1), Effect::BolsterStressBuff(1)],
    &[Effect::Bolster(2), Effect::BolsterStressBuff(2)],
    &[Effect::Bolster(3), Effect::BolsterStressBuff(3)],
    &[Effect::Bolster(4), Effect::BolsterStressBuff(4)],
    &[Effect::Bolster(5), Effect::BolsterStressBuff(5)],
];
pub const BULWARK_OF_FAITH_EFFECTS: [&[Effect]; 5] = [
    &[Effect::CrusaderBulwark(1), Effect::CrusaderBulwarkMark, Effect::CrusaderBulwarkLight],
    &[Effect::CrusaderBulwark(2), Effect::CrusaderBulwarkMark, Effect::CrusaderBulwarkLight],
    &[Effect::CrusaderBulwark(3), Effect::CrusaderBulwarkMark, Effect::CrusaderBulwarkLight],
    &[Effect::CrusaderBulwark(4), Effect::CrusaderBulwarkMark, Effect::CrusaderBulwarkLight],
    &[Effect::CrusaderBulwark(5), Effect::CrusaderBulwarkMark, Effect::CrusaderBulwarkLight],
];
pub const COLLECT_BOUNTY_EFFECTS: [&[Effect]; 5] = [
    &[Effect::BhDmgMarked, Effect::ManKiller(1)],
    &[Effect::BhDmgMarked, Effect::ManKiller(2)],
    &[Effect::BhDmgMarked, Effect::ManKiller(3)],
    &[Effect::BhDmgMarked, Effect::ManKiller(4)],
    &[Effect::BhDmgMarked, Effect::ManKiller(5)],
];
pub const COME_HITHER_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Pull2(1), Effect::BhMinorMark],
    &[Effect::Pull2(1), Effect::BhMinorMark],
    &[Effect::Pull2(1), Effect::BhMinorMark],
    &[Effect::Pull2(1), Effect::BhMinorMark],
    &[Effect::Pull2(1), Effect::BhMinorMark],
];
pub const COMMAND: [&[Effect]; 5] = [
    &[Effect::Command(1)],
    &[Effect::Command(2)],
    &[Effect::Command(3)],
    &[Effect::Command(4)],
    &[Effect::Command(5)],
];
pub const COWER_EFFECTS: [&[Effect]; 5] = [
    &[Effect::AntiqCower(1), Effect::AntiqSelfSpeed(1), Effect::AntiqBlightBuff(1)],
    &[Effect::AntiqCower(2), Effect::AntiqSelfSpeed(2), Effect::AntiqBlightBuff(2)],
    &[Effect::AntiqCower(3), Effect::AntiqSelfSpeed(3), Effect::AntiqBlightBuff(3)],
    &[Effect::AntiqCower(4), Effect::AntiqSelfSpeed(4), Effect::AntiqBlightBuff(4)],
    &[Effect::AntiqCower(5), Effect::AntiqSelfSpeed(5), Effect::AntiqBlightBuff(5)],
];
pub const DAEMONS_PULL_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Pull2(1), Effect::ClearCorpses],
    &[Effect::Pull2(2), Effect::ClearCorpses],
    &[Effect::Pull2(3), Effect::ClearCorpses],
    &[Effect::Pull2(4), Effect::ClearCorpses],
    &[Effect::Pull2(5), Effect::ClearCorpses],
];
pub const DAZZLING_LIGHT_EFFECTS: [&[Effect]; 5] = [
    &[Effect::VestalStun(1), Effect::DazzlingLight],
    &[Effect::VestalStun(2), Effect::DazzlingLight],
    &[Effect::VestalStun(3), Effect::DazzlingLight],
    &[Effect::VestalStun(4), Effect::DazzlingLight],
    &[Effect::VestalStun(5), Effect::DazzlingLight],
];
pub const DEFENDER_EFFECTS: [&[Effect]; 5] = [
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::MaaGuard,
        Effect::Defender(1),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::MaaGuard,
        Effect::Defender(2),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::MaaGuard,
        Effect::Defender(3),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::MaaGuard,
        Effect::Defender(4),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::MaaGuard,
        Effect::Defender(5),
    ],
];
pub const DISORIENTING_BLAST_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Disorient(1), Effect::ClearCorpses, Effect::PdDisorientingStun(1)],
    &[Effect::Disorient(2), Effect::ClearCorpses, Effect::PdDisorientingStun(2)],
    &[Effect::Disorient(3), Effect::ClearCorpses, Effect::PdDisorientingStun(3)],
    &[Effect::Disorient(4), Effect::ClearCorpses, Effect::PdDisorientingStun(4)],
    &[Effect::Disorient(5), Effect::ClearCorpses, Effect::PdDisorientingStun(5)],
];
pub const DISRUPTIVE_CURSE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::OccVulnerabilityCurse(1), Effect::MarkTarget],
    &[Effect::OccVulnerabilityCurse(2), Effect::MarkTarget],
    &[Effect::OccVulnerabilityCurse(3), Effect::MarkTarget],
    &[Effect::OccVulnerabilityCurse(4), Effect::MarkTarget],
    &[Effect::OccVulnerabilityCurse(5), Effect::MarkTarget],
];
pub const ELDRITCH_KILLER: [&[Effect]; 5] = [
    &[Effect::EldritchKiller(1)],
    &[Effect::EldritchKiller(2)],
    &[Effect::EldritchKiller(3)],
    &[Effect::EldritchKiller(4)],
    &[Effect::EldritchKiller(5)],
];
pub const FESTERING_VAPOURS_EFFECTS: [&[Effect]; 5] = [
    &[Effect::AntiqBlight(1), Effect::AntiqBlightDebuff(1)],
    &[Effect::AntiqBlight(2), Effect::AntiqBlightDebuff(2)],
    &[Effect::AntiqBlight(3), Effect::AntiqBlightDebuff(3)],
    &[Effect::AntiqBlight(4), Effect::AntiqBlightDebuff(4)],
    &[Effect::AntiqBlight(5), Effect::AntiqBlightDebuff(5)],
];
pub const FLARE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::FlareClear, Effect::FlareLight(1), Effect::Stealth, Effect::FlareHealStress(1)],
    &[Effect::FlareClear, Effect::FlareLight(2), Effect::Stealth, Effect::FlareHealStress(2)],
    &[Effect::FlareClear, Effect::FlareLight(3), Effect::Stealth, Effect::FlareHealStress(3)],
    &[Effect::FlareClear, Effect::FlareLight(4), Effect::Stealth, Effect::FlareHealStress(4)],
    &[Effect::FlareClear, Effect::FlareLight(5), Effect::Stealth, Effect::FlareHealStress(5)],
];
pub const FLASHBANG_EFFECTS: [&[Effect]; 5] = [
    &[Effect::HeroStrongStun(1), Effect::Disorient(1)],
    &[Effect::HeroStrongStun(2), Effect::Disorient(2)],
    &[Effect::HeroStrongStun(3), Effect::Disorient(3)],
    &[Effect::HeroStrongStun(4), Effect::Disorient(4)],
    &[Effect::HeroStrongStun(5), Effect::Disorient(5)],
];
pub const FLASHPOWDER_EFFECTS: [&[Effect]; 5] = [
    &[Effect::AntiqDistract(1), Effect::Destealth],
    &[Effect::AntiqDistract(2), Effect::Destealth],
    &[Effect::AntiqDistract(3), Effect::Destealth],
    &[Effect::AntiqDistract(4), Effect::Destealth],
    &[Effect::AntiqDistract(5), Effect::Destealth],
];
pub const FOCUS_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Push3(1), Effect::ClearCorpses, Effect::LeperAcc],
    &[Effect::Push3(2), Effect::ClearCorpses, Effect::LeperAcc],
    &[Effect::Push3(3), Effect::ClearCorpses, Effect::LeperAcc],
    &[Effect::Push3(4), Effect::ClearCorpses, Effect::LeperAcc],
    &[Effect::Push3(5), Effect::ClearCorpses, Effect::LeperAcc],
];
pub const FORTIFY_RESISTS: [&[Effect]; 5] = [
    &[Effect::FortifyResists(1)],
    &[Effect::FortifyResists(2)],
    &[Effect::FortifyResists(3)],
    &[Effect::FortifyResists(4)],
    &[Effect::FortifyResists(5)],
];
pub const GODS_HAND_EFFECTS: [&[Effect]; 5] = [
    &[Effect::VestalInspiration(1), Effect::UnholyKiller(1)],
    &[Effect::VestalInspiration(2), Effect::UnholyKiller(2)],
    &[Effect::VestalInspiration(3), Effect::UnholyKiller(3)],
    &[Effect::VestalInspiration(4), Effect::UnholyKiller(4)],
    &[Effect::VestalInspiration(5), Effect::UnholyKiller(5)],
];
pub const GODS_ILLUMINATION_EFFECTS: [&[Effect]; 5] = [
    &[Effect::DodgeCurse(1), Effect::VestalLight(1), Effect::Destealth],
    &[Effect::DodgeCurse(2), Effect::VestalLight(2), Effect::Destealth],
    &[Effect::DodgeCurse(3), Effect::VestalLight(3), Effect::Destealth],
    &[Effect::DodgeCurse(4), Effect::VestalLight(4), Effect::Destealth],
    &[Effect::DodgeCurse(5), Effect::VestalLight(5), Effect::Destealth],
];
pub const GR_BLEED_DEBUFF: [&[Effect]; 5] = [
    &[Effect::GrBleedDebuff(1)],
    &[Effect::GrBleedDebuff(2)],
    &[Effect::GrBleedDebuff(3)],
    &[Effect::GrBleedDebuff(4)],
    &[Effect::GrBleedDebuff(5)],
];
pub const GRAPESHOT_VULNERABILITY: [&[Effect]; 5] = [
    &[Effect::GrapeshotVulnerability(1)],
    &[Effect::GrapeshotVulnerability(2)],
    &[Effect::GrapeshotVulnerability(3)],
    &[Effect::GrapeshotVulnerability(4)],
    &[Effect::GrapeshotVulnerability(5)],
];
pub const GUARD_DOG_EFFECTS: [&[Effect]; 5] = [
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::HmGuard,
        Effect::HoundProtect(1),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::HmGuard,
        Effect::HoundProtect(2),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::HmGuard,
        Effect::HoundProtect(3),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::HmGuard,
        Effect::HoundProtect(4),
    ],
    &[
        Effect::ClearGuardPerformer,
        Effect::ClearGuardTarget,
        Effect::HmGuard,
        Effect::HoundProtect(5),
    ],
];
pub const HANDS_FROM_ABYSS: [&[Effect]; 5] = [
    &[Effect::AbyssalStun(1), Effect::Darkness],
    &[Effect::AbyssalStun(2), Effect::Darkness],
    &[Effect::AbyssalStun(3), Effect::Darkness],
    &[Effect::AbyssalStun(4), Effect::Darkness],
    &[Effect::AbyssalStun(5), Effect::Darkness],
];
pub const HARRY_BLEED: [&[Effect]; 5] = [
    &[Effect::HarryBleed(1)],
    &[Effect::HarryBleed(2)],
    &[Effect::HarryBleed(3)],
    &[Effect::HarryBleed(4)],
    &[Effect::HarryBleed(5)],
];
pub const HARVEST_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Bleed(1), Effect::BuildToFinale(1)],
    &[Effect::Bleed(2), Effect::BuildToFinale(1)],
    &[Effect::Bleed(3), Effect::BuildToFinale(1)],
    &[Effect::Bleed(4), Effect::BuildToFinale(1)],
    &[Effect::Bleed(5), Effect::BuildToFinale(1)],
];
pub const HERO_STRONG_STUN: [&[Effect]; 5] = [
    &[Effect::HeroStrongStun(1)],
    &[Effect::HeroStrongStun(2)],
    &[Effect::HeroStrongStun(3)],
    &[Effect::HeroStrongStun(4)],
    &[Effect::HeroStrongStun(5)],
];
pub const HOOK_AND_SLICE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Bleed(1), Effect::CaltropsSpdDebuff(1), Effect::CaltropsPreyDebuff(1)],
    &[Effect::Bleed(2), Effect::CaltropsSpdDebuff(2), Effect::CaltropsPreyDebuff(2)],
    &[Effect::Bleed(3), Effect::CaltropsSpdDebuff(3), Effect::CaltropsPreyDebuff(3)],
    &[Effect::Bleed(4), Effect::CaltropsSpdDebuff(4), Effect::CaltropsPreyDebuff(4)],
    &[Effect::Bleed(5), Effect::CaltropsSpdDebuff(5), Effect::CaltropsPreyDebuff(5)],
];
pub const HOUND_HOWL: [&[Effect]; 5] = [
    &[Effect::HoundHowl(1)],
    &[Effect::HoundHowl(2)],
    &[Effect::HoundHowl(3)],
    &[Effect::HoundHowl(4)],
    &[Effect::HoundHowl(5)],
];
pub const HOUNDS_RUSH_EFFECTS: [&[Effect]; 5] = [
    &[Effect::BeastKiller(1), Effect::HmDmgMarked(1), Effect::HoundBleed(1)],
    &[Effect::BeastKiller(2), Effect::HmDmgMarked(2), Effect::HoundBleed(2)],
    &[Effect::BeastKiller(3), Effect::HmDmgMarked(3), Effect::HoundBleed(3)],
    &[Effect::BeastKiller(4), Effect::HmDmgMarked(4), Effect::HoundBleed(4)],
    &[Effect::BeastKiller(5), Effect::HmDmgMarked(5), Effect::HoundBleed(5)],
];
pub const HW_PISTOL_DMG_MARKED: [&[Effect]; 5] = [
    &[Effect::HwPistolDmgMarked(1)],
    &[Effect::HwPistolDmgMarked(2)],
    &[Effect::HwPistolDmgMarked(3)],
    &[Effect::HwPistolDmgMarked(4)],
    &[Effect::HwPistolDmgMarked(5)],
];
pub const HWY_RIPOSTE: [&[Effect]; 5] = [
    &[Effect::HwyRiposte(1)],
    &[Effect::HwyRiposte(2)],
    &[Effect::HwyRiposte(3)],
    &[Effect::HwyRiposte(4)],
    &[Effect::HwyRiposte(5)],
];
pub const INSPIRING_CRY_EFFECTS: [&[Effect]; 5] = [
    &[Effect::CrusaderHealStress(1), Effect::CrusaderLight(1)],
    &[Effect::CrusaderHealStress(2), Effect::CrusaderLight(2)],
    &[Effect::CrusaderHealStress(3), Effect::CrusaderLight(3)],
    &[Effect::CrusaderHealStress(4), Effect::CrusaderLight(4)],
    &[Effect::CrusaderHealStress(5), Effect::CrusaderLight(5)],
];
pub const INSPIRING_TUNE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::InspiringTune(1), Effect::JesterTuneBuff(1), Effect::BuildToFinaleSong],
    &[Effect::InspiringTune(2), Effect::JesterTuneBuff(2), Effect::BuildToFinaleSong],
    &[Effect::InspiringTune(3), Effect::JesterTuneBuff(3), Effect::BuildToFinaleSong],
    &[Effect::InspiringTune(4), Effect::JesterTuneBuff(4), Effect::BuildToFinaleSong],
    &[Effect::InspiringTune(5), Effect::JesterTuneBuff(5), Effect::BuildToFinaleSong],
];
pub const INTIMIDATE_EFFECTS: [&[Effect]; 5] = [
    &[
        Effect::LeperIntimidate(1),
        Effect::LeperIntimidateMark,
        Effect::Destealth,
        Effect::LeperHype(1),
    ],
    &[
        Effect::LeperIntimidate(2),
        Effect::LeperIntimidateMark,
        Effect::Destealth,
        Effect::LeperHype(2),
    ],
    &[
        Effect::LeperIntimidate(3),
        Effect::LeperIntimidateMark,
        Effect::Destealth,
        Effect::LeperHype(3),
    ],
    &[
        Effect::LeperIntimidate(4),
        Effect::LeperIntimidateMark,
        Effect::Destealth,
        Effect::LeperHype(4),
    ],
    &[
        Effect::LeperIntimidate(5),
        Effect::LeperIntimidateMark,
        Effect::Destealth,
        Effect::LeperHype(5),
    ],
];
pub const LICK_WOUNDS: [&[Effect]; 5] = [
    &[Effect::LickWounds(1)],
    &[Effect::LickWounds(2)],
    &[Effect::LickWounds(3)],
    &[Effect::LickWounds(4)],
    &[Effect::LickWounds(5)],
];
pub const NOXIOUS_BLAST_EFFECTS: [&[Effect]; 5] = [
    &[Effect::PdSingleBlight(1), Effect::NoxiousDebuff(1)],
    &[Effect::PdSingleBlight(2), Effect::NoxiousDebuff(2)],
    &[Effect::PdSingleBlight(3), Effect::NoxiousDebuff(3)],
    &[Effect::PdSingleBlight(4), Effect::NoxiousDebuff(4)],
    &[Effect::PdSingleBlight(5), Effect::NoxiousDebuff(5)],
];
pub const OPENED_VEIN_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Bleed(1), Effect::HwOpenVeinBleedDebuff(1), Effect::HwOpenVeinSpdDebuff(1)],
    &[Effect::Bleed(2), Effect::HwOpenVeinBleedDebuff(2), Effect::HwOpenVeinSpdDebuff(2)],
    &[Effect::Bleed(3), Effect::HwOpenVeinBleedDebuff(3), Effect::HwOpenVeinSpdDebuff(3)],
    &[Effect::Bleed(4), Effect::HwOpenVeinBleedDebuff(4), Effect::HwOpenVeinSpdDebuff(4)],
    &[Effect::Bleed(5), Effect::HwOpenVeinBleedDebuff(5), Effect::HwOpenVeinSpdDebuff(5)],
];
pub const PD_BLIGHT: [&[Effect]; 5] = [
    &[Effect::PdBlight(1)],
    &[Effect::PdBlight(2)],
    &[Effect::PdBlight(3)],
    &[Effect::PdBlight(4)],
    &[Effect::PdBlight(5)],
];
pub const PD_VAPOURS_BUFF: [&[Effect]; 5] = [
    &[Effect::PdVapoursBuff(1)],
    &[Effect::PdVapoursBuff(2)],
    &[Effect::PdVapoursBuff(3)],
    &[Effect::PdVapoursBuff(4)],
    &[Effect::PdVapoursBuff(5)],
];
pub const POISON_DART_EFFECTS: [&[Effect]; 5] = [
    &[Effect::GrBlight(1), Effect::GrBlightDebuff(1)],
    &[Effect::GrBlight(2), Effect::GrBlightDebuff(2)],
    &[Effect::GrBlight(3), Effect::GrBlightDebuff(3)],
    &[Effect::GrBlight(4), Effect::GrBlightDebuff(4)],
    &[Effect::GrBlight(5), Effect::GrBlightDebuff(5)],
];
pub const POISON_KILLER: [&[Effect]; 5] = [
    &[Effect::PoisonKiller(1)],
    &[Effect::PoisonKiller(2)],
    &[Effect::PoisonKiller(3)],
    &[Effect::PoisonKiller(4)],
    &[Effect::PoisonKiller(5)],
];
pub const PROTECT_ME_EFFECTS: [&[Effect]; 5] = [
    &[
        Effect::AntiqDefBuff(1),
        Effect::MarkTarget,
        Effect::AntiqProtectMeClearGuardsPerformer,
        Effect::AntiqProtectMeClearGuardsTarget,
        Effect::AntiqProtectMeGuard,
    ],
    &[
        Effect::AntiqDefBuff(2),
        Effect::MarkTarget,
        Effect::AntiqProtectMeClearGuardsPerformer,
        Effect::AntiqProtectMeClearGuardsTarget,
        Effect::AntiqProtectMeGuard,
    ],
    &[
        Effect::AntiqDefBuff(3),
        Effect::MarkTarget,
        Effect::AntiqProtectMeClearGuardsPerformer,
        Effect::AntiqProtectMeClearGuardsTarget,
        Effect::AntiqProtectMeGuard,
    ],
    &[
        Effect::AntiqDefBuff(4),
        Effect::MarkTarget,
        Effect::AntiqProtectMeClearGuardsPerformer,
        Effect::AntiqProtectMeClearGuardsTarget,
        Effect::AntiqProtectMeGuard,
    ],
    &[
        Effect::AntiqDefBuff(5),
        Effect::MarkTarget,
        Effect::AntiqProtectMeClearGuardsPerformer,
        Effect::AntiqProtectMeClearGuardsTarget,
        Effect::AntiqProtectMeGuard,
    ],
];
pub const PUSH_1: [&[Effect]; 5] = [
    &[Effect::Push1(1)],
    &[Effect::Push1(2)],
    &[Effect::Push1(3)],
    &[Effect::Push1(4)],
    &[Effect::Push1(5)],
];
pub const RAMPART_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Push1(1), Effect::Stun(1)],
    &[Effect::Push1(2), Effect::Stun(2)],
    &[Effect::Push1(3), Effect::Stun(3)],
    &[Effect::Push1(4), Effect::Stun(4)],
    &[Effect::Push1(5), Effect::Stun(5)],
];
pub const RETRIBUTION_EFFECTS: [&[Effect]; 5] = [
    &[Effect::MaaRiposte(1), Effect::MarkSelf],
    &[Effect::MaaRiposte(2), Effect::MarkSelf],
    &[Effect::MaaRiposte(3), Effect::MarkSelf],
    &[Effect::MaaRiposte(4), Effect::MarkSelf],
    &[Effect::MaaRiposte(5), Effect::MarkSelf],
];
pub const REVENGE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::LeperStrength(1), Effect::LeperVulnerability],
    &[Effect::LeperStrength(2), Effect::LeperVulnerability],
    &[Effect::LeperStrength(3), Effect::LeperVulnerability],
    &[Effect::LeperStrength(4), Effect::LeperVulnerability],
    &[Effect::LeperStrength(5), Effect::LeperVulnerability],
];
pub const SHADOW_FADE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::StealthSelf, Effect::GrFadeAttack(1), Effect::GrDodge(1)],
    &[Effect::StealthSelf, Effect::GrFadeAttack(2), Effect::GrDodge(2)],
    &[Effect::StealthSelf, Effect::GrFadeAttack(3), Effect::GrDodge(3)],
    &[Effect::StealthSelf, Effect::GrFadeAttack(4), Effect::GrDodge(4)],
    &[Effect::StealthSelf, Effect::GrFadeAttack(5), Effect::GrDodge(5)],
];
pub const SLICE_OFF_EFFECTS: [&[Effect]; 5] = [
    &[Effect::StrongBleed(1), Effect::BuildToFinale(1)],
    &[Effect::StrongBleed(2), Effect::BuildToFinale(1)],
    &[Effect::StrongBleed(3), Effect::BuildToFinale(1)],
    &[Effect::StrongBleed(4), Effect::BuildToFinale(1)],
    &[Effect::StrongBleed(5), Effect::BuildToFinale(1)],
];
pub const SNIPER_DAMAGE: [&[Effect]; 5] = [
    &[Effect::SniperDamage(1)],
    &[Effect::SniperDamage(2)],
    &[Effect::SniperDamage(3)],
    &[Effect::SniperDamage(4)],
    &[Effect::SniperDamage(5)],
];
pub const SNIPER_MARK_EFFECTS: [&[Effect]; 5] = [
    &[Effect::ArbMarkTarget, Effect::ArbMarkDebuff(1)],
    &[Effect::ArbMarkTarget, Effect::ArbMarkDebuff(2)],
    &[Effect::ArbMarkTarget, Effect::ArbMarkDebuff(3)],
    &[Effect::ArbMarkTarget, Effect::ArbMarkDebuff(4)],
    &[Effect::ArbMarkTarget, Effect::ArbMarkDebuff(5)],
];
pub const SOLEMNITY_EFFECTS: [&[Effect]; 5] = [
    &[Effect::LeperHealSelf(1), Effect::LeperHealSelfStress(1)],
    &[Effect::LeperHealSelf(2), Effect::LeperHealSelfStress(2)],
    &[Effect::LeperHealSelf(3), Effect::LeperHealSelfStress(3)],
    &[Effect::LeperHealSelf(4), Effect::LeperHealSelfStress(4)],
    &[Effect::LeperHealSelf(5), Effect::LeperHealSelfStress(5)],
];
pub const SOLO_EFFECTS: [&[Effect]; 5] = [
    &[Effect::SoloMarkSelf, Effect::JesterSpotlight(1), Effect::BuildToFinale(2)],
    &[Effect::SoloMarkSelf, Effect::JesterSpotlight(2), Effect::BuildToFinale(2)],
    &[Effect::SoloMarkSelf, Effect::JesterSpotlight(3), Effect::BuildToFinale(2)],
    &[Effect::SoloMarkSelf, Effect::JesterSpotlight(4), Effect::BuildToFinale(2)],
    &[Effect::SoloMarkSelf, Effect::JesterSpotlight(5), Effect::BuildToFinale(2)],
];
pub const STUN: [&[Effect]; 5] = [
    &[Effect::Stun(1)],
    &[Effect::Stun(2)],
    &[Effect::Stun(3)],
    &[Effect::Stun(4)],
    &[Effect::Stun(5)],
];
pub const STUN_KILLER: [&[Effect]; 5] = [
    &[Effect::StunKiller(1)],
    &[Effect::StunKiller(2)],
    &[Effect::StunKiller(3)],
    &[Effect::StunKiller(4)],
    &[Effect::StunKiller(5)],
];
pub const SUPPRESSION: [&[Effect]; 5] = [
    &[Effect::Suppression(1)],
    &[Effect::Suppression(2)],
    &[Effect::Suppression(3)],
    &[Effect::Suppression(4)],
    &[Effect::Suppression(5)],
];
pub const TAKE_AIM_EFFECTS: [&[Effect]; 5] = [
    &[Effect::TrackingBuff(1), Effect::TakeAim],
    &[Effect::TrackingBuff(2), Effect::TakeAim],
    &[Effect::TrackingBuff(3), Effect::TakeAim],
    &[Effect::TrackingBuff(4), Effect::TakeAim],
    &[Effect::TrackingBuff(5), Effect::TakeAim],
];
pub const TARGET_TAG_EFFECTS: [&[Effect]; 5] = [
    &[Effect::BhMarkTarget, Effect::BhMarkDebuff(1), Effect::BhSelfSpeed(1)],
    &[Effect::BhMarkTarget, Effect::BhMarkDebuff(2), Effect::BhSelfSpeed(2)],
    &[Effect::BhMarkTarget, Effect::BhMarkDebuff(3), Effect::BhSelfSpeed(3)],
    &[Effect::BhMarkTarget, Effect::BhMarkDebuff(4), Effect::BhSelfSpeed(4)],
    &[Effect::BhMarkTarget, Effect::BhMarkDebuff(5), Effect::BhSelfSpeed(5)],
];
pub const THROWN_DAGGER_EFFECTS: [&[Effect]; 5] = [
    &[Effect::GrAccBuff(1), Effect::GrDaggerDmgMarked(1), Effect::PoisonKiller(1)],
    &[Effect::GrAccBuff(2), Effect::GrDaggerDmgMarked(2), Effect::PoisonKiller(2)],
    &[Effect::GrAccBuff(3), Effect::GrDaggerDmgMarked(3), Effect::PoisonKiller(3)],
    &[Effect::GrAccBuff(4), Effect::GrDaggerDmgMarked(4), Effect::PoisonKiller(4)],
    &[Effect::GrAccBuff(5), Effect::GrDaggerDmgMarked(5), Effect::PoisonKiller(5)],
];
pub const TOXIN_TRICKERY_EFFECTS: [&[Effect]; 5] = [
    &[Effect::ShadowBlood(1), Effect::GrSelfSpeed(1)],
    &[Effect::ShadowBlood(2), Effect::GrSelfSpeed(2)],
    &[Effect::ShadowBlood(3), Effect::GrSelfSpeed(3)],
    &[Effect::ShadowBlood(4), Effect::GrSelfSpeed(4)],
    &[Effect::ShadowBlood(5), Effect::GrSelfSpeed(5)],
];
pub const UNHOLY_KILLER: [&[Effect]; 5] = [
    &[Effect::UnholyKiller(1)],
    &[Effect::UnholyKiller(2)],
    &[Effect::UnholyKiller(3)],
    &[Effect::UnholyKiller(4)],
    &[Effect::UnholyKiller(5)],
];
pub const UPPERCUT_EFFECTS: [&[Effect]; 5] = [
    &[Effect::Push2(1), Effect::Stun(1)],
    &[Effect::Push2(2), Effect::Stun(2)],
    &[Effect::Push2(3), Effect::Stun(3)],
    &[Effect::Push2(4), Effect::Stun(4)],
    &[Effect::Push2(5), Effect::Stun(5)],
];
pub const VESTAL_HEALSELF: [&[Effect]; 5] = [
    &[Effect::VestalHealSelf(1)],
    &[Effect::VestalHealSelf(2)],
    &[Effect::VestalHealSelf(3)],
    &[Effect::VestalHealSelf(4)],
    &[Effect::VestalHealSelf(5)],
];
pub const WEAKENING_CURSE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::OccWeakeningCurse(1), Effect::OccWeakenProt(1)],
    &[Effect::OccWeakeningCurse(2), Effect::OccWeakenProt(2)],
    &[Effect::OccWeakeningCurse(3), Effect::OccWeakenProt(3)],
    &[Effect::OccWeakeningCurse(4), Effect::OccWeakenProt(4)],
    &[Effect::OccWeakeningCurse(5), Effect::OccWeakenProt(5)],
];
pub const WHISTLE_EFFECTS: [&[Effect]; 5] = [
    &[Effect::HmMarkTarget, Effect::HoundDebuff(1)],
    &[Effect::HmMarkTarget, Effect::HoundDebuff(2)],
    &[Effect::HmMarkTarget, Effect::HoundDebuff(3)],
    &[Effect::HmMarkTarget, Effect::HoundDebuff(4)],
    &[Effect::HmMarkTarget, Effect::HoundDebuff(5)],
];
pub const WITHSTAND_EFFECTS: [&[Effect]; 5] = [
    &[Effect::LeperProtect(1), Effect::LeperResistBuff, Effect::LeperMarkSelf],
    &[Effect::LeperProtect(2), Effect::LeperResistBuff, Effect::LeperMarkSelf],
    &[Effect::LeperProtect(3), Effect::LeperResistBuff, Effect::LeperMarkSelf],
    &[Effect::LeperProtect(4), Effect::LeperResistBuff, Effect::LeperMarkSelf],
    &[Effect::LeperProtect(5), Effect::LeperResistBuff, Effect::LeperMarkSelf],
];
pub const WYRD_BLEED: [&[Effect]; 5] = [
    &[Effect::WyrdBleed(1)],
    &[Effect::WyrdBleed(2)],
    &[Effect::WyrdBleed(3)],
    &[Effect::WyrdBleed(4)],
    &[Effect::WyrdBleed(5)],
];
