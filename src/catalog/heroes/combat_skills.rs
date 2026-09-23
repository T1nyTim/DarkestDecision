use crate::catalog::{
    Rank,
    effect::{
        consts::{
            ABSOLUTION_EFFECTS, ABYSSAL_KILLER, ADRENALINE_RUSH_EFFECTS, ANTIQ_DODGE, ARB_SELF_SPEED, ARB_STACKING_HEAL, BARBARIC_YAWP_EFFECTS,
            BATTLE_BALLAD_EFFECTS, BELLOW_EFFECTS, BLEED, BLEED_OUT_EFFECTS, BOLO_PUSH_1, BOLSTER_EFFECTS, BULWARK_OF_FAITH_EFFECTS,
            COLLECT_BOUNTY_EFFECTS, COME_HITHER_EFFECTS, COMMAND, COWER_EFFECTS, DAEMONS_PULL_EFFECTS, DAZZLING_LIGHT_EFFECTS, DEFENDER_EFFECTS,
            DISORIENTING_BLAST_EFFECTS, DISRUPTIVE_CURSE_EFFECTS, ELDRITCH_KILLER, FESTERING_VAPOURS_EFFECTS, FLARE_EFFECTS, FLASHBANG_EFFECTS,
            FLASHPOWDER_EFFECTS, FOCUS_EFFECTS, FORTIFY_RESISTS, GODS_HAND_EFFECTS, GODS_ILLUMINATION_EFFECTS, GR_BLEED_DEBUFF,
            GRAPESHOT_VULNERABILITY, GUARD_DOG_EFFECTS, HANDS_FROM_ABYSS, HARRY_BLEED, HARVEST_EFFECTS, HERO_STRONG_STUN, HOOK_AND_SLICE_EFFECTS,
            HOUND_HOWL, HOUNDS_RUSH_EFFECTS, HW_PISTOL_DMG_MARKED, HWY_RIPOSTE, INSPIRING_CRY_EFFECTS, INSPIRING_TUNE_EFFECTS, INTIMIDATE_EFFECTS,
            LICK_WOUNDS, MANACLES_STUN, NOXIOUS_BLAST_EFFECTS, OPENED_VEIN_EFFECTS, PD_BLIGHT, PD_VAPOURS_BUFF, POISON_DART_EFFECTS, POISON_KILLER,
            PROTECT_ME_EFFECTS, PUSH_1, RAKEBUFF, RAMPART_EFFECTS, RETRIBUTION_EFFECTS, REVENGE_EFFECTS, SHADOW_FADE_EFFECTS, SLAM_EFFECTS,
            SLICE_OFF_EFFECTS, SNIPER_DAMAGE, SNIPER_MARK_EFFECTS, SOLEMNITY_EFFECTS, SOLO_EFFECTS, STUN, STUN_KILLER, SUPPRESSION, TAKE_AIM_EFFECTS,
            TARGET_TAG_EFFECTS, THROWN_DAGGER_EFFECTS, TOXIN_TRICKERY_EFFECTS, TRANSFORM_BEAST_EFFECTS, TRANSFORM_HUMAN_EFFECTS, UNHOLY_KILLER,
            UPPERCUT_EFFECTS, VESTAL_HEALSELF, VOMIT_EFFECTS, WEAKENING_CURSE_EFFECTS, WHISTLE_EFFECTS, WITHSTAND_EFFECTS, WYRD_BLEED,
        },
        effect::Effect,
    },
    heroes::{AttackData, CombatSkillData, Mode, SkillData, TargetMod, combat_skill::CombatSkill, shared_effects},
};

const R1: &'static [Rank] = &[Rank::One];
const R12: &'static [Rank] = &[Rank::One, Rank::Two];
const R123: &'static [Rank] = &[Rank::One, Rank::Two, Rank::Three];
const R1234: &'static [Rank] = &[Rank::One, Rank::Two, Rank::Three, Rank::Four];
const R21: &'static [Rank] = &[Rank::Two, Rank::One];
const R23: &'static [Rank] = &[Rank::Two, Rank::Three];
const R234: &'static [Rank] = &[Rank::Two, Rank::Three, Rank::Four];
const R32: &'static [Rank] = &[Rank::Three, Rank::Two];
const R321: &'static [Rank] = &[Rank::Three, Rank::Two, Rank::One];
const R34: &'static [Rank] = &[Rank::Three, Rank::Four];
const R4: &'static [Rank] = &[Rank::Four];
const R43: &'static [Rank] = &[Rank::Four, Rank::Three];
const R432: &'static [Rank] = &[Rank::Four, Rank::Three, Rank::Two];
const R4321: &'static [Rank] = &[Rank::Four, Rank::Three, Rank::Two, Rank::One];

pub const COMBAT_SKILLS: &[CombatSkillData] = &[
    CombatSkillData::new(
        CombatSkill::Absolution,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_effects(ABSOLUTION_EFFECTS)
    .with_modes(&[Mode::Human])
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::AbyssalArtillery,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-33; 5], [0, 10, 20, 30, 40], true)),
        R43,
        TargetMod::Multi(R34),
    )
    .with_effects(ABYSSAL_KILLER),
    CombatSkillData::new(
        CombatSkill::AdrenalineRush,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_effects(ADRENALINE_RUSH_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::BarbaricYawp,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R21,
        TargetMod::Multi(R12),
    )
    .with_battle_limit(3)
    .with_effects(BARBARIC_YAWP_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::BattleBallad,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R43,
        TargetMod::AlliesMulti,
    )
    .with_effects(BATTLE_BALLAD_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::BattlefieldBandage,
        SkillData::Heal([(2, 3), (3, 3), (3, 4), (4, 4), (4, 5)]),
        R43,
        TargetMod::Allies,
    )
    .with_effects(ARB_STACKING_HEAL)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::BattlefieldMedicine,
        SkillData::Heal([(1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]),
        R43,
        TargetMod::Allies,
    )
    .with_effects(shared_effects(&[Effect::Cure, Effect::CureSelf]))
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::BattleHeal,
        SkillData::Heal([(2, 3), (3, 3), (3, 4), (4, 5), (5, 6)]),
        R4321,
        TargetMod::Allies,
    )
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Bellow,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-100; 5], [0; 5], false)),
        R4321,
        TargetMod::Multi(R1234),
    )
    .with_effects(BELLOW_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Blackjack,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-65; 5], [50, 60, 70, 80, 90], true)),
        R21,
        TargetMod::Single(R123),
    )
    .with_effects(HERO_STRONG_STUN),
    CombatSkillData::new(
        CombatSkill::BleedOut,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [20; 5], [60, 70, 80, 90, 100], true)),
        R1,
        TargetMod::Single(R1),
    )
    .with_effects(BLEED_OUT_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Blindfire,
        SkillData::Ranged(AttackData::new([75, 80, 85, 90, 95], [-10; 5], [0, 10, 20, 30, 40], true)),
        R4321,
        TargetMod::Random,
    )
    .with_effects(ARB_SELF_SPEED),
    CombatSkillData::new(
        CombatSkill::BlindingGas,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R43,
        TargetMod::Multi(R34),
    )
    .with_battle_limit(3)
    .with_effects(STUN)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Bloodlet,
        SkillData::Melee(AttackData::new([80, 85, 90, 95, 100], [0; 5], [90, 100, 110, 120, 130], true)),
        R321,
        TargetMod::Single(R123),
    )
    .with_effects(ELDRITCH_KILLER),
    CombatSkillData::new(
        CombatSkill::Bola,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-50; 5], [20, 30, 40, 50, 60], true)),
        R43,
        TargetMod::Multi(R12),
    )
    .with_effects(BOLO_PUSH_1),
    CombatSkillData::new(
        CombatSkill::Bolster,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::AlliesMulti,
    )
    .with_battle_limit(1)
    .with_effects(BOLSTER_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Breakthru,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [-50; 5], [-10, 0, 10, 20, 30], true)),
        R432,
        TargetMod::Multi(R123),
    )
    .with_movement((0, 1))
    .with_effects(shared_effects(&[Effect::HellionExhaustSm])),
    CombatSkillData::new(
        CombatSkill::BulwarkOfFaith,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R21,
        TargetMod::Performer,
    )
    .with_battle_limit(1)
    .with_effects(BULWARK_OF_FAITH_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Chop,
        SkillData::Melee(AttackData::new([75, 80, 85, 90, 95], [0; 5], [30, 40, 50, 60, 70], true)),
        R21,
        TargetMod::Single(R12),
    ),
    CombatSkillData::new(
        CombatSkill::Crush,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [-0; 5], [50, 60, 70, 80, 90], true)),
        R12,
        TargetMod::Single(R123),
    ),
    CombatSkillData::new(
        CombatSkill::CollectBounty,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [70, 80, 90, 100, 110], true)),
        R321,
        TargetMod::Single(R12),
    )
    .with_effects(COLLECT_BOUNTY_EFFECTS),
    CombatSkillData::new(
        CombatSkill::ComeHither,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-80; 5], [0, 10, 20, 30, 40], true)),
        R4321,
        TargetMod::Single(R34),
    )
    .with_effects(COME_HITHER_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Command,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::AlliesMulti,
    )
    .with_effects(COMMAND)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Cower,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_movement((2, 0))
    .with_effects(COWER_EFFECTS),
    CombatSkillData::new(
        CombatSkill::DaemonsPull,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-50; 5], [50, 60, 70, 80, 90], true)),
        R432,
        TargetMod::Single(R34),
    )
    .with_effects(DAEMONS_PULL_EFFECTS),
    CombatSkillData::new(
        CombatSkill::DazzlingLight,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-75; 5], [50, 60, 70, 80, 90], true)),
        R432,
        TargetMod::Single(R123),
    )
    .with_effects(DAZZLING_LIGHT_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Defender,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Allies,
    )
    .with_effects(DEFENDER_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::DirkStab,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [50, 60, 70, 80, 90], true)),
        R4321,
        TargetMod::Single(R123),
    )
    .with_movement((0, 1))
    .ignore_guard()
    .with_effects(shared_effects(&[Effect::BuildToFinale(1)])),
    CombatSkillData::new(
        CombatSkill::DisorientingBlast,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R432,
        TargetMod::Single(R234),
    )
    .with_effects(DISORIENTING_BLAST_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::DisruptiveCurse,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-90; 5], [50, 60, 70, 80, 90], true)),
        R4321,
        TargetMod::Single(R1234),
    )
    .with_effects(DISRUPTIVE_CURSE_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::DivineGrace,
        SkillData::Heal([(4, 5), (5, 6), (6, 7), (7, 8), (8, 9)]),
        R43,
        TargetMod::Allies,
    )
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::DuelistAdvance,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-20; 5], [50, 60, 70, 80, 90], true)),
        R432,
        TargetMod::Single(R123),
    )
    .with_movement((0, 1))
    .with_effects(HWY_RIPOSTE),
    CombatSkillData::new(
        CombatSkill::EmboldeningVapours,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Allies,
    )
    .with_battle_limit(2)
    .with_effects(PD_VAPOURS_BUFF),
    CombatSkillData::new(
        CombatSkill::FesteringVapours,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-75; 5], [0, 10, 20, 30, 40], true)),
        R4321,
        TargetMod::Single(R1234),
    )
    .with_effects(FESTERING_VAPOURS_EFFECTS),
    CombatSkillData::new(
        CombatSkill::FinishHim,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [50, 60, 70, 80, 90], true)),
        R321,
        TargetMod::Single(R123),
    )
    .with_effects(STUN_KILLER),
    CombatSkillData::new(
        CombatSkill::Flare,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R4321,
        TargetMod::Multi(R1234),
    )
    .ignore_stealth()
    .with_effects(FLARE_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Flashbang,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R432,
        TargetMod::Single(R234),
    )
    .with_effects(FLASHBANG_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::FlashingDaggers,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-33; 5], [-50, -40, -30, -20, -10], true)),
        R432,
        TargetMod::Multi(R23),
    )
    .with_effects(GR_BLEED_DEBUFF),
    CombatSkillData::new(
        CombatSkill::Flashpowder,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R4321,
        TargetMod::Single(R1234),
    )
    .ignore_stealth()
    .with_effects(FLASHPOWDER_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Focus,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [-40; 5], [0, 10, 20, 30, 40], true)),
        R1,
        TargetMod::Single(R1),
    )
    .with_effects(FOCUS_EFFECTS),
    CombatSkillData::new(
        CombatSkill::FortifyingVapours,
        SkillData::Heal([(1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]),
        R43,
        TargetMod::Allies,
    )
    .with_effects(FORTIFY_RESISTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::GodsComfort,
        SkillData::Heal([(1, 3), (2, 3), (3, 3), (3, 4), (4, 5)]),
        R432,
        TargetMod::AlliesMulti,
    )
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::GodsHand,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-50; 5], [10, 20, 30, 40, 50], true)),
        R12,
        TargetMod::Single(R123),
    )
    .with_effects(GODS_HAND_EFFECTS),
    CombatSkillData::new(
        CombatSkill::GodsIllumination,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-75; 5], [0; 5], false)),
        R321,
        TargetMod::Single(R1234),
    )
    .ignore_stealth()
    .with_effects(GODS_ILLUMINATION_EFFECTS),
    CombatSkillData::new(
        CombatSkill::GrapeShotBlast,
        SkillData::Ranged(AttackData::new([75, 80, 85, 90, 95], [-50; 5], [-90, -80, -70, -60, -50], true)),
        R32,
        TargetMod::Multi(R123),
    )
    .with_effects(GRAPESHOT_VULNERABILITY),
    CombatSkillData::new(
        CombatSkill::GuardDog,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Allies,
    )
    .with_effects(GUARD_DOG_EFFECTS),
    CombatSkillData::new(
        CombatSkill::HandsFromAbyss,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-50; 5], [90, 100, 110, 120, 130], true)),
        R21,
        TargetMod::Single(R123),
    )
    .with_effects(HANDS_FROM_ABYSS),
    CombatSkillData::new(
        CombatSkill::Harvest,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-50; 5], [0, 10, 20, 30, 40], true)),
        R32,
        TargetMod::Multi(R23),
    )
    .with_effects(HARVEST_EFFECTS),
    CombatSkillData::new(
        CombatSkill::HeroicEnd,
        SkillData::Melee(AttackData::new([140, 145, 150, 155, 160], [50; 5], [50, 60, 70, 80, 90], true)),
        R21,
        TargetMod::Single(R1234),
    )
    .with_movement((3, 0))
    .with_battle_limit(1)
    .with_effects(shared_effects(&[Effect::MortalWeakness, Effect::MortalWeaknessStress])),
    CombatSkillData::new(
        CombatSkill::Hew,
        SkillData::Melee(AttackData::new([75, 80, 85, 90, 95], [-50; 5], [-40, -30, -20, -10, 0], true)),
        R21,
        TargetMod::Multi(R12),
    ),
    CombatSkillData::new(
        CombatSkill::HolyLance,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [65, 75, 85, 95, 105], true)),
        R43,
        TargetMod::Single(R234),
    )
    .with_movement((0, 1))
    .with_effects(UNHOLY_KILLER),
    CombatSkillData::new(
        CombatSkill::HookAndSlice,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-95; 5], [50, 60, 70, 80, 90], true)),
        R234,
        TargetMod::Single(R34),
    )
    .with_effects(HOOK_AND_SLICE_EFFECTS),
    CombatSkillData::new(
        CombatSkill::HoundsHarry,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-75; 5], [-50, -40, -30, -20, -10], true)),
        R4321,
        TargetMod::Multi(R1234),
    )
    .with_effects(HARRY_BLEED),
    CombatSkillData::new(
        CombatSkill::HoundsRush,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [0; 5], [50, 60, 70, 80, 90], true)),
        R432,
        TargetMod::Single(R1234),
    )
    .with_effects(HOUNDS_RUSH_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Howl,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R43,
        TargetMod::AlliesMulti,
    )
    .with_effects(HOUND_HOWL)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::IfItBleeds,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [-35; 5], [0, 10, 20, 30, 40], true)),
        R321,
        TargetMod::Single(R23),
    )
    .with_effects(BLEED),
    CombatSkillData::new(
        CombatSkill::Incision,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [50, 60, 70, 80, 90], true)),
        R321,
        TargetMod::Single(R12),
    )
    .with_effects(BLEED),
    CombatSkillData::new(
        CombatSkill::InspiringCry,
        SkillData::Heal([(1, 1), (1, 1), (1, 2), (1, 2), (2, 2)]),
        R4321,
        TargetMod::Allies,
    )
    .with_effects(INSPIRING_CRY_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::InspiringTune,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R43,
        TargetMod::Allies,
    )
    .with_effects(INSPIRING_TUNE_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Intimidate,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-85, -85, -85, -80, -80], [0; 5], true)),
        R1,
        TargetMod::Single(R1234),
    )
    .ignore_stealth()
    .with_effects(INTIMIDATE_EFFECTS),
    CombatSkillData::new(
        CombatSkill::InvigoratingVapours,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R43,
        TargetMod::AlliesMulti,
    )
    .with_effects(ANTIQ_DODGE)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::IronSwan,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [50, 60, 70, 80, 90], true)),
        R1,
        TargetMod::Single(R4),
    ),
    CombatSkillData::new(
        CombatSkill::Judgement,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-25; 5], [50, 60, 70, 80, 90], true)),
        R43,
        TargetMod::Single(R1234),
    )
    .with_effects(VESTAL_HEALSELF),
    CombatSkillData::new(
        CombatSkill::KrisStab,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [30, 40, 50, 60, 70], true)),
        R4321,
        TargetMod::Single(R123),
    ),
    CombatSkillData::new(
        CombatSkill::LickWounds,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R432,
        TargetMod::Performer,
    )
    .with_effects(LICK_WOUNDS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Lunge,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [40; 5], [80, 90, 100, 110, 120], true)),
        R43,
        TargetMod::Single(R123),
    )
    .with_movement((0, 2))
    .with_effects(POISON_KILLER),
    CombatSkillData::new(
        CombatSkill::MaceBash,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [0, 10, 20, 30, 40], true)),
        R21,
        TargetMod::Single(R12),
    )
    .with_effects(UNHOLY_KILLER),
    CombatSkillData::new(
        CombatSkill::Manacles,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-60; 5], [10, 20, 30, 40, 50], true)),
        R32,
        TargetMod::Single(R123),
    )
    .with_effects(MANACLES_STUN)
    .with_modes(&[Mode::Human]),
    CombatSkillData::new(
        CombatSkill::NoxiousBlast,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-80; 5], [50, 60, 70, 80, 90], true)),
        R432,
        TargetMod::Single(R12),
    )
    .with_effects(NOXIOUS_BLAST_EFFECTS),
    CombatSkillData::new(
        CombatSkill::OpenedVein,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-15; 5], [0, 10, 20, 30, 40], true)),
        R321,
        TargetMod::Single(R12),
    )
    .with_effects(OPENED_VEIN_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Pick,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-15; 5], [10, 20, 30, 40, 50], true)),
        R321,
        TargetMod::Single(R12),
    )
    .ignore_protection(),
    CombatSkillData::new(
        CombatSkill::PistolShot,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-15; 5], [75, 85, 95, 105, 115], true)),
        R432,
        TargetMod::Single(R234),
    )
    .with_effects(HW_PISTOL_DMG_MARKED),
    CombatSkillData::new(
        CombatSkill::PlagueGrenade,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-90; 5], [0, 10, 20, 30, 40], true)),
        R43,
        TargetMod::Multi(R34),
    )
    .with_effects(PD_BLIGHT),
    CombatSkillData::new(
        CombatSkill::PointBlankShot,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [50; 5], [50, 60, 70, 80, 90], true)),
        R1,
        TargetMod::Single(R1),
    )
    .with_movement((1, 0))
    .with_effects(PUSH_1),
    CombatSkillData::new(
        CombatSkill::PoisonDart,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-60; 5], [75, 85, 95, 105, 115], true)),
        R432,
        TargetMod::Single(R1234),
    )
    .with_effects(POISON_DART_EFFECTS),
    CombatSkillData::new(
        CombatSkill::ProtectMe,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Allies,
    )
    .not_self_target()
    .with_battle_limit(3)
    .with_effects(PROTECT_ME_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Rage,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [75, 85, 95, 105, 115], true)),
        R21,
        TargetMod::Single(R123),
    )
    .with_modes(&[Mode::Beast]),
    CombatSkillData::new(
        CombatSkill::Rake,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-50; 5], [-30, -20, -10, 0, 10], true)),
        R21,
        TargetMod::Multi(R12),
    )
    .with_effects(RAKEBUFF)
    .with_modes(&[Mode::Beast]),
    CombatSkillData::new(
        CombatSkill::Rampart,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-60; 5], [50, 60, 70, 80, 90], true)),
        R321,
        TargetMod::Single(R12),
    )
    .with_movement((0, 1))
    .with_effects(RAMPART_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Retribution,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [-75; 5], [25, 35, 45, 55, 65], true)),
        R321,
        TargetMod::Single(R123),
    )
    .with_effects(RETRIBUTION_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Revenge,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_battle_limit(1)
    .with_effects(REVENGE_EFFECTS),
    CombatSkillData::new(
        CombatSkill::ShadowFade,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-100; 5], [0; 5], false)),
        R21,
        TargetMod::Performer,
    )
    .with_movement((2, 0))
    .with_effects(SHADOW_FADE_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Slam,
        SkillData::Melee(AttackData::new([80, 85, 90, 95, 100], [-25; 5], [10, 20, 30, 40, 50], true)),
        R321,
        TargetMod::Single(R12),
    )
    .with_movement((0, 1))
    .with_effects(SLAM_EFFECTS)
    .with_modes(&[Mode::Beast]),
    CombatSkillData::new(
        CombatSkill::SliceOff,
        SkillData::Melee(AttackData::new([95, 100, 105, 110, 115], [-33; 5], [80, 90, 100, 110, 120], true)),
        R32,
        TargetMod::Single(R23),
    )
    .with_effects(SLICE_OFF_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Smite,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [0, 10, 20, 30, 40], true)),
        R21,
        TargetMod::Single(R12),
    )
    .with_effects(UNHOLY_KILLER),
    CombatSkillData::new(
        CombatSkill::SniperMark,
        SkillData::Ranged(AttackData::new([100, 105, 110, 115, 120], [-100; 5], [0; 5], false)),
        R43,
        TargetMod::Single(R234),
    )
    .with_effects(SNIPER_MARK_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::SniperShot,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [0; 5], [50, 60, 70, 80, 90], true)),
        R43,
        TargetMod::Single(R234),
    )
    .with_effects(SNIPER_DAMAGE),
    CombatSkillData::new(
        CombatSkill::Solemnity,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R12,
        TargetMod::Performer,
    )
    .with_effects(SOLEMNITY_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Solo,
        SkillData::Ranged(AttackData::new([125, 130, 135, 140, 145], [-100; 5], [0; 5], false)),
        R43,
        TargetMod::Multi(R1234),
    )
    .with_movement((0, 3))
    .with_battle_limit(2)
    .with_effects(SOLO_EFFECTS),
    CombatSkillData::new(
        CombatSkill::StunningBlow,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-50; 5], [0, 10, 20, 30, 40], true)),
        R21,
        TargetMod::Single(R12),
    )
    .with_effects(STUN),
    CombatSkillData::new(
        CombatSkill::SuppressingFire,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-80; 5], [-100, -90, -80, -70, -60], true)),
        R43,
        TargetMod::Multi(R34),
    )
    .with_effects(SUPPRESSION),
    CombatSkillData::new(
        CombatSkill::TargetTag,
        SkillData::Ranged(AttackData::new([100, 105, 110, 115, 120], [-100; 5], [0; 5], false)),
        R4321,
        TargetMod::Single(R1234),
    )
    .with_effects(TARGET_TAG_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::TakeAim,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-80; 5], [0, 0, 5, 5, 10], true)),
        R4321,
        TargetMod::Single(R234),
    )
    .with_battle_limit(1)
    .ignore_stealth()
    .with_effects(TAKE_AIM_EFFECTS),
    CombatSkillData::new(
        CombatSkill::ThrownDagger,
        SkillData::Ranged(AttackData::new([90, 95, 100, 105, 110], [-10; 5], [80, 90, 100, 110, 120], true)),
        R432,
        TargetMod::Single(R234),
    )
    .with_effects(THROWN_DAGGER_EFFECTS),
    CombatSkillData::new(
        CombatSkill::ToxinTrickery,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_battle_limit(1)
    .with_effects(TOXIN_TRICKERY_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::Transform,
        SkillData::Ranged(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R4321,
        TargetMod::Performer,
    )
    .with_modes(&[Mode::Human, Mode::Beast])
    .with_mode_effects([(Mode::Human, TRANSFORM_HUMAN_EFFECTS), (Mode::Beast, TRANSFORM_BEAST_EFFECTS)])
    .is_continue_turn()
    .with_turn_limit(1)
    .with_battle_limit(2),
    CombatSkillData::new(
        CombatSkill::Uppercut,
        SkillData::Melee(AttackData::new([90, 95, 100, 105, 110], [-67; 5], [0, 10, 20, 30, 40], true)),
        R21,
        TargetMod::Single(R12),
    )
    .with_effects(UPPERCUT_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Vomit,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-90; 5], [20, 30, 40, 50, 60], true)),
        R32,
        TargetMod::Multi(R23),
    )
    .with_effects(VOMIT_EFFECTS)
    .with_modes(&[Mode::Human]),
    CombatSkillData::new(
        CombatSkill::WeakeningCurse,
        SkillData::Ranged(AttackData::new([95, 100, 105, 110, 115], [-75; 5], [50, 60, 70, 80, 90], true)),
        R4321,
        TargetMod::Single(R1234),
    )
    .with_effects(WEAKENING_CURSE_EFFECTS),
    CombatSkillData::new(
        CombatSkill::Whistle,
        SkillData::Ranged(AttackData::new([100, 105, 110, 115, 120], [-100; 5], [0; 5], false)),
        R4321,
        TargetMod::Single(R1234),
    )
    .with_effects(WHISTLE_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::WickedHack,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [0; 5], [40, 50, 60, 70, 80], true)),
        R21,
        TargetMod::Single(R12),
    ),
    CombatSkillData::new(
        CombatSkill::WickedSlice,
        SkillData::Melee(AttackData::new([85, 90, 95, 100, 105], [15; 5], [50, 60, 70, 80, 90], true)),
        R321,
        TargetMod::Single(R12),
    ),
    CombatSkillData::new(
        CombatSkill::Withstand,
        SkillData::Melee(AttackData::new([0; 5], [0; 5], [0; 5], true)),
        R321,
        TargetMod::Performer,
    )
    .with_battle_limit(1)
    .with_effects(WITHSTAND_EFFECTS)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::WyrdReconstruction,
        SkillData::Heal([(0, 13), (0, 15), (0, 17), (0, 19), (0, 22)]),
        R4321,
        TargetMod::Allies,
    )
    .with_effects(WYRD_BLEED)
    .not_stall_invalidating(),
    CombatSkillData::new(
        CombatSkill::ZealousAccusation,
        SkillData::Ranged(AttackData::new([85, 90, 95, 100, 105], [-40; 5], [-40, -30, -20, -10, 0], true)),
        R21,
        TargetMod::Multi(R12),
    ),
];
