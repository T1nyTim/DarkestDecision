use crate::catalog::{
    Rank,
    effect::{
        consts::{
            ABYSSAL_KILLER, ADRENALINE_RUSH_EFFECTS, ANTIQ_DODGE, ARB_SELF_SPEED, ARB_STACKING_HEAL, BARBARIC_YAWP_EFFECTS, BATTLE_BALLAD_EFFECTS,
            BELLOW_EFFECTS, BLEED, BLEED_OUT_EFFECTS, BOLO_PUSH_1, BOLSTER_EFFECTS, BULWARK_OF_FAITH_EFFECTS, COLLECT_BOUNTY_EFFECTS,
            COME_HITHER_EFFECTS, COMMAND, COWER_EFFECTS, DAEMONS_PULL_EFFECTS, DAZZLING_LIGHT_EFFECTS, DEFENDER_EFFECTS, DISORIENTING_BLAST_EFFECTS,
            DISRUPTIVE_CURSE_EFFECTS, ELDRITCH_KILLER, FESTERING_VAPOURS_EFFECTS, FLARE_EFFECTS, FLASHBANG_EFFECTS, FLASHPOWDER_EFFECTS,
            FOCUS_EFFECTS, FORTIFY_RESISTS, GODS_HAND_EFFECTS, GODS_ILLUMINATION_EFFECTS, GR_BLEED_DEBUFF, GRAPESHOT_VULNERABILITY,
            GUARD_DOG_EFFECTS, HANDS_FROM_ABYSS, HARRY_BLEED, HARVEST_EFFECTS, HERO_STRONG_STUN, HOOK_AND_SLICE_EFFECTS, HOUND_HOWL,
            HOUNDS_RUSH_EFFECTS, HW_PISTOL_DMG_MARKED, HWY_RIPOSTE, INSPIRING_CRY_EFFECTS, INSPIRING_TUNE_EFFECTS, INTIMIDATE_EFFECTS, LICK_WOUNDS,
            NOXIOUS_BLAST_EFFECTS, OPENED_VEIN_EFFECTS, PD_BLIGHT, PD_VAPOURS_BUFF, POISON_DART_EFFECTS, POISON_KILLER, PROTECT_ME_EFFECTS, PUSH_1,
            RAMPART_EFFECTS, RETRIBUTION_EFFECTS, REVENGE_EFFECTS, SHADOW_FADE_EFFECTS, SLICE_OFF_EFFECTS, SNIPER_DAMAGE, SNIPER_MARK_EFFECTS,
            SOLEMNITY_EFFECTS, SOLO_EFFECTS, STUN, STUN_KILLER, SUPPRESSION, TAKE_AIM_EFFECTS, TARGET_TAG_EFFECTS, THROWN_DAGGER_EFFECTS,
            TOXIN_TRICKERY_EFFECTS, UNHOLY_KILLER, UPPERCUT_EFFECTS, VESTAL_HEALSELF, WEAKENING_CURSE_EFFECTS, WHISTLE_EFFECTS, WITHSTAND_EFFECTS,
            WYRD_BLEED,
        },
        effect::Effect,
    },
    heroes::{Percent10, SkillType, TargetMod},
};

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
        let atk = match self {
            Self::HeroicEnd => [140, 145, 150, 155, 160],
            Self::Solo => [125, 130, 135, 140, 145],
            Self::SniperMark | Self::TargetTag | Self::Whistle => [100, 105, 110, 115, 120],
            Self::BarbaricYawp
            | Self::Blackjack
            | Self::BlindingGas
            | Self::Bola
            | Self::DisorientingBlast
            | Self::DisruptiveCurse
            | Self::FesteringVapours
            | Self::Flare
            | Self::Flashbang
            | Self::Flashpowder
            | Self::Intimidate
            | Self::Lunge
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::PoisonDart
            | Self::ShadowFade
            | Self::SliceOff
            | Self::SniperShot
            | Self::SuppressingFire
            | Self::TakeAim
            | Self::WeakeningCurse => [95, 100, 105, 110, 115],
            Self::Bellow
            | Self::ComeHither
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::DuelistAdvance
            | Self::FlashingDaggers
            | Self::GodsIllumination
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::HookAndSlice
            | Self::Pick
            | Self::Rampart
            | Self::StunningBlow
            | Self::ThrownDagger
            | Self::Uppercut => [90, 95, 100, 105, 110],
            Self::AbyssalArtillery
            | Self::BleedOut
            | Self::Breakthru
            | Self::CollectBounty
            | Self::Crush
            | Self::DirkStab
            | Self::FinishHim
            | Self::Focus
            | Self::GodsHand
            | Self::HolyLance
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::IfItBleeds
            | Self::Incision
            | Self::IronSwan
            | Self::Judgement
            | Self::KrisStab
            | Self::MaceBash
            | Self::PistolShot
            | Self::Retribution
            | Self::Smite
            | Self::WickedHack
            | Self::WickedSlice
            | Self::ZealousAccusation => [85, 90, 95, 100, 105],
            Self::Bloodlet => [80, 85, 90, 95, 100],
            Self::Blindfire | Self::Chop | Self::GrapeShotBlast | Self::Hew => [75, 80, 85, 90, 95],
            Self::AdrenalineRush
            | Self::BattleBallad
            | Self::Bolster
            | Self::BulwarkOfFaith
            | Self::Command
            | Self::Cower
            | Self::Defender
            | Self::EmboldeningVapours
            | Self::GuardDog
            | Self::Howl
            | Self::InspiringTune
            | Self::InvigoratingVapours
            | Self::LickWounds
            | Self::ProtectMe
            | Self::Revenge
            | Self::Solemnity
            | Self::ToxinTrickery
            | Self::Withstand => [0; 5],
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::InspiringCry
            | Self::WyrdReconstruction => panic!("atk() called for a non-attack skill"),
        };
        atk[lv]
    }

    const fn battle_limit(&self) -> Option<u8> {
        match self {
            Self::BarbaricYawp | Self::BlindingGas | Self::ProtectMe => Some(3),
            Self::EmboldeningVapours | Self::Solo => Some(2),
            Self::Bolster | Self::BulwarkOfFaith | Self::HeroicEnd | Self::Revenge | Self::TakeAim | Self::ToxinTrickery | Self::Withstand => Some(1),
            Self::AbyssalArtillery
            | Self::AdrenalineRush
            | Self::BattleBallad
            | Self::BattlefieldBandage
            | Self::BattleHeal
            | Self::BattlefieldMedicine
            | Self::Bellow
            | Self::Blackjack
            | Self::BleedOut
            | Self::Blindfire
            | Self::Bloodlet
            | Self::Bola
            | Self::Breakthru
            | Self::Chop
            | Self::CollectBounty
            | Self::ComeHither
            | Self::Command
            | Self::Cower
            | Self::Crush
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::Defender
            | Self::DirkStab
            | Self::DisorientingBlast
            | Self::DisruptiveCurse
            | Self::DivineGrace
            | Self::DuelistAdvance
            | Self::FesteringVapours
            | Self::FinishHim
            | Self::Flare
            | Self::Flashbang
            | Self::FlashingDaggers
            | Self::Flashpowder
            | Self::Focus
            | Self::FortifyingVapours
            | Self::GuardDog
            | Self::GodsComfort
            | Self::GodsHand
            | Self::GodsIllumination
            | Self::GrapeShotBlast
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::Hew
            | Self::HolyLance
            | Self::HookAndSlice
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::Howl
            | Self::IfItBleeds
            | Self::Incision
            | Self::InspiringCry
            | Self::InspiringTune
            | Self::Intimidate
            | Self::InvigoratingVapours
            | Self::IronSwan
            | Self::Judgement
            | Self::KrisStab
            | Self::LickWounds
            | Self::Lunge
            | Self::MaceBash
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::Pick
            | Self::PistolShot
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::PoisonDart
            | Self::Rampart
            | Self::Retribution
            | Self::ShadowFade
            | Self::SliceOff
            | Self::Smite
            | Self::SniperMark
            | Self::SniperShot
            | Self::Solemnity
            | Self::StunningBlow
            | Self::SuppressingFire
            | Self::TargetTag
            | Self::ThrownDagger
            | Self::Uppercut
            | Self::WeakeningCurse
            | Self::Whistle
            | Self::WickedHack
            | Self::WickedSlice
            | Self::WyrdReconstruction
            | Self::ZealousAccusation => None,
        }
    }

    const fn crit(&self, lv: usize) -> Percent10 {
        let crit = match self {
            Self::Bloodlet | Self::HandsFromAbyss => [90, 100, 110, 120, 130],
            Self::Lunge | Self::SliceOff | Self::ThrownDagger => [80, 90, 100, 110, 120],
            Self::PistolShot | Self::PoisonDart => [75, 85, 95, 105, 115],
            Self::CollectBounty => [70, 80, 90, 100, 110],
            Self::HolyLance => [65, 75, 85, 95, 105],
            Self::BleedOut => [60, 70, 80, 90, 100],
            Self::Crush
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::DirkStab
            | Self::DisruptiveCurse
            | Self::DuelistAdvance
            | Self::FinishHim
            | Self::HeroicEnd
            | Self::HookAndSlice
            | Self::HoundsRush
            | Self::Incision
            | Self::IronSwan
            | Self::Judgement
            | Self::Blackjack
            | Self::NoxiousBlast
            | Self::PointBlankShot
            | Self::Rampart
            | Self::SniperShot
            | Self::WeakeningCurse
            | Self::WickedSlice => [50, 60, 70, 80, 90],
            Self::WickedHack => [40, 50, 60, 70, 80],
            Self::Chop | Self::KrisStab => [30, 40, 50, 60, 70],
            Self::Retribution => [25, 35, 45, 55, 65],
            Self::Bola => [20, 30, 40, 50, 60],
            Self::GodsHand | Self::Pick => [10, 20, 30, 40, 50],
            Self::AbyssalArtillery
            | Self::Blindfire
            | Self::ComeHither
            | Self::FesteringVapours
            | Self::Focus
            | Self::Harvest
            | Self::IfItBleeds
            | Self::MaceBash
            | Self::OpenedVein
            | Self::PlagueGrenade
            | Self::Smite
            | Self::StunningBlow
            | Self::Uppercut => [0, 10, 20, 30, 40],
            Self::Breakthru => [-10, 0, 10, 20, 30],
            Self::TakeAim => [0, 0, 5, 5, 10],
            Self::AdrenalineRush
            | Self::BarbaricYawp
            | Self::BattleBallad
            | Self::Bellow
            | Self::BlindingGas
            | Self::Bolster
            | Self::BulwarkOfFaith
            | Self::Command
            | Self::Cower
            | Self::Defender
            | Self::DisorientingBlast
            | Self::EmboldeningVapours
            | Self::Flare
            | Self::Flashbang
            | Self::Flashpowder
            | Self::GodsIllumination
            | Self::GuardDog
            | Self::Howl
            | Self::InspiringTune
            | Self::Intimidate
            | Self::InvigoratingVapours
            | Self::LickWounds
            | Self::ProtectMe
            | Self::Revenge
            | Self::ShadowFade
            | Self::SniperMark
            | Self::Solemnity
            | Self::Solo
            | Self::TargetTag
            | Self::ToxinTrickery
            | Self::Whistle
            | Self::Withstand => [0; 5],
            Self::Hew | Self::ZealousAccusation => [-40, -30, -20, -10, 0],
            Self::FlashingDaggers | Self::HoundsHarry => [-50, -40, -30, -20, -10],
            Self::GrapeShotBlast => [-90, -80, -70, -60, -50],
            Self::SuppressingFire => [-100, -90, -80, -70, -60],
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::InspiringCry
            | Self::WyrdReconstruction => panic!("crit() called for a non-attack skill"),
        };
        crit[lv]
    }

    const fn dmg(&self) -> i8 {
        match self {
            Self::HeroicEnd | Self::PointBlankShot => 50,
            Self::Lunge => 40,
            Self::BleedOut => 20,
            Self::WickedSlice => 15,
            Self::AdrenalineRush
            | Self::BattleBallad
            | Self::Bloodlet
            | Self::Bolster
            | Self::BulwarkOfFaith
            | Self::Chop
            | Self::CollectBounty
            | Self::Command
            | Self::Crush
            | Self::Defender
            | Self::DirkStab
            | Self::EmboldeningVapours
            | Self::FinishHim
            | Self::GuardDog
            | Self::HolyLance
            | Self::HoundsRush
            | Self::Howl
            | Self::Incision
            | Self::InspiringTune
            | Self::Intimidate
            | Self::InvigoratingVapours
            | Self::IronSwan
            | Self::KrisStab
            | Self::LickWounds
            | Self::MaceBash
            | Self::ProtectMe
            | Self::Revenge
            | Self::Smite
            | Self::SniperShot
            | Self::Solemnity
            | Self::ToxinTrickery
            | Self::WickedHack
            | Self::Withstand => 0,
            Self::Blindfire | Self::ThrownDagger => -10,
            Self::OpenedVein | Self::Pick | Self::PistolShot => -15,
            Self::DuelistAdvance => -20,
            Self::Judgement => -25,
            Self::AbyssalArtillery | Self::FlashingDaggers | Self::SliceOff => -33,
            Self::IfItBleeds => -35,
            Self::Focus | Self::ZealousAccusation => -40,
            Self::Bola
            | Self::Breakthru
            | Self::DaemonsPull
            | Self::GodsHand
            | Self::GrapeShotBlast
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::Hew
            | Self::StunningBlow => -50,
            Self::PoisonDart | Self::Rampart => -60,
            Self::Blackjack => -65,
            Self::Uppercut => -67,
            Self::DazzlingLight | Self::FesteringVapours | Self::GodsIllumination | Self::HoundsHarry | Self::Retribution | Self::WeakeningCurse => {
                -75
            }
            Self::ComeHither | Self::NoxiousBlast | Self::SuppressingFire | Self::TakeAim => -80,
            Self::DisruptiveCurse | Self::PlagueGrenade => -90,
            Self::HookAndSlice => -95,
            Self::BarbaricYawp
            | Self::Bellow
            | Self::BlindingGas
            | Self::Cower
            | Self::DisorientingBlast
            | Self::Flare
            | Self::Flashbang
            | Self::Flashpowder
            | Self::ShadowFade
            | Self::SniperMark
            | Self::Solo
            | Self::TargetTag
            | Self::Whistle => -100,
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::InspiringCry
            | Self::WyrdReconstruction => panic!("dmg() called for a non-attack skill"),
        }
    }

    const fn effects(&self, lv: usize) -> &'static [Effect] {
        match self {
            Self::DirkStab => &[Effect::BuildToFinale(1)],
            Self::BattlefieldMedicine => &[Effect::Cure, Effect::Cureself],
            Self::Breakthru => &[Effect::HellionExhaustSm],
            Self::HeroicEnd => &[Effect::MortalWeakness, Effect::MortalWeaknessStress],
            Self::AbyssalArtillery => ABYSSAL_KILLER[lv],
            Self::AdrenalineRush => ADRENALINE_RUSH_EFFECTS[lv],
            Self::InvigoratingVapours => ANTIQ_DODGE[lv],
            Self::Blindfire => ARB_SELF_SPEED[lv],
            Self::BattlefieldBandage => ARB_STACKING_HEAL[lv],
            Self::BarbaricYawp => BARBARIC_YAWP_EFFECTS[lv],
            Self::BattleBallad => BATTLE_BALLAD_EFFECTS[lv],
            Self::Bellow => BELLOW_EFFECTS[lv],
            Self::IfItBleeds | Self::Incision => BLEED[lv],
            Self::BleedOut => BLEED_OUT_EFFECTS[lv],
            Self::Bola => BOLO_PUSH_1[lv],
            Self::Bolster => BOLSTER_EFFECTS[lv],
            Self::BulwarkOfFaith => BULWARK_OF_FAITH_EFFECTS[lv],
            Self::CollectBounty => COLLECT_BOUNTY_EFFECTS[lv],
            Self::ComeHither => COME_HITHER_EFFECTS[lv],
            Self::Command => COMMAND[lv],
            Self::Cower => COWER_EFFECTS[lv],
            Self::DaemonsPull => DAEMONS_PULL_EFFECTS[lv],
            Self::DazzlingLight => DAZZLING_LIGHT_EFFECTS[lv],
            Self::Defender => DEFENDER_EFFECTS[lv],
            Self::DisorientingBlast => DISORIENTING_BLAST_EFFECTS[lv],
            Self::DisruptiveCurse => DISRUPTIVE_CURSE_EFFECTS[lv],
            Self::Bloodlet => ELDRITCH_KILLER[lv],
            Self::FesteringVapours => FESTERING_VAPOURS_EFFECTS[lv],
            Self::Flare => FLARE_EFFECTS[lv],
            Self::Flashbang => FLASHBANG_EFFECTS[lv],
            Self::Flashpowder => FLASHPOWDER_EFFECTS[lv],
            Self::Focus => FOCUS_EFFECTS[lv],
            Self::FortifyingVapours => FORTIFY_RESISTS[lv],
            Self::GuardDog => GUARD_DOG_EFFECTS[lv],
            Self::GodsHand => GODS_HAND_EFFECTS[lv],
            Self::GodsIllumination => GODS_ILLUMINATION_EFFECTS[lv],
            Self::FlashingDaggers => GR_BLEED_DEBUFF[lv],
            Self::GrapeShotBlast => GRAPESHOT_VULNERABILITY[lv],
            Self::HandsFromAbyss => HANDS_FROM_ABYSS[lv],
            Self::HoundsHarry => HARRY_BLEED[lv],
            Self::Harvest => HARVEST_EFFECTS[lv],
            Self::Blackjack => HERO_STRONG_STUN[lv],
            Self::HookAndSlice => HOOK_AND_SLICE_EFFECTS[lv],
            Self::Howl => HOUND_HOWL[lv],
            Self::HoundsRush => HOUNDS_RUSH_EFFECTS[lv],
            Self::PistolShot => HW_PISTOL_DMG_MARKED[lv],
            Self::DuelistAdvance => HWY_RIPOSTE[lv],
            Self::InspiringCry => INSPIRING_CRY_EFFECTS[lv],
            Self::InspiringTune => INSPIRING_TUNE_EFFECTS[lv],
            Self::Intimidate => INTIMIDATE_EFFECTS[lv],
            Self::LickWounds => LICK_WOUNDS[lv],
            Self::NoxiousBlast => NOXIOUS_BLAST_EFFECTS[lv],
            Self::OpenedVein => OPENED_VEIN_EFFECTS[lv],
            Self::PlagueGrenade => PD_BLIGHT[lv],
            Self::EmboldeningVapours => PD_VAPOURS_BUFF[lv],
            Self::PoisonDart => POISON_DART_EFFECTS[lv],
            Self::Lunge => POISON_KILLER[lv],
            Self::ProtectMe => PROTECT_ME_EFFECTS[lv],
            Self::PointBlankShot => PUSH_1[lv],
            Self::Rampart => RAMPART_EFFECTS[lv],
            Self::Retribution => RETRIBUTION_EFFECTS[lv],
            Self::Revenge => REVENGE_EFFECTS[lv],
            Self::ShadowFade => SHADOW_FADE_EFFECTS[lv],
            Self::SliceOff => SLICE_OFF_EFFECTS[lv],
            Self::SniperShot => SNIPER_DAMAGE[lv],
            Self::SniperMark => SNIPER_MARK_EFFECTS[lv],
            Self::Solemnity => SOLEMNITY_EFFECTS[lv],
            Self::Solo => SOLO_EFFECTS[lv],
            Self::BlindingGas | Self::StunningBlow => STUN[lv],
            Self::FinishHim => STUN_KILLER[lv],
            Self::SuppressingFire => SUPPRESSION[lv],
            Self::TakeAim => TAKE_AIM_EFFECTS[lv],
            Self::TargetTag => TARGET_TAG_EFFECTS[lv],
            Self::ThrownDagger => THROWN_DAGGER_EFFECTS[lv],
            Self::ToxinTrickery => TOXIN_TRICKERY_EFFECTS[lv],
            Self::HolyLance | Self::MaceBash | Self::Smite => UNHOLY_KILLER[lv],
            Self::Uppercut => UPPERCUT_EFFECTS[lv],
            Self::Judgement => VESTAL_HEALSELF[lv],
            Self::WeakeningCurse => WEAKENING_CURSE_EFFECTS[lv],
            Self::Whistle => WHISTLE_EFFECTS[lv],
            Self::Withstand => WITHSTAND_EFFECTS[lv],
            Self::WyrdReconstruction => WYRD_BLEED[lv],
            Self::BattleHeal
            | Self::Chop
            | Self::Crush
            | Self::DivineGrace
            | Self::GodsComfort
            | Self::Hew
            | Self::IronSwan
            | Self::KrisStab
            | Self::Pick
            | Self::WickedHack
            | Self::WickedSlice
            | Self::ZealousAccusation => &[],
        }
    }

    const fn heal(&self, lv: usize) -> (u8, u8) {
        let heal = match self {
            Self::WyrdReconstruction => [(0, 13), (0, 15), (0, 17), (0, 19), (0, 22)],
            Self::DivineGrace => [(4, 5), (5, 6), (6, 7), (7, 8), (8, 9)],
            Self::BattleHeal => [(2, 3), (3, 3), (3, 4), (4, 5), (5, 6)],
            Self::BattlefieldBandage => [(2, 3), (3, 3), (3, 4), (4, 4), (4, 5)],
            Self::GodsComfort => [(1, 3), (2, 3), (3, 3), (3, 4), (4, 5)],
            Self::BattlefieldMedicine | Self::FortifyingVapours => [(1, 1), (1, 2), (2, 2), (2, 3), (3, 3)],
            Self::InspiringCry => [(1, 1), (1, 1), (1, 2), (1, 2), (2, 2)],
            Self::AbyssalArtillery
            | Self::AdrenalineRush
            | Self::BarbaricYawp
            | Self::BattleBallad
            | Self::Bellow
            | Self::Blackjack
            | Self::BleedOut
            | Self::Blindfire
            | Self::BlindingGas
            | Self::Bloodlet
            | Self::Bola
            | Self::Bolster
            | Self::Breakthru
            | Self::BulwarkOfFaith
            | Self::Chop
            | Self::CollectBounty
            | Self::ComeHither
            | Self::Command
            | Self::Cower
            | Self::Crush
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::Defender
            | Self::DirkStab
            | Self::DisorientingBlast
            | Self::DisruptiveCurse
            | Self::DuelistAdvance
            | Self::EmboldeningVapours
            | Self::FesteringVapours
            | Self::FinishHim
            | Self::Flare
            | Self::Flashbang
            | Self::FlashingDaggers
            | Self::Flashpowder
            | Self::Focus
            | Self::GodsHand
            | Self::GodsIllumination
            | Self::GrapeShotBlast
            | Self::GuardDog
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::HeroicEnd
            | Self::Hew
            | Self::HolyLance
            | Self::HookAndSlice
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::Howl
            | Self::IfItBleeds
            | Self::Incision
            | Self::InspiringTune
            | Self::Intimidate
            | Self::InvigoratingVapours
            | Self::IronSwan
            | Self::Judgement
            | Self::KrisStab
            | Self::LickWounds
            | Self::Lunge
            | Self::MaceBash
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::Pick
            | Self::PistolShot
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::PoisonDart
            | Self::ProtectMe
            | Self::Rampart
            | Self::Retribution
            | Self::Revenge
            | Self::ShadowFade
            | Self::Smite
            | Self::SniperMark
            | Self::SniperShot
            | Self::Solemnity
            | Self::Solo
            | Self::StunningBlow
            | Self::SuppressingFire
            | Self::TakeAim
            | Self::TargetTag
            | Self::ThrownDagger
            | Self::ToxinTrickery
            | Self::Uppercut
            | Self::WeakeningCurse
            | Self::Whistle
            | Self::WickedHack
            | Self::WickedSlice
            | Self::Withstand
            | Self::ZealousAccusation => panic!("heal() called for a non-healing skill"),
        };
        heal[lv]
    }

    const fn ignore_guard(&self) -> bool {
        if matches!(self, Self::DirkStab) { true } else { false }
    }

    const fn ignore_protection(&self) -> bool {
        if matches!(self, Self::Pick) { true } else { false }
    }

    const fn ignore_stealth(&self) -> bool {
        if matches!(
            self,
            Self::Flare | Self::Flashpowder | Self::GodsIllumination | Self::Intimidate | Self::TakeAim
        ) {
            true
        } else {
            false
        }
    }

    const fn is_crit_valid(&self) -> bool {
        match self {
            Self::AbyssalArtillery
            | Self::AdrenalineRush
            | Self::BattleBallad
            | Self::Blackjack
            | Self::BleedOut
            | Self::Blindfire
            | Self::Bloodlet
            | Self::Bola
            | Self::Bolster
            | Self::Breakthru
            | Self::BulwarkOfFaith
            | Self::Chop
            | Self::CollectBounty
            | Self::ComeHither
            | Self::Command
            | Self::Cower
            | Self::Crush
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::Defender
            | Self::DirkStab
            | Self::DisruptiveCurse
            | Self::DuelistAdvance
            | Self::EmboldeningVapours
            | Self::FesteringVapours
            | Self::FinishHim
            | Self::FlashingDaggers
            | Self::Focus
            | Self::GodsHand
            | Self::GrapeShotBlast
            | Self::GuardDog
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::HeroicEnd
            | Self::Hew
            | Self::HolyLance
            | Self::HookAndSlice
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::Howl
            | Self::IfItBleeds
            | Self::Incision
            | Self::InspiringTune
            | Self::Intimidate
            | Self::InvigoratingVapours
            | Self::IronSwan
            | Self::Judgement
            | Self::KrisStab
            | Self::LickWounds
            | Self::Lunge
            | Self::MaceBash
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::Pick
            | Self::PistolShot
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::PoisonDart
            | Self::ProtectMe
            | Self::Rampart
            | Self::Retribution
            | Self::Revenge
            | Self::SliceOff
            | Self::Smite
            | Self::SniperShot
            | Self::StunningBlow
            | Self::SuppressingFire
            | Self::TakeAim
            | Self::ThrownDagger
            | Self::ToxinTrickery
            | Self::Uppercut
            | Self::WeakeningCurse
            | Self::WickedHack
            | Self::WickedSlice
            | Self::Withstand
            | Self::ZealousAccusation => true,
            Self::BarbaricYawp
            | Self::Bellow
            | Self::BlindingGas
            | Self::DisorientingBlast
            | Self::Flare
            | Self::Flashbang
            | Self::Flashpowder
            | Self::GodsIllumination
            | Self::ShadowFade
            | Self::SniperMark
            | Self::Solo
            | Self::TargetTag
            | Self::Whistle => false,
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::InspiringCry
            | Self::WyrdReconstruction => panic!("is_crit_valid() called for a non-attack skill"),
        }
    }

    const fn is_stall_invalidating(&self) -> bool {
        match self {
            Self::AbyssalArtillery
            | Self::Blackjack
            | Self::BleedOut
            | Self::Blindfire
            | Self::Bloodlet
            | Self::Bola
            | Self::Breakthru
            | Self::Chop
            | Self::CollectBounty
            | Self::ComeHither
            | Self::Crush
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::DirkStab
            | Self::DuelistAdvance
            | Self::EmboldeningVapours
            | Self::FesteringVapours
            | Self::FinishHim
            | Self::FlashingDaggers
            | Self::Focus
            | Self::GodsHand
            | Self::GodsIllumination
            | Self::GrapeShotBlast
            | Self::GuardDog
            | Self::HandsFromAbyss
            | Self::Harvest
            | Self::HeroicEnd
            | Self::Hew
            | Self::HolyLance
            | Self::HookAndSlice
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::IfItBleeds
            | Self::Incision
            | Self::Intimidate
            | Self::IronSwan
            | Self::Judgement
            | Self::KrisStab
            | Self::Lunge
            | Self::MaceBash
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::Pick
            | Self::PistolShot
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::Rampart
            | Self::Retribution
            | Self::Revenge
            | Self::PoisonDart
            | Self::SliceOff
            | Self::Smite
            | Self::SniperShot
            | Self::Solo
            | Self::StunningBlow
            | Self::SuppressingFire
            | Self::TakeAim
            | Self::ThrownDagger
            | Self::Uppercut
            | Self::WeakeningCurse
            | Self::WickedHack
            | Self::WickedSlice
            | Self::ZealousAccusation => true,
            Self::AdrenalineRush
            | Self::BarbaricYawp
            | Self::BattleBallad
            | Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::Bellow
            | Self::BlindingGas
            | Self::Bolster
            | Self::BulwarkOfFaith
            | Self::Command
            | Self::Cower
            | Self::Defender
            | Self::DisorientingBlast
            | Self::DisruptiveCurse
            | Self::DivineGrace
            | Self::Flare
            | Self::Flashbang
            | Self::Flashpowder
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::Howl
            | Self::InspiringCry
            | Self::InspiringTune
            | Self::InvigoratingVapours
            | Self::LickWounds
            | Self::ProtectMe
            | Self::ShadowFade
            | Self::SniperMark
            | Self::Solemnity
            | Self::TargetTag
            | Self::ToxinTrickery
            | Self::Whistle
            | Self::Withstand
            | Self::WyrdReconstruction => false,
        }
    }

    const fn launch(&self) -> &'static [Rank] {
        match self {
            Self::AdrenalineRush
            | Self::BattleHeal
            | Self::Bellow
            | Self::Blindfire
            | Self::Bolster
            | Self::ComeHither
            | Self::Command
            | Self::Cower
            | Self::Defender
            | Self::DirkStab
            | Self::DisruptiveCurse
            | Self::EmboldeningVapours
            | Self::FesteringVapours
            | Self::Flare
            | Self::Flashpowder
            | Self::GuardDog
            | Self::HoundsHarry
            | Self::InspiringCry
            | Self::KrisStab
            | Self::ProtectMe
            | Self::Revenge
            | Self::TakeAim
            | Self::TargetTag
            | Self::ToxinTrickery
            | Self::WeakeningCurse
            | Self::Whistle
            | Self::WyrdReconstruction => &[Rank::Four, Rank::Three, Rank::Two, Rank::One],
            Self::Breakthru
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::DisorientingBlast
            | Self::DuelistAdvance
            | Self::Flashbang
            | Self::FlashingDaggers
            | Self::GodsComfort
            | Self::HookAndSlice
            | Self::HoundsRush
            | Self::LickWounds
            | Self::NoxiousBlast
            | Self::PistolShot
            | Self::PoisonDart
            | Self::ThrownDagger => &[Rank::Four, Rank::Three, Rank::Two],
            Self::Bloodlet
            | Self::CollectBounty
            | Self::FinishHim
            | Self::GodsIllumination
            | Self::IfItBleeds
            | Self::Incision
            | Self::OpenedVein
            | Self::Pick
            | Self::Rampart
            | Self::Retribution
            | Self::WickedSlice
            | Self::Withstand => &[Rank::Three, Rank::Two, Rank::One],
            Self::AbyssalArtillery
            | Self::BattleBallad
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::BlindingGas
            | Self::Bola
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::HolyLance
            | Self::Howl
            | Self::InspiringTune
            | Self::InvigoratingVapours
            | Self::Judgement
            | Self::Lunge
            | Self::PlagueGrenade
            | Self::SniperMark
            | Self::SniperShot
            | Self::Solo
            | Self::SuppressingFire => &[Rank::Four, Rank::Three],
            Self::GrapeShotBlast | Self::Harvest | Self::SliceOff => &[Rank::Three, Rank::Two],
            Self::BarbaricYawp
            | Self::Blackjack
            | Self::BulwarkOfFaith
            | Self::Chop
            | Self::GodsHand
            | Self::Crush
            | Self::HandsFromAbyss
            | Self::Hew
            | Self::HeroicEnd
            | Self::MaceBash
            | Self::ShadowFade
            | Self::Smite
            | Self::Solemnity
            | Self::StunningBlow
            | Self::Uppercut
            | Self::WickedHack
            | Self::ZealousAccusation => &[Rank::Two, Rank::One],
            Self::BleedOut | Self::Focus | Self::Intimidate | Self::IronSwan | Self::PointBlankShot => &[Rank::One],
        }
    }

    const fn movement(&self) -> (u8, u8) {
        if matches!(self, Self::HeroicEnd) {
            return (3, 0);
        }
        if matches!(self, Self::Cower | Self::ShadowFade) {
            return (2, 0);
        }
        if matches!(self, Self::PointBlankShot) {
            return (1, 0);
        }
        if matches!(self, Self::Solo) {
            return (0, 3);
        }
        if matches!(self, Self::Lunge) {
            return (0, 2);
        }
        if matches!(
            self,
            Self::Breakthru | Self::DirkStab | Self::DuelistAdvance | Self::HolyLance | Self::Rampart
        ) {
            return (0, 1);
        }
        (0, 0)
    }

    const fn skill_type(&self) -> SkillType {
        match self {
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::FortifyingVapours
            | Self::GodsComfort
            | Self::InspiringCry
            | Self::WyrdReconstruction => SkillType::Heal,
            Self::AdrenalineRush
            | Self::BarbaricYawp
            | Self::Blackjack
            | Self::BleedOut
            | Self::Bloodlet
            | Self::Breakthru
            | Self::BulwarkOfFaith
            | Self::Chop
            | Self::CollectBounty
            | Self::Crush
            | Self::Defender
            | Self::DirkStab
            | Self::DuelistAdvance
            | Self::EmboldeningVapours
            | Self::FinishHim
            | Self::Focus
            | Self::GuardDog
            | Self::Harvest
            | Self::HeroicEnd
            | Self::Hew
            | Self::HolyLance
            | Self::IfItBleeds
            | Self::Incision
            | Self::Intimidate
            | Self::IronSwan
            | Self::KrisStab
            | Self::LickWounds
            | Self::Lunge
            | Self::MaceBash
            | Self::OpenedVein
            | Self::Pick
            | Self::Rampart
            | Self::Retribution
            | Self::Revenge
            | Self::ShadowFade
            | Self::SliceOff
            | Self::Smite
            | Self::Solemnity
            | Self::StunningBlow
            | Self::ToxinTrickery
            | Self::Uppercut
            | Self::WickedHack
            | Self::WickedSlice
            | Self::Withstand => SkillType::Melee,
            Self::AbyssalArtillery
            | Self::BattleBallad
            | Self::Bellow
            | Self::Blindfire
            | Self::BlindingGas
            | Self::Bola
            | Self::Bolster
            | Self::ComeHither
            | Self::Command
            | Self::Cower
            | Self::DaemonsPull
            | Self::DazzlingLight
            | Self::DisorientingBlast
            | Self::DisruptiveCurse
            | Self::FesteringVapours
            | Self::Flare
            | Self::Flashbang
            | Self::FlashingDaggers
            | Self::Flashpowder
            | Self::GodsHand
            | Self::GodsIllumination
            | Self::GrapeShotBlast
            | Self::HandsFromAbyss
            | Self::HookAndSlice
            | Self::HoundsHarry
            | Self::HoundsRush
            | Self::Howl
            | Self::InspiringTune
            | Self::InvigoratingVapours
            | Self::Judgement
            | Self::NoxiousBlast
            | Self::PistolShot
            | Self::PlagueGrenade
            | Self::PointBlankShot
            | Self::PoisonDart
            | Self::ProtectMe
            | Self::SniperMark
            | Self::SniperShot
            | Self::Solo
            | Self::SuppressingFire
            | Self::TakeAim
            | Self::TargetTag
            | Self::ThrownDagger
            | Self::WeakeningCurse
            | Self::Whistle
            | Self::ZealousAccusation => SkillType::Ranged,
        }
    }

    const fn target(&self) -> (TargetMod, &'static [Rank]) {
        match self {
            Self::Bellow | Self::Flare | Self::HoundsHarry | Self::Solo => (TargetMod::Multi, &[Rank::One, Rank::Two, Rank::Three, Rank::Four]),
            Self::Breakthru | Self::GrapeShotBlast => (TargetMod::Multi, &[Rank::One, Rank::Two, Rank::Three]),
            Self::BarbaricYawp | Self::Bola | Self::Hew | Self::ZealousAccusation => (TargetMod::Multi, &[Rank::One, Rank::Two]),
            Self::FlashingDaggers | Self::Harvest => (TargetMod::Multi, &[Rank::Two, Rank::Three]),
            Self::AbyssalArtillery | Self::BlindingGas | Self::PlagueGrenade | Self::SuppressingFire => {
                (TargetMod::Multi, &[Rank::Three, Rank::Four])
            }
            Self::DisruptiveCurse
            | Self::FesteringVapours
            | Self::Flashpowder
            | Self::GodsIllumination
            | Self::HeroicEnd
            | Self::HoundsRush
            | Self::Intimidate
            | Self::Judgement
            | Self::PoisonDart
            | Self::TargetTag
            | Self::Whistle => (TargetMod::Single, &[Rank::One, Rank::Two, Rank::Three, Rank::Four]),
            Self::Blackjack
            | Self::Bloodlet
            | Self::Crush
            | Self::DazzlingLight
            | Self::DirkStab
            | Self::DuelistAdvance
            | Self::FinishHim
            | Self::GodsHand
            | Self::HandsFromAbyss
            | Self::KrisStab
            | Self::Lunge
            | Self::Retribution
            | Self::WeakeningCurse => (TargetMod::Single, &[Rank::One, Rank::Two, Rank::Three]),
            Self::DisorientingBlast
            | Self::Flashbang
            | Self::HolyLance
            | Self::PistolShot
            | Self::SniperMark
            | Self::SniperShot
            | Self::TakeAim
            | Self::ThrownDagger => (TargetMod::Single, &[Rank::Two, Rank::Three, Rank::Four]),
            Self::Chop
            | Self::CollectBounty
            | Self::Incision
            | Self::MaceBash
            | Self::NoxiousBlast
            | Self::OpenedVein
            | Self::Pick
            | Self::Rampart
            | Self::Smite
            | Self::StunningBlow
            | Self::Uppercut
            | Self::WickedHack
            | Self::WickedSlice => (TargetMod::Single, &[Rank::One, Rank::Two]),
            Self::IfItBleeds | Self::SliceOff => (TargetMod::Single, &[Rank::Two, Rank::Three]),
            Self::ComeHither | Self::DaemonsPull | Self::HookAndSlice => (TargetMod::Single, &[Rank::Three, Rank::Four]),
            Self::BleedOut | Self::Focus | Self::PointBlankShot => (TargetMod::Single, &[Rank::One]),
            Self::IronSwan => (TargetMod::Single, &[Rank::Four]),
            Self::Blindfire => (TargetMod::Random, &[Rank::One, Rank::Two, Rank::Three, Rank::Four]),
            Self::BattleBallad | Self::Bolster | Self::Command | Self::GodsComfort | Self::Howl | Self::InvigoratingVapours => {
                (TargetMod::AlliesMulti, &[Rank::One, Rank::Two, Rank::Three, Rank::Four])
            }
            Self::BattleHeal
            | Self::BattlefieldBandage
            | Self::BattlefieldMedicine
            | Self::DivineGrace
            | Self::EmboldeningVapours
            | Self::FortifyingVapours
            | Self::InspiringCry
            | Self::InspiringTune
            | Self::WyrdReconstruction => (TargetMod::Allies, &[Rank::One, Rank::Two, Rank::Three, Rank::Four]),
            Self::Defender | Self::GuardDog | Self::ProtectMe => (TargetMod::OtherAllies, &[Rank::One, Rank::Two, Rank::Three, Rank::Four]),
            Self::AdrenalineRush
            | Self::BulwarkOfFaith
            | Self::Cower
            | Self::LickWounds
            | Self::Revenge
            | Self::ShadowFade
            | Self::Solemnity
            | Self::ToxinTrickery
            | Self::Withstand => (TargetMod::Performer, &[]),
        }
    }
}
