pub mod consts;
pub mod effect;

enum ApplicationKind {
    ApplyOnce,
    Queue,
    QueueOnce,
    Stack,
}

enum Buff {
    AntiqBlightBuff(u8),
    AntiqBlightDebuff(u8),
    BleedResist(u8),
    BlightResist(u8),
    BuildToFinaleDmgH(u8),
    BuildToFinaleDmgL(u8),
    CaltropsDmgReceived(u8),
    MaaBellowCritReceived,
    MaaCommandAcc(u8),
    MaaCommandCrit(u8),
    MaaCommandGuardedDmgH(u8),
    MaaCommandGuardedDmgL(u8),
    StressDmg(i8),
    OnCritAcc,
    OnCritBleedChance,
    OnCritBlightChance,
    OnCritDef,
    OnCritDmgH,
    OnCritDmgHBleeding,
    OnCritDmgHMarked,
    OnCritDmgL,
    OnCritDmgLBleeding,
    OnCritDmgLMarked,
    OnCritHealBuff,
    OnCritProt,
    OnCritSpd,
    OnCritStressHealBuff,
    OnCritStressResist,
}

impl Buff {
    const fn amount(&self) -> i8 {
        match self {
            Self::OnCritStressResist => -33,
            Self::OnCritSpd => 2,
            Self::OnCritAcc | Self::OnCritDef => 10,
            Self::OnCritProt => 15,
            Self::OnCritBleedChance | Self::OnCritBlightChance | Self::OnCritDmgH | Self::OnCritDmgL => 20,
            Self::OnCritHealBuff => 25,
            Self::OnCritDmgHBleeding | Self::OnCritDmgHMarked | Self::OnCritDmgLBleeding | Self::OnCritDmgLMarked => 33,
            Self::OnCritStressHealBuff => 40,
        }
    }

    const fn buff_kind(&self) -> BuffKind {
        match self {
            Self::OnCritBleedChance => BuffKind::BleedChance,
            Self::OnCritAcc => BuffKind::CombatStatAdd(Stat::AttackRating),
            Self::OnCritDef => BuffKind::CombatStatAdd(Stat::DefenseRating),
            Self::OnCritProt => BuffKind::CombatStatAdd(Stat::ProtectionRating),
            Self::OnCritSpd => BuffKind::CombatStatAdd(Stat::SpeedRating),
            Self::OnCritDmgH | Self::OnCritDmgHBleeding | Self::OnCritDmgHMarked => BuffKind::CombatStatMultiply(Stat::DamageHigh),
            Self::OnCritDmgL | Self::OnCritDmgLBleeding | Self::OnCritDmgLMarked => BuffKind::CombatStatMultiply(Stat::DamageLow),
            Self::OnCritHealBuff => BuffKind::HpHealPercent,
            Self::OnCritBlightChance => BuffKind::PoisonChance,
            Self::OnCritStressResist => BuffKind::StressDmgReceivedPercent,
            Self::OnCritStressHealBuff => BuffKind::StressHealPercent,
        }
    }

    const fn condition(&self) -> Condition {
        match self {
            Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmgH
            | Self::OnCritDmgL
            | Self::OnCritHealBuff
            | Self::OnCritProt
            | Self::OnCritSpd
            | Self::OnCritStressHealBuff
            | Self::OnCritStressResist => Condition::Always,
            Self::OnCritDmgHBleeding | Self::OnCritDmgLBleeding => Condition::TargetStatus(Status::Bleeding),
            Self::OnCritDmgHMarked | Self::OnCritDmgLMarked => Condition::TargetStatus(Status::Tagged),
        }
    }

    const fn duration(&self) -> u8 {
        match self {
            Self::OnCritAcc
            | Self::OnCritBleedChance
            | Self::OnCritBlightChance
            | Self::OnCritDef
            | Self::OnCritDmgH
            | Self::OnCritDmgHBleeding
            | Self::OnCritDmgHMarked
            | Self::OnCritDmgL
            | Self::OnCritDmgLBleeding
            | Self::OnCritDmgLMarked
            | Self::OnCritProt
            | Self::OnCritSpd => 2,
            Self::OnCritHealBuff | Self::OnCritStressHealBuff | Self::OnCritStressResist => 3,
        }
    }
}

enum BuffKind {
    BleedChance,
    CombatStatAdd(Stat),
    CombatStatMultiply(Stat),
    HpHealPercent,
    PoisonChance,
    StressDmgReceivedPercent,
    StressHealPercent,
}

enum CombatStatBuff {
    AttackRatingAdd(i8),
    DamageLowMultiply(u8),
    DamageHighMultiply(u8),
    DefenseRatingAdd(u8),
    ProtectionRatingAdd(u8),
}

enum Condition {
    Always,
    MonsterType(MonsterType),
    TargetStatus(Status),
}

enum Duration {
    Rounds(u8),
}

enum MonsterType {
    Eldritch,
}

enum Stat {
    AttackRating,
    DamageHigh,
    DamageLow,
    DefenseRating,
    ProtectionRating,
    SpeedRating,
}

enum Status {
    Bleeding,
    Tagged,
}

enum StatusEffect {
    ClearGuarded,
    ClearGuarding,
    Cure,
    DotPoison(u8),
    Guard,
    Stun,
}

enum Target {
    Performer,
    Target,
}
