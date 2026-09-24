use crate::catalog::{
    effect::{ApplicationKind, Buff, CombatStatBuff, Condition, Duration, EffectData, MonsterType, Status, StatusEffect, Target, effect::Effect},
    heroes::Mode,
};

pub const fn effect_data(effect: &Effect) -> EffectData {
    match effect {
        Effect::AbomVomit(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotPoison(5)],
                4 => &[StatusEffect::DotPoison(4)],
                2..=3 => &[StatusEffect::DotPoison(3)],
                0..=1 => &[StatusEffect::DotPoison(2)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::AbsolutionHeal(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(match *lv {
            5.. => &[StatusEffect::Heal { amount: 5, is_skill: true }],
            3..=4 => &[StatusEffect::Heal { amount: 4, is_skill: true }],
            0..=2 => &[StatusEffect::Heal { amount: 3, is_skill: true }],
        }),
        Effect::AbsolutionHealStress(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue).with_status_effects(match *lv {
            5.. => &[StatusEffect::HealStress(10)],
            4 => &[StatusEffect::HealStress(9)],
            2..=3 => &[StatusEffect::HealStress(8)],
            0..=1 => &[StatusEffect::HealStress(7)],
        }),
        Effect::AbyssalKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_condition(Condition::MonsterType(MonsterType::Eldritch))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                4 => &[CombatStatBuff::DamageHighMultiply(22), CombatStatBuff::DamageLowMultiply(22)],
                3 => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
                2.. => &[CombatStatBuff::DamageHighMultiply(17), CombatStatBuff::DamageLowMultiply(17)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            }),
        Effect::AbyssalStun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 150,
                4 => 140,
                3 => 130,
                2 => 120,
                0..=1 => 110,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::Adrenaline(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Cure])
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::AttackRatingAdd(10),
                    CombatStatBuff::DamageHighMultiply(30),
                    CombatStatBuff::DamageLowMultiply(30),
                ],
                4 => &[
                    CombatStatBuff::AttackRatingAdd(8),
                    CombatStatBuff::DamageHighMultiply(26),
                    CombatStatBuff::DamageLowMultiply(26),
                ],
                3 => &[
                    CombatStatBuff::AttackRatingAdd(7),
                    CombatStatBuff::DamageHighMultiply(24),
                    CombatStatBuff::DamageLowMultiply(24),
                ],
                2 => &[
                    CombatStatBuff::AttackRatingAdd(6),
                    CombatStatBuff::DamageHighMultiply(22),
                    CombatStatBuff::DamageLowMultiply(22),
                ],
                0..=1 => &[
                    CombatStatBuff::AttackRatingAdd(5),
                    CombatStatBuff::DamageHighMultiply(20),
                    CombatStatBuff::DamageLowMultiply(20),
                ],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::AntiqBlight(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotPoison(4)],
                4 => &[StatusEffect::DotPoison(3)],
                2..=3 => &[StatusEffect::DotPoison(2)],
                0..=1 => &[StatusEffect::DotPoison(1)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::AntiqBlightBuff(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_buffs(match *lv {
                5.. => &[Buff::AntiqBlightBuff(5)],
                4 => &[Buff::AntiqBlightBuff(4)],
                3 => &[Buff::AntiqBlightBuff(3)],
                2 => &[Buff::AntiqBlightBuff(2)],
                0..=1 => &[Buff::AntiqBlightBuff(1)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::AntiqBlightDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::AntiqBlightDebuff(5)],
                4 => &[Buff::AntiqBlightDebuff(4)],
                3 => &[Buff::AntiqBlightDebuff(3)],
                2 => &[Buff::AntiqBlightDebuff(2)],
                0..=1 => &[Buff::AntiqBlightDebuff(1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::AntiqCower(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(25)],
                4 => &[CombatStatBuff::DefenseRatingAdd(22)],
                3 => &[CombatStatBuff::DefenseRatingAdd(20)],
                2 => &[CombatStatBuff::DefenseRatingAdd(18)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(15)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::AntiqDefBuff(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(8), CombatStatBuff::ProtectionRatingAdd(200)],
                4 => &[CombatStatBuff::DefenseRatingAdd(7), CombatStatBuff::ProtectionRatingAdd(180)],
                3 => &[CombatStatBuff::DefenseRatingAdd(6), CombatStatBuff::ProtectionRatingAdd(150)],
                2 => &[CombatStatBuff::DefenseRatingAdd(5), CombatStatBuff::ProtectionRatingAdd(130)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(4), CombatStatBuff::ProtectionRatingAdd(100)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::AntiqDistract(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::AttackRatingAdd(-15)],
                4 => &[CombatStatBuff::AttackRatingAdd(-14)],
                3 => &[CombatStatBuff::AttackRatingAdd(-12)],
                2 => &[CombatStatBuff::AttackRatingAdd(-11)],
                0..=1 => &[CombatStatBuff::AttackRatingAdd(-10)],
            })
            .with_duration(Duration::Rounds(2)),
        Effect::AntiqDodge(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(10)],
                4 => &[CombatStatBuff::DefenseRatingAdd(9)],
                3 => &[CombatStatBuff::DefenseRatingAdd(7)],
                2 => &[CombatStatBuff::DefenseRatingAdd(5)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(3)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::AntiqProtectMeClearGuardsPerformer => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding])
            .apply_on_miss(),
        Effect::AntiqProtectMeClearGuardsTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding])
            .apply_on_miss(),
        Effect::AntiqProtectMeGuard => EffectData::new(Target::Target, ApplicationKind::Stack)
            .swap_source_and_target()
            .with_status_effects(&[StatusEffect::Guard])
            .with_duration(Duration::Rounds(2))
            .apply_on_miss(),
        Effect::AntiqSelfSpeed(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(5)],
                4 => &[CombatStatBuff::SpeedRatingAdd(4)],
                3 => &[CombatStatBuff::SpeedRatingAdd(3)],
                2 => &[CombatStatBuff::SpeedRatingAdd(2)],
                0..=1 => &[CombatStatBuff::SpeedRatingAdd(1)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::ArbMarkDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(-30)],
                4 => &[CombatStatBuff::DefenseRatingAdd(-27)],
                3 => &[CombatStatBuff::DefenseRatingAdd(-25)],
                2 => &[CombatStatBuff::DefenseRatingAdd(-22)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(-20)],
            })
            .with_duration(Duration::Rounds(2)),
        Effect::ArbMarkTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(3)),
        Effect::ArbSelfSpeed(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(5)],
                3..=4 => &[CombatStatBuff::SpeedRatingAdd(4)],
                0..=2 => &[CombatStatBuff::SpeedRatingAdd(3)],
            })
            .apply_on_miss(),
        Effect::ArbStackingHeal(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::HpHealReceivedPercent(38)],
                4 => &[CombatStatBuff::HpHealReceivedPercent(33)],
                3 => &[CombatStatBuff::HpHealReceivedPercent(28)],
                2 => &[CombatStatBuff::HpHealReceivedPercent(24)],
                0..=1 => &[CombatStatBuff::HpHealReceivedPercent(20)],
            })
            .apply_on_miss(),
        Effect::BeastBuff(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(5)],
                4 => &[CombatStatBuff::SpeedRatingAdd(4)],
                3 => &[CombatStatBuff::SpeedRatingAdd(3)],
                2 => &[CombatStatBuff::SpeedRatingAdd(2)],
                0..=1 => &[CombatStatBuff::SpeedRatingAdd(1)],
            })
            .with_duration(Duration::Rounds(4)),
        Effect::BeastBuff2(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_buffs(match *lv {
                5.. => &[Buff::BeastBlightBuff(5)],
                4 => &[Buff::BeastBlightBuff(4)],
                3 => &[Buff::BeastBlightBuff(3)],
                2 => &[Buff::BeastBlightBuff(2)],
                0..=1 => &[Buff::BeastBlightBuff(1)],
            })
            .with_duration(Duration::Rounds(4)),
        Effect::BeastDebuff(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack).with_combat_stat_buffs(match *lv {
            4 => &[CombatStatBuff::SpeedRatingAdd(-1)],
            3 => &[CombatStatBuff::SpeedRatingAdd(-2)],
            2 => &[CombatStatBuff::SpeedRatingAdd(-3)],
            0..=1 => &[CombatStatBuff::SpeedRatingAdd(-4)],
            _ => &[],
        }),
        Effect::BeastKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::MonsterType(MonsterType::Beast))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)],
                4 => &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)],
                3 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                2.. => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            }),
        Effect::BeastStressParty => {
            EffectData::new(Target::PerformerGroupOther, ApplicationKind::Queue).with_status_effects(&[StatusEffect::Stress(8)])
        }
        Effect::BellowCrit(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(&[Buff::MaaBellowCritReceived])
            .with_duration(Duration::Rounds(3)),
        Effect::BhDmgMarked => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Tagged))
            .with_combat_stat_buffs(&[CombatStatBuff::DamageHighMultiply(90), CombatStatBuff::DamageLowMultiply(90)]),
        Effect::BhMarkDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(-200)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(-175)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(-150)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(-125)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(-100)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::BhMarkTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(3)),
        Effect::BhMinorMark => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(2)),
        Effect::BhSelfSpeed(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(5)],
                3..=4 => &[CombatStatBuff::SpeedRatingAdd(4)],
                0..=2 => &[CombatStatBuff::SpeedRatingAdd(3)],
            })
            .with_duration(Duration::Rounds(2)),
        Effect::Bleed(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotBleed(4)],
                3..=4 => &[StatusEffect::DotBleed(3)],
                0..=2 => &[StatusEffect::DotBleed(2)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::BoloPush1(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 115,
                4 => 105,
                3 => 95,
                2 => 85,
                0..=1 => 75,
            })
            .with_status_effects(&[StatusEffect::Push(1)])
            .apply_on_death(),
        Effect::Bolster(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(10)],
                4 => &[CombatStatBuff::DefenseRatingAdd(8)],
                3 => &[CombatStatBuff::DefenseRatingAdd(7)],
                2 => &[CombatStatBuff::DefenseRatingAdd(6)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(5)],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::BolsterStressBuff(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_buffs(match *lv {
                5.. => &[Buff::StressDmg(-20)],
                3..=4 => &[Buff::StressDmg(-15)],
                0..=2 => &[Buff::StressDmg(-10)],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::BuildToFinale(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_buffs(match *lv {
                2.. => &[Buff::BuildToFinaleDmgH(2), Buff::BuildToFinaleDmgL(2)],
                0..=1 => &[Buff::BuildToFinaleDmgH(1), Buff::BuildToFinaleDmgL(1)],
            })
            .with_duration(Duration::Rounds(8)),
        Effect::BuildToFinaleSong => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_buffs(&[Buff::BuildToFinaleDmgH(1), Buff::BuildToFinaleDmgL(1)])
            .with_duration(Duration::Rounds(8)),
        Effect::CaltropsPreyDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::CaltropsDmgReceived(5)],
                4 => &[Buff::CaltropsDmgReceived(4)],
                3 => &[Buff::CaltropsDmgReceived(3)],
                2 => &[Buff::CaltropsDmgReceived(2)],
                0..=1 => &[Buff::CaltropsDmgReceived(1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::CaltropsSpdDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(-8)],
                4 => &[CombatStatBuff::SpeedRatingAdd(-7)],
                3 => &[CombatStatBuff::SpeedRatingAdd(-6)],
                2 => &[CombatStatBuff::SpeedRatingAdd(-5)],
                0..=1 => &[CombatStatBuff::SpeedRatingAdd(-4)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::ClearCorpses => EffectData::new(Target::TargetGroup, ApplicationKind::Stack)
            .with_condition(Condition::MonsterType(MonsterType::Corpse))
            .with_status_effects(&[StatusEffect::Kill])
            .apply_on_death(),
        Effect::ClearGuardPerformer => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding])
            .apply_on_miss(),
        Effect::ClearGuardTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::ClearGuarded, StatusEffect::ClearGuarding])
            .apply_on_miss(),
        Effect::Command(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_buffs(match *lv {
                5.. => &[
                    Buff::MaaCommandAcc(5),
                    Buff::MaaCommandCrit(5),
                    Buff::MaaCommandGuardedDmgH(5),
                    Buff::MaaCommandGuardedDmgL(5),
                ],
                4 => &[
                    Buff::MaaCommandAcc(4),
                    Buff::MaaCommandCrit(4),
                    Buff::MaaCommandGuardedDmgH(4),
                    Buff::MaaCommandGuardedDmgL(4),
                ],
                3 => &[
                    Buff::MaaCommandAcc(3),
                    Buff::MaaCommandCrit(3),
                    Buff::MaaCommandGuardedDmgH(3),
                    Buff::MaaCommandGuardedDmgL(3),
                ],
                2 => &[
                    Buff::MaaCommandAcc(2),
                    Buff::MaaCommandCrit(2),
                    Buff::MaaCommandGuardedDmgH(2),
                    Buff::MaaCommandGuardedDmgL(2),
                ],
                0..=1 => &[
                    Buff::MaaCommandAcc(1),
                    Buff::MaaCommandCrit(1),
                    Buff::MaaCommandGuardedDmgH(1),
                    Buff::MaaCommandGuardedDmgL(1),
                ],
            })
            .apply_on_miss(),
        Effect::CrusaderBulwark(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(300)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(280)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(250)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(220)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(200)],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::CrusaderBulwarkLight => EffectData::new(Target::Global, ApplicationKind::QueueOnce)
            .with_status_effects(&[StatusEffect::Torch(24)])
            .apply_on_miss(),
        Effect::CrusaderBulwarkMark => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::CrusaderHealStress(lv) => EffectData::new(Target::Target, ApplicationKind::Queue).with_status_effects(match *lv {
            5.. => &[StatusEffect::HealStress(8)],
            4 => &[StatusEffect::HealStress(7)],
            3 => &[StatusEffect::HealStress(6)],
            0..=2 => &[StatusEffect::HealStress(5)],
        }),
        Effect::CrusaderLight(lv) => EffectData::new(Target::Global, ApplicationKind::QueueOnce)
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::Torch(10)],
                4 => &[StatusEffect::Torch(8)],
                3 => &[StatusEffect::Torch(7)],
                2 => &[StatusEffect::Torch(6)],
                0..=1 => &[StatusEffect::Torch(5)],
            })
            .apply_on_miss(),
        Effect::Cure => EffectData::new(Target::Target, ApplicationKind::Stack).with_status_effects(&[StatusEffect::Cure]),
        Effect::CureSelf => EffectData::new(Target::Performer, ApplicationKind::QueueOnce).with_status_effects(&[StatusEffect::Cure]),
        Effect::Darkness => EffectData::new(Target::Global, ApplicationKind::QueueOnce).with_status_effects(&[StatusEffect::Torch(-5)]),
        Effect::DazzlingLight => EffectData::new(Target::Global, ApplicationKind::QueueOnce)
            .with_status_effects(&[StatusEffect::Torch(6)])
            .apply_on_miss(),
        Effect::Defender(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(300)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(260)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(220)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(180)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(150)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::Destealth => EffectData::new(Target::Target, ApplicationKind::Stack).with_status_effects(&[StatusEffect::Unstealth]),
        Effect::Disorient(lv) => EffectData::new(Target::Target, ApplicationKind::ApplyOnce)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Shuffle])
            .apply_on_death(),
        Effect::Disrupt(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(-10), CombatStatBuff::SpeedRatingAdd(-7)],
                4 => &[CombatStatBuff::DefenseRatingAdd(-8), CombatStatBuff::SpeedRatingAdd(-6)],
                3 => &[CombatStatBuff::DefenseRatingAdd(-7), CombatStatBuff::SpeedRatingAdd(-6)],
                2 => &[CombatStatBuff::DefenseRatingAdd(-6), CombatStatBuff::SpeedRatingAdd(-5)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(-5), CombatStatBuff::SpeedRatingAdd(-5)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::DodgeCurse(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(-30)],
                4 => &[CombatStatBuff::DefenseRatingAdd(-27)],
                3 => &[CombatStatBuff::DefenseRatingAdd(-25)],
                2 => &[CombatStatBuff::DefenseRatingAdd(-23)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(-20)],
            }),
        Effect::EldritchKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_condition(Condition::MonsterType(MonsterType::Eldritch))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)],
                4 => &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)],
                3 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                2.. => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            }),
        Effect::EmboldenTeam(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::AttackRatingAdd(10),
                    CombatStatBuff::CritChanceAdd(6),
                    CombatStatBuff::SpeedRatingAdd(4),
                ],
                4 => &[
                    CombatStatBuff::AttackRatingAdd(8),
                    CombatStatBuff::CritChanceAdd(5),
                    CombatStatBuff::SpeedRatingAdd(3),
                ],
                3 => &[
                    CombatStatBuff::AttackRatingAdd(7),
                    CombatStatBuff::CritChanceAdd(4),
                    CombatStatBuff::SpeedRatingAdd(3),
                ],
                2 => &[
                    CombatStatBuff::AttackRatingAdd(6),
                    CombatStatBuff::CritChanceAdd(3),
                    CombatStatBuff::SpeedRatingAdd(2),
                ],
                0..=1 => &[
                    CombatStatBuff::AttackRatingAdd(5),
                    CombatStatBuff::CritChanceAdd(2),
                    CombatStatBuff::SpeedRatingAdd(2),
                ],
            })
            .apply_on_miss(),
        Effect::FlareClear => EffectData::new(Target::PerformerGroupOther, ApplicationKind::ApplyOnce)
            .with_status_effects(&[StatusEffect::Unstun, StatusEffect::Untag])
            .apply_on_miss(),
        Effect::FlareHealStress(lv) => EffectData::new(Target::PerformerGroupOther, ApplicationKind::QueueOnce)
            .with_chance(match *lv {
                3 | 5.. => 67,
                0..=2 | 4 => 60,
            })
            .with_status_effects(match *lv {
                4.. => &[StatusEffect::HealStress(3)],
                2..=3 => &[StatusEffect::HealStress(2)],
                0..=1 => &[StatusEffect::HealStress(1)],
            })
            .apply_on_miss(),
        Effect::FlareLight(lv) => EffectData::new(Target::Global, ApplicationKind::QueueOnce)
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::Torch(7)],
                4 => &[StatusEffect::Torch(6)],
                3 => &[StatusEffect::Torch(5)],
                2 => &[StatusEffect::Torch(4)],
                0..=1 => &[StatusEffect::Torch(3)],
            })
            .apply_on_miss(),
        Effect::FortifyResists(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_buffs(match *lv {
                5.. => &[Buff::BleedResist(15), Buff::BlightResist(15)],
                4 => &[Buff::BleedResist(14), Buff::BlightResist(14)],
                3 => &[Buff::BleedResist(12), Buff::BlightResist(12)],
                2 => &[Buff::BleedResist(11), Buff::BlightResist(11)],
                0..=1 => &[Buff::BleedResist(10), Buff::BlightResist(10)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::GrAccBuff(lv) => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce).with_combat_stat_buffs(match *lv {
            5.. => &[CombatStatBuff::AttackRatingAdd(10)],
            4 => &[CombatStatBuff::AttackRatingAdd(8)],
            3 => &[CombatStatBuff::AttackRatingAdd(7)],
            2 => &[CombatStatBuff::AttackRatingAdd(6)],
            0..=1 => &[CombatStatBuff::AttackRatingAdd(5)],
        }),
        Effect::GrBleedDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::BleedDebuff(5)],
                4 => &[Buff::BleedDebuff(4)],
                3 => &[Buff::BleedDebuff(3)],
                2 => &[Buff::BleedDebuff(2)],
                0..=1 => &[Buff::BleedDebuff(1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::GrBlight(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotPoison(4)],
                2..=4 => &[StatusEffect::DotPoison(3)],
                0..=1 => &[StatusEffect::DotPoison(2)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_death(),
        Effect::GrBlightDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::BlightDebuff(5)],
                4 => &[Buff::BlightDebuff(4)],
                3 => &[Buff::BlightDebuff(3)],
                2 => &[Buff::BlightDebuff(2)],
                0..=1 => &[Buff::BlightDebuff(1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::GrDaggerDmgMarked(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Tagged))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(40), CombatStatBuff::DamageLowMultiply(40)],
                4 => &[CombatStatBuff::DamageHighMultiply(36), CombatStatBuff::DamageLowMultiply(36)],
                3 => &[CombatStatBuff::DamageHighMultiply(32), CombatStatBuff::DamageLowMultiply(32)],
                2.. => &[CombatStatBuff::DamageHighMultiply(28), CombatStatBuff::DamageLowMultiply(28)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
            }),
        Effect::GrDodge(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(15)],
                4 => &[CombatStatBuff::DefenseRatingAdd(13)],
                3 => &[CombatStatBuff::DefenseRatingAdd(12)],
                2 => &[CombatStatBuff::DefenseRatingAdd(11)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(10)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::GrFadeAttack(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::CritChanceAdd(8),
                    CombatStatBuff::DamageHighMultiply(100),
                    CombatStatBuff::DamageLowMultiply(100),
                ],
                4 => &[
                    CombatStatBuff::CritChanceAdd(7),
                    CombatStatBuff::DamageHighMultiply(95),
                    CombatStatBuff::DamageLowMultiply(95),
                ],
                3 => &[
                    CombatStatBuff::CritChanceAdd(6),
                    CombatStatBuff::DamageHighMultiply(90),
                    CombatStatBuff::DamageLowMultiply(90),
                ],
                2.. => &[
                    CombatStatBuff::CritChanceAdd(5),
                    CombatStatBuff::DamageHighMultiply(85),
                    CombatStatBuff::DamageLowMultiply(85),
                ],
                0..=1 => &[
                    CombatStatBuff::CritChanceAdd(4),
                    CombatStatBuff::DamageHighMultiply(80),
                    CombatStatBuff::DamageLowMultiply(80),
                ],
            })
            .with_duration(Duration::Rounds(2)),
        Effect::GrSelfSpeed(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(4)],
                3..=4 => &[CombatStatBuff::SpeedRatingAdd(3)],
                0..=2 => &[CombatStatBuff::SpeedRatingAdd(2)],
            })
            .with_duration(Duration::Combat),
        Effect::GrapeshotVulnerability(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::CritReceivedChance(8)],
                4 => &[CombatStatBuff::CritReceivedChance(7)],
                3 => &[CombatStatBuff::CritReceivedChance(6)],
                2.. => &[CombatStatBuff::CritReceivedChance(5)],
                0..=1 => &[CombatStatBuff::CritReceivedChance(4)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::HarryBleed(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 150,
                4 => 140,
                3 => 130,
                2 => 120,
                0..=1 => 110,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotBleed(3)],
                3..=4 => &[StatusEffect::DotBleed(2)],
                0..=2 => &[StatusEffect::DotBleed(1)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::HellionExhaust => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_buffs(&[Buff::HellionExDmgH, Buff::HellionExDmgL, Buff::HellionExSpd])
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::HellionExhaustSm => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_buffs(&[Buff::HellionExDmgHSm, Buff::HellionExDmgLSm, Buff::HellionExSpdSm])
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::HellionHealSelf(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(match *lv {
            5.. => &[StatusEffect::Heal { amount: 4, is_skill: true }],
            4 => &[StatusEffect::Heal { amount: 3, is_skill: true }],
            2..=3 => &[StatusEffect::Heal { amount: 2, is_skill: true }],
            0..=1 => &[StatusEffect::Heal { amount: 1, is_skill: true }],
        }),
        Effect::HeroStrongStun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 150,
                4 => 140,
                3 => 130,
                2 => 120,
                0..=1 => 110,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::HmDmgMarked(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Tagged))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(100), CombatStatBuff::DamageLowMultiply(100)],
                4 => &[CombatStatBuff::DamageHighMultiply(90), CombatStatBuff::DamageLowMultiply(90)],
                3 => &[CombatStatBuff::DamageHighMultiply(80), CombatStatBuff::DamageLowMultiply(80)],
                2.. => &[CombatStatBuff::DamageHighMultiply(70), CombatStatBuff::DamageLowMultiply(70)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(60), CombatStatBuff::DamageLowMultiply(60)],
            }),
        Effect::HmGuard => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Guard])
            .with_duration(Duration::Rounds(2))
            .apply_on_miss(),
        Effect::HmMarkTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(3)),
        Effect::HoundBleed(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                4.. => &[StatusEffect::DotBleed(2)],
                0..=3 => &[StatusEffect::DotBleed(1)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::HoundDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 170,
                4 => 160,
                3 => 150,
                2 => 140,
                0..=1 => 130,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(-300)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(-270)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(-250)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(-220)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(-200)],
            }),
        Effect::HoundHowl(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 74,
                4 => 72,
                3 => 70,
                2 => 68,
                0..=1 => 66,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::HealStress(6)],
                4 => &[StatusEffect::HealStress(5)],
                3 => &[StatusEffect::HealStress(4)],
                2 => &[StatusEffect::HealStress(3)],
                0..=1 => &[StatusEffect::HealStress(2)],
            }),
        Effect::HoundProtect(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(20)],
                4 => &[CombatStatBuff::DefenseRatingAdd(17)],
                3 => &[CombatStatBuff::DefenseRatingAdd(14)],
                2 => &[CombatStatBuff::DefenseRatingAdd(12)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(10)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::HumanStressHealParty => {
            EffectData::new(Target::PerformerGroupOther, ApplicationKind::Queue).with_status_effects(&[StatusEffect::HealStress(2)])
        }
        Effect::HwOpenVeinBleedDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::BleedDebuff(5)],
                4 => &[Buff::BleedDebuff(4)],
                3 => &[Buff::BleedDebuff(3)],
                2 => &[Buff::BleedDebuff(2)],
                0..=1 => &[Buff::BleedDebuff(1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::HwOpenVeinSpdDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::Spd(-3)],
                3..=4 => &[Buff::Spd(-2)],
                0..=2 => &[Buff::Spd(-1)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::HwPistolDmgMarked(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Tagged))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(50), CombatStatBuff::DamageLowMultiply(50)],
                4 => &[CombatStatBuff::DamageHighMultiply(40), CombatStatBuff::DamageLowMultiply(40)],
                3 => &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)],
                2.. => &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
            }),
        Effect::HwyRiposte(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::Riposte { damage: -15, crit: 5 }],
                4 => &[StatusEffect::Riposte { damage: -20, crit: 4 }],
                3 => &[StatusEffect::Riposte { damage: -25, crit: 3 }],
                2 => &[StatusEffect::Riposte { damage: -33, crit: 2 }],
                0..=1 => &[StatusEffect::Riposte { damage: -40, crit: 0 }],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::InspiringTune(lv) => EffectData::new(Target::Target, ApplicationKind::Queue).with_status_effects(match *lv {
            5.. => &[StatusEffect::HealStress(12)],
            4 => &[StatusEffect::HealStress(11)],
            3 => &[StatusEffect::HealStress(10)],
            2 => &[StatusEffect::HealStress(9)],
            0..=1 => &[StatusEffect::HealStress(8)],
        }),
        Effect::JesterSpotlight(lv) => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(30)],
                4 => &[CombatStatBuff::DefenseRatingAdd(27)],
                3 => &[CombatStatBuff::DefenseRatingAdd(25)],
                2 => &[CombatStatBuff::DefenseRatingAdd(22)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(20)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::JesterTuneBuff(lv) => EffectData::new(Target::Target, ApplicationKind::Queue).with_buffs(match *lv {
            5.. => &[Buff::JestTuneStressResistance(5)],
            4 => &[Buff::JestTuneStressResistance(4)],
            3 => &[Buff::JestTuneStressResistance(3)],
            2 => &[Buff::JestTuneStressResistance(2)],
            0..=1 => &[Buff::JestTuneStressResistance(1)],
        }),
        Effect::LeperAcc => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_buffs(&[Buff::Acc(5)])
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::LeperHealSelf(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(match *lv {
            5.. => &[StatusEffect::Heal { amount: 10, is_skill: true }],
            4 => &[StatusEffect::Heal { amount: 9, is_skill: true }],
            3 => &[StatusEffect::Heal { amount: 8, is_skill: true }],
            2 => &[StatusEffect::Heal { amount: 7, is_skill: true }],
            0..=1 => &[StatusEffect::Heal { amount: 6, is_skill: true }],
        }),
        Effect::LeperHealSelfStress(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue).with_status_effects(match *lv {
            5.. => &[StatusEffect::HealStress(7)],
            3..=4 => &[StatusEffect::HealStress(6)],
            0..=2 => &[StatusEffect::HealStress(5)],
        }),
        Effect::LeperHype(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::SpeedRatingAdd(4)],
                3..=4 => &[CombatStatBuff::SpeedRatingAdd(3)],
                0..=2 => &[CombatStatBuff::SpeedRatingAdd(2)],
            })
            .apply_on_miss(),
        Effect::LeperIntimidate(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::DamageHighMultiply(-20),
                    CombatStatBuff::DamageLowMultiply(-20),
                    CombatStatBuff::SpeedRatingAdd(-3),
                ],
                4 => &[
                    CombatStatBuff::DamageHighMultiply(-23),
                    CombatStatBuff::DamageLowMultiply(-23),
                    CombatStatBuff::SpeedRatingAdd(-3),
                ],
                3 => &[
                    CombatStatBuff::DamageHighMultiply(-26),
                    CombatStatBuff::DamageLowMultiply(-26),
                    CombatStatBuff::SpeedRatingAdd(-4),
                ],
                2.. => &[
                    CombatStatBuff::DamageHighMultiply(-29),
                    CombatStatBuff::DamageLowMultiply(-29),
                    CombatStatBuff::SpeedRatingAdd(-4),
                ],
                0..=1 => &[
                    CombatStatBuff::DamageHighMultiply(-33),
                    CombatStatBuff::DamageLowMultiply(-33),
                    CombatStatBuff::SpeedRatingAdd(-5),
                ],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::LeperIntimidateMark => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(4)),
        Effect::LeperMarkSelf => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::LeperProtect(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(300)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(270)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(250)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(220)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(200)],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::LeperResistBuff => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_buffs(&[
                Buff::LeperBleedResist,
                Buff::LeperBlightResist,
                Buff::LeperDebuffResist,
                Buff::LeperMoveResist,
            ])
            .with_duration(Duration::Combat),
        Effect::LeperStrength(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::AttackRatingAdd(15),
                    CombatStatBuff::CritChanceAdd(11),
                    CombatStatBuff::DamageHighMultiply(35),
                    CombatStatBuff::DamageLowMultiply(35),
                ],
                4 => &[
                    CombatStatBuff::AttackRatingAdd(13),
                    CombatStatBuff::CritChanceAdd(10),
                    CombatStatBuff::DamageHighMultiply(32),
                    CombatStatBuff::DamageLowMultiply(32),
                ],
                3 => &[
                    CombatStatBuff::AttackRatingAdd(12),
                    CombatStatBuff::CritChanceAdd(9),
                    CombatStatBuff::DamageHighMultiply(30),
                    CombatStatBuff::DamageLowMultiply(30),
                ],
                2.. => &[
                    CombatStatBuff::AttackRatingAdd(11),
                    CombatStatBuff::CritChanceAdd(8),
                    CombatStatBuff::DamageHighMultiply(27),
                    CombatStatBuff::DamageLowMultiply(27),
                ],
                0..=1 => &[
                    CombatStatBuff::AttackRatingAdd(10),
                    CombatStatBuff::CritChanceAdd(7),
                    CombatStatBuff::DamageHighMultiply(25),
                    CombatStatBuff::DamageLowMultiply(25),
                ],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::LeperVulnerability => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_buffs(&[Buff::LeperDefVuln, Buff::LeperDmgVuln])
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::LickWounds(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(match *lv {
            5.. => &[StatusEffect::Heal { amount: 8, is_skill: true }],
            4 => &[StatusEffect::Heal { amount: 7, is_skill: true }],
            3 => &[StatusEffect::Heal { amount: 6, is_skill: true }],
            2 => &[StatusEffect::Heal { amount: 5, is_skill: true }],
            0..=1 => &[StatusEffect::Heal { amount: 4, is_skill: true }],
        }),
        Effect::MaaGuard => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Guard])
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::MaaRiposte(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::Riposte { damage: -20, crit: 4 }],
                4 => &[StatusEffect::Riposte { damage: -25, crit: 3 }],
                3 => &[StatusEffect::Riposte { damage: -30, crit: 2 }],
                2 => &[StatusEffect::Riposte { damage: -35, crit: 1 }],
                0..=1 => &[StatusEffect::Riposte { damage: -40, crit: 0 }],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::ManKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::MonsterType(MonsterType::Man))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)],
                4 => &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)],
                3 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                2.. => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            }),
        Effect::ManaclesStun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 130,
                4 => 120,
                3 => 110,
                2 => 100,
                0..=1 => 90,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::MarkSelf => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(2))
            .apply_on_miss(),
        Effect::MarkTarget => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(3)),
        Effect::MortalWeakness => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_combat_stat_buffs(&[CombatStatBuff::DefenseRatingAdd(-25), CombatStatBuff::SpeedRatingAdd(-3)])
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::MortalWeaknessStress => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_buffs(&[Buff::MortalWeaknessStress])
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::NoxiousDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::AttackRatingAdd(-7)],
                3..=4 => &[CombatStatBuff::AttackRatingAdd(-6)],
                0..=2 => &[CombatStatBuff::AttackRatingAdd(-5)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::OccVulnerabilityCurse(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(-20)],
                4 => &[CombatStatBuff::DefenseRatingAdd(-18)],
                3 => &[CombatStatBuff::DefenseRatingAdd(-17)],
                2 => &[CombatStatBuff::DefenseRatingAdd(-16)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(-15)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::OccWeakenProt(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::ProtectionRatingAdd(-200)],
                4 => &[CombatStatBuff::ProtectionRatingAdd(-170)],
                3 => &[CombatStatBuff::ProtectionRatingAdd(-150)],
                2 => &[CombatStatBuff::ProtectionRatingAdd(-130)],
                0..=1 => &[CombatStatBuff::ProtectionRatingAdd(-100)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::OccWeakeningCurse(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(-20), CombatStatBuff::DamageLowMultiply(-20)],
                4 => &[CombatStatBuff::DamageHighMultiply(-17), CombatStatBuff::DamageLowMultiply(-17)],
                3 => &[CombatStatBuff::DamageHighMultiply(-15), CombatStatBuff::DamageLowMultiply(-15)],
                2.. => &[CombatStatBuff::DamageHighMultiply(-12), CombatStatBuff::DamageLowMultiply(-12)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(-10), CombatStatBuff::DamageLowMultiply(-10)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::OnCritAcc => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce).with_buffs(&[Buff::OnCritAcc]),
        Effect::OnCritBleedChance => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritBleedChance])
            .with_duration(Duration::Rounds(2)),
        Effect::OnCritBlightChance => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritBlightChance])
            .with_duration(Duration::Rounds(2)),
        Effect::OnCritDef => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce).with_buffs(&[Buff::OnCritDef]),
        Effect::OnCritDmg => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritDmgH, Buff::OnCritDmgL])
            .with_duration(Duration::Rounds(3)),
        Effect::OnCritDmgBleeding => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritDmgHBleeding, Buff::OnCritDmgLBleeding])
            .with_duration(Duration::Rounds(3)),
        Effect::OnCritDmgMarked => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritDmgHMarked, Buff::OnCritDmgLMarked])
            .with_duration(Duration::Rounds(3)),
        Effect::OnCritHealDone => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritHealBuff])
            .with_duration(Duration::Rounds(2)),
        Effect::OnCritProt => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce).with_buffs(&[Buff::OnCritProt]),
        Effect::OnCritSpeed => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce).with_buffs(&[Buff::OnCritSpd]),
        Effect::OnCritStressHealDone => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritStressHealBuff])
            .with_duration(Duration::Rounds(2)),
        Effect::OnCritStressResist => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(&[Buff::OnCritStressResist])
            .with_duration(Duration::Rounds(3)),
        Effect::PdBlight(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotPoison(6)],
                3..=4 => &[StatusEffect::DotPoison(5)],
                0..=2 => &[StatusEffect::DotPoison(4)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::PdDisorientingStun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::PdSingleBlight(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotPoison(7)],
                3..=4 => &[StatusEffect::DotPoison(6)],
                0..=2 => &[StatusEffect::DotPoison(5)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::PdVapoursBuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::DamageHighMultiply(25),
                    CombatStatBuff::DamageLowMultiply(25),
                    CombatStatBuff::SpeedRatingAdd(5),
                ],
                4 => &[
                    CombatStatBuff::DamageHighMultiply(23),
                    CombatStatBuff::DamageLowMultiply(23),
                    CombatStatBuff::SpeedRatingAdd(4),
                ],
                3 => &[
                    CombatStatBuff::DamageHighMultiply(22),
                    CombatStatBuff::DamageLowMultiply(22),
                    CombatStatBuff::SpeedRatingAdd(4),
                ],
                2.. => &[
                    CombatStatBuff::DamageHighMultiply(21),
                    CombatStatBuff::DamageLowMultiply(21),
                    CombatStatBuff::SpeedRatingAdd(3),
                ],
                0..=1 => &[
                    CombatStatBuff::DamageHighMultiply(20),
                    CombatStatBuff::DamageLowMultiply(20),
                    CombatStatBuff::SpeedRatingAdd(3),
                ],
            })
            .with_duration(Duration::Combat),
        Effect::PoisonKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Poisoned))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(33), CombatStatBuff::DamageLowMultiply(33)],
                4 => &[CombatStatBuff::DamageHighMultiply(29), CombatStatBuff::DamageLowMultiply(29)],
                3 => &[CombatStatBuff::DamageHighMultiply(26), CombatStatBuff::DamageLowMultiply(26)],
                2.. => &[CombatStatBuff::DamageHighMultiply(23), CombatStatBuff::DamageLowMultiply(23)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
            }),
        Effect::Pull2(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Pull(2)])
            .apply_on_death(),
        Effect::Push1(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Push(1)])
            .apply_on_death(),
        Effect::Push2(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Push(2)])
            .apply_on_death(),
        Effect::Push3(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Push(3)])
            .apply_on_death(),
        Effect::Rakebuff(lv) => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_buffs(match *lv {
                5.. => &[Buff::RakeBuffH(5), Buff::RakeBuffL(5)],
                4 => &[Buff::RakeBuffH(4), Buff::RakeBuffL(4)],
                3 => &[Buff::RakeBuffH(3), Buff::RakeBuffL(3)],
                2 => &[Buff::RakeBuffH(2), Buff::RakeBuffL(2)],
                0..=1 => &[Buff::RakeBuffH(1), Buff::RakeBuffL(1)],
            })
            .with_duration(Duration::Rounds(4))
            .apply_on_miss(),
        Effect::ShadowBlood(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue)
            .with_status_effects(&[StatusEffect::Cure])
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(13)],
                4 => &[CombatStatBuff::DefenseRatingAdd(12)],
                3 => &[CombatStatBuff::DefenseRatingAdd(11)],
                2 => &[CombatStatBuff::DefenseRatingAdd(10)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(9)],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::SlamDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DefenseRatingAdd(-20), CombatStatBuff::SpeedRatingAdd(-6)],
                4 => &[CombatStatBuff::DefenseRatingAdd(-17), CombatStatBuff::SpeedRatingAdd(-5)],
                3 => &[CombatStatBuff::DefenseRatingAdd(-15), CombatStatBuff::SpeedRatingAdd(-4)],
                2 => &[CombatStatBuff::DefenseRatingAdd(-12), CombatStatBuff::SpeedRatingAdd(-3)],
                0..=1 => &[CombatStatBuff::DefenseRatingAdd(-10), CombatStatBuff::SpeedRatingAdd(-2)],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::SniperDamage(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Tagged))
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::CritChanceAdd(13),
                    CombatStatBuff::DamageHighMultiply(100),
                    CombatStatBuff::DamageLowMultiply(100),
                ],
                4 => &[
                    CombatStatBuff::CritChanceAdd(12),
                    CombatStatBuff::DamageHighMultiply(80),
                    CombatStatBuff::DamageLowMultiply(80),
                ],
                3 => &[
                    CombatStatBuff::CritChanceAdd(11),
                    CombatStatBuff::DamageHighMultiply(70),
                    CombatStatBuff::DamageLowMultiply(70),
                ],
                2.. => &[
                    CombatStatBuff::CritChanceAdd(10),
                    CombatStatBuff::DamageHighMultiply(60),
                    CombatStatBuff::DamageLowMultiply(60),
                ],
                0..=1 => &[
                    CombatStatBuff::CritChanceAdd(9),
                    CombatStatBuff::DamageHighMultiply(50),
                    CombatStatBuff::DamageLowMultiply(50),
                ],
            })
            .with_duration(Duration::Rounds(3)),
        Effect::SoloMarkSelf => EffectData::new(Target::Performer, ApplicationKind::QueueOnce)
            .with_status_effects(&[StatusEffect::Tag])
            .with_duration(Duration::Rounds(3))
            .apply_on_miss(),
        Effect::Stealth => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Stealth])
            .with_duration(Duration::Rounds(1)),
        Effect::StealthSelf => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_status_effects(&[StatusEffect::Stealth])
            .with_duration(Duration::Rounds(2)),
        Effect::StrongBleed(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotBleed(5)],
                3..=4 => &[StatusEffect::DotBleed(4)],
                0..=2 => &[StatusEffect::DotBleed(3)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::Stun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::StunKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::TargetStatus(Status::Stunned))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(60), CombatStatBuff::DamageLowMultiply(60)],
                4 => &[CombatStatBuff::DamageHighMultiply(50), CombatStatBuff::DamageLowMultiply(50)],
                3 => &[CombatStatBuff::DamageHighMultiply(40), CombatStatBuff::DamageLowMultiply(40)],
                2.. => &[CombatStatBuff::DamageHighMultiply(33), CombatStatBuff::DamageLowMultiply(33)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
            }),
        Effect::Suppression(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::AttackRatingAdd(-20), CombatStatBuff::CritChanceAdd(-19)],
                4 => &[CombatStatBuff::AttackRatingAdd(-18), CombatStatBuff::CritChanceAdd(-18)],
                3 => &[CombatStatBuff::AttackRatingAdd(-17), CombatStatBuff::CritChanceAdd(-17)],
                2 => &[CombatStatBuff::AttackRatingAdd(-16), CombatStatBuff::CritChanceAdd(-16)],
                0..=1 => &[CombatStatBuff::AttackRatingAdd(-15), CombatStatBuff::CritChanceAdd(-15)],
            })
            .with_duration(Duration::Rounds(2)),
        Effect::SwitchModeBeastSelf => {
            EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(&[StatusEffect::SetMode(Mode::Beast)])
        }
        Effect::SwitchModeHumanSelf => {
            EffectData::new(Target::Performer, ApplicationKind::Stack).with_status_effects(&[StatusEffect::SetMode(Mode::Human)])
        }
        Effect::TrackingBuff(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::AttackRatingAdd(10),
                    CombatStatBuff::CritChanceAdd(8),
                    CombatStatBuff::DamageHighMultiply(20),
                    CombatStatBuff::DamageLowMultiply(20),
                ],
                4 => &[
                    CombatStatBuff::AttackRatingAdd(9),
                    CombatStatBuff::CritChanceAdd(7),
                    CombatStatBuff::DamageHighMultiply(18),
                    CombatStatBuff::DamageLowMultiply(18),
                ],
                3 => &[
                    CombatStatBuff::AttackRatingAdd(8),
                    CombatStatBuff::CritChanceAdd(6),
                    CombatStatBuff::DamageHighMultiply(16),
                    CombatStatBuff::DamageLowMultiply(16),
                ],
                2 => &[
                    CombatStatBuff::AttackRatingAdd(7),
                    CombatStatBuff::CritChanceAdd(5),
                    CombatStatBuff::DamageHighMultiply(14),
                    CombatStatBuff::DamageLowMultiply(14),
                ],
                0..=1 => &[
                    CombatStatBuff::AttackRatingAdd(6),
                    CombatStatBuff::CritChanceAdd(4),
                    CombatStatBuff::DamageHighMultiply(12),
                    CombatStatBuff::DamageLowMultiply(12),
                ],
            })
            .with_duration(Duration::Combat)
            .apply_on_miss(),
        Effect::TransformHealSelf(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue).with_status_effects(match *lv {
            5.. => &[StatusEffect::Heal { amount: 10, is_skill: false }],
            4 => &[StatusEffect::Heal { amount: 8, is_skill: false }],
            3 => &[StatusEffect::Heal { amount: 7, is_skill: false }],
            2 => &[StatusEffect::Heal { amount: 6, is_skill: false }],
            0..=1 => &[StatusEffect::Heal { amount: 5, is_skill: false }],
        }),
        Effect::UnholyKiller(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_condition(Condition::MonsterType(MonsterType::Unholy))
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(35), CombatStatBuff::DamageLowMultiply(35)],
                4 => &[CombatStatBuff::DamageHighMultiply(30), CombatStatBuff::DamageLowMultiply(30)],
                3 => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                2.. => &[CombatStatBuff::DamageHighMultiply(20), CombatStatBuff::DamageLowMultiply(20)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(15), CombatStatBuff::DamageLowMultiply(15)],
            }),
        Effect::VestalHealSelf(lv) => EffectData::new(Target::Performer, ApplicationKind::Queue).with_status_effects(match *lv {
            3.. => &[StatusEffect::Heal { amount: 5, is_skill: true }],
            2 => &[StatusEffect::Heal { amount: 4, is_skill: true }],
            0..=1 => &[StatusEffect::Heal { amount: 3, is_skill: true }],
        }),
        Effect::VestalInspiration(lv) => EffectData::new(Target::Performer, ApplicationKind::Stack)
            .with_combat_stat_buffs(match *lv {
                5.. => &[
                    CombatStatBuff::AttackRatingAdd(10),
                    CombatStatBuff::DamageHighMultiply(35),
                    CombatStatBuff::DamageLowMultiply(35),
                ],
                4 => &[
                    CombatStatBuff::AttackRatingAdd(9),
                    CombatStatBuff::DamageHighMultiply(33),
                    CombatStatBuff::DamageLowMultiply(33),
                ],
                3 => &[
                    CombatStatBuff::AttackRatingAdd(8),
                    CombatStatBuff::DamageHighMultiply(30),
                    CombatStatBuff::DamageLowMultiply(30),
                ],
                2 => &[
                    CombatStatBuff::AttackRatingAdd(7),
                    CombatStatBuff::DamageHighMultiply(27),
                    CombatStatBuff::DamageLowMultiply(27),
                ],
                0..=1 => &[
                    CombatStatBuff::AttackRatingAdd(6),
                    CombatStatBuff::DamageHighMultiply(25),
                    CombatStatBuff::DamageLowMultiply(25),
                ],
            })
            .apply_on_miss(),
        Effect::VestalLight(lv) => EffectData::new(Target::Global, ApplicationKind::QueueOnce)
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::Torch(10)],
                4 => &[StatusEffect::Torch(8)],
                3 => &[StatusEffect::Torch(7)],
                2 => &[StatusEffect::Torch(6)],
                0..=1 => &[StatusEffect::Torch(5)],
            })
            .apply_on_miss(),
        Effect::VestalStun(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_status_effects(&[StatusEffect::Stun])
            .with_duration(Duration::Rounds(1)),
        Effect::VomitDebuff(lv) => EffectData::new(Target::Target, ApplicationKind::Stack)
            .with_chance(match *lv {
                5.. => 140,
                4 => 130,
                3 => 120,
                2 => 110,
                0..=1 => 100,
            })
            .with_buffs(match *lv {
                5.. => &[Buff::VomitBlightResistDebuff(5)],
                4 => &[Buff::VomitBlightResistDebuff(4)],
                3 => &[Buff::VomitBlightResistDebuff(3)],
                2 => &[Buff::VomitBlightResistDebuff(2)],
                0..=1 => &[Buff::VomitBlightResistDebuff(1)],
            }),
        Effect::WyrdBleed(lv) => EffectData::new(Target::Target, ApplicationKind::Queue)
            .with_chance(match *lv {
                5.. => 85,
                4 => 80,
                2..=3 => 70,
                0..=1 => 60,
            })
            .with_status_effects(match *lv {
                5.. => &[StatusEffect::DotBleed(3)],
                3..=4 => &[StatusEffect::DotBleed(2)],
                0..=2 => &[StatusEffect::DotBleed(1)],
            })
            .with_duration(Duration::Rounds(3))
            .apply_on_death(),
        Effect::XformDamage(lv) => EffectData::new(Target::Performer, ApplicationKind::ApplyOnce)
            .with_combat_stat_buffs(match *lv {
                5.. => &[CombatStatBuff::DamageHighMultiply(25), CombatStatBuff::DamageLowMultiply(25)],
                4 => &[CombatStatBuff::DamageHighMultiply(21), CombatStatBuff::DamageLowMultiply(21)],
                3 => &[CombatStatBuff::DamageHighMultiply(17), CombatStatBuff::DamageLowMultiply(17)],
                2.. => &[CombatStatBuff::DamageHighMultiply(14), CombatStatBuff::DamageLowMultiply(14)],
                0..=1 => &[CombatStatBuff::DamageHighMultiply(10), CombatStatBuff::DamageLowMultiply(10)],
            })
            .with_duration(Duration::Rounds(3)),
    }
}
