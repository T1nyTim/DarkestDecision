use crate::catalog::Difficulty;

#[derive(Clone)]
pub enum EnemyKind {
    AncestorSmall(Difficulty),
    BloatedCorpse(Difficulty),
    BrigandBlood(Difficulty),
    BrigandCannon(Difficulty),
    BrigandCutthroat(Difficulty),
    BrigandFuseman(Difficulty),
    BrigandFusilier(Difficulty),
    BrigandHunter(Difficulty),
    BrigandRaider(Difficulty),
    CarrionEater(Difficulty),
    CarrionEaterBig(Difficulty),
    CauldronEmpty(Difficulty),
    CellBattle(Difficulty),
    CellWhite(Difficulty),
    Collector(Difficulty),
    Crone(Difficulty),
    Crow(Difficulty),
    CultistBrawler(Difficulty),
    CultistHarpy(Difficulty),
    CultistOrgiastic(Difficulty),
    CultistShrouded(Difficulty),
    CultistWarlord(Difficulty),
    CultistWitch(Difficulty),
    Cyst(Difficulty),
    DrownedCaptain(Difficulty),
    DrownedPirate(Difficulty),
    Ectoplasm(Difficulty),
    EctoplasmLarge(Difficulty),
    ErrantFleshBat(Difficulty),
    ErrantFleshDog(Difficulty),
    FishmanCrabby(Difficulty),
    FishmanHarpoon(Difficulty),
    FishmanShaman(Difficulty),
    FormlessGuard(Difficulty),
    FormlessMelee(Difficulty),
    FormlessRanged(Difficulty),
    FormlessWeak(Difficulty),
    FungalArtillery(Difficulty),
    FungalBloat(Difficulty),
    Gargoyle(Difficulty),
    Ghoul(Difficulty),
    Hag(Difficulty),
    Jellyfish(Difficulty),
    Madman(Difficulty),
    Maggot(Difficulty),
    Necromancer(Difficulty),
    Nest(Difficulty),
    Octotank(Difficulty),
    PewLarge(Difficulty),
    PewMedium(Difficulty),
    PewSmall(Difficulty),
    Prophet(Difficulty),
    RabidDog(Difficulty),
    Shambler(Difficulty),
    Shuffler(Difficulty),
    Siren(Difficulty),
    SkeletonArbalist(Difficulty),
    SkeletonBearer(Difficulty),
    SkeletonCaptain(Difficulty),
    SkeletonCommon(Difficulty),
    SkeletonCourtier(Difficulty),
    SkeletonDefender(Difficulty),
    SkeletonMilitia(Difficulty),
    SkeletonSpear(Difficulty),
    SnailUrchin(Difficulty),
    SpiderSpitter(Difficulty),
    SpiderWebber(Difficulty),
    SwineDrummer(Difficulty),
    SwinePiglet(Difficulty),
    SwinePrince(Difficulty),
    SwineReaver(Difficulty),
    SwineSkiver(Difficulty),
    SwineSlasher(Difficulty),
    Swinetaur(Difficulty),
    SwineWretch(Difficulty),
    TemplarMelee(Difficulty),
    TemplarMeleeMb(Difficulty),
    TemplarRanged(Difficulty),
    TemplarRangedMb(Difficulty),
    TotemAttack(Difficulty),
    TotemGuard(Difficulty),
    UncleanGiant(Difficulty),
    ViragoHateful(Difficulty),
}

impl EnemyKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::AncestorSmall(Difficulty::Darkest) => "Ancestor",
            Self::BloatedCorpse(Difficulty::Apprentice) => "Drowned Thrall",
            Self::BloatedCorpse(Difficulty::Veteran) => "Pustulent Thrall",
            Self::BloatedCorpse(Difficulty::Champion) => "Bloated Thrall",
            Self::BrigandBlood(Difficulty::Apprentice) => "Brigand Bloodletter",
            Self::BrigandBlood(Difficulty::Veteran) => "Outlaw Bloodletter",
            Self::BrigandBlood(Difficulty::Champion) => "Infamous Bloodletter",
            Self::BrigandCannon(Difficulty::Apprentice) => "Brigand 8 Pounder",
            Self::BrigandCannon(Difficulty::Veteran) => "Brigand 12 Pounder",
            Self::BrigandCannon(Difficulty::Champion) => "Brigand 16 Pounder",
            Self::BrigandCutthroat(Difficulty::Apprentice) => "Brigand Cutthroat",
            Self::BrigandCutthroat(Difficulty::Veteran) => "Outlaw Cutthroat",
            Self::BrigandCutthroat(Difficulty::Champion) => "Infamous Cutthroat",
            Self::BrigandFuseman(Difficulty::Apprentice) => "Brigand Matchman",
            Self::BrigandFuseman(Difficulty::Veteran) => "Brigand Fuseman",
            Self::BrigandFuseman(Difficulty::Champion) => "Brigand Pyro",
            Self::BrigandFusilier(Difficulty::Apprentice) => "Brigand Fusilier",
            Self::BrigandFusilier(Difficulty::Veteran) => "Outlaw Fusilier",
            Self::BrigandFusilier(Difficulty::Champion) => "Infamous Fusilier",
            Self::BrigandHunter(Difficulty::Darkest) => "Brigand Hunter",
            Self::BrigandRaider(Difficulty::Darkest) => "Brigand Raider",
            Self::CarrionEater(Difficulty::Apprentice) => "Carrion Eater",
            Self::CarrionEater(Difficulty::Veteran) => "Corpse Eater",
            Self::CarrionEater(Difficulty::Champion) => "Flesh Eater",
            Self::CarrionEaterBig(Difficulty::Veteran) => "Large Corpse Eater",
            Self::CarrionEaterBig(Difficulty::Champion) => "Large Flesh Eater",
            Self::CauldronEmpty(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Cauldron (empty)",
            Self::CellBattle(Difficulty::Darkest) => "Antibody",
            Self::CellWhite(Difficulty::Darkest) => "White Cell Stalk",
            Self::Collector(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "The Collector",
            Self::Crone(Difficulty::Veteran) => "Tenacious Crone",
            Self::Crone(Difficulty::Champion) => "Blighted Crone",
            Self::Crow(Difficulty::Apprentice) => "Callous Shrieker",
            Self::Crow(Difficulty::Veteran) => "Cretinous Shrieker",
            Self::Crow(Difficulty::Champion) => "Horrid Shrieker",
            Self::CultistBrawler(Difficulty::Apprentice) => "Cultist Brawler",
            Self::CultistBrawler(Difficulty::Veteran) => "Cultist Gladiator",
            Self::CultistBrawler(Difficulty::Champion) => "Cultist Champion",
            Self::CultistHarpy(Difficulty::Darkest) => "Ascended Witch",
            Self::CultistOrgiastic(Difficulty::Darkest) => "Rapturous Cultist",
            Self::CultistShrouded(Difficulty::Darkest) => "Cultist Priest",
            Self::CultistWarlord(Difficulty::Darkest) => "Ascended Brawler",
            Self::CultistWitch(Difficulty::Apprentice) => "Cultist Acolyte",
            Self::CultistWitch(Difficulty::Veteran) => "Cultist Enchantress",
            Self::CultistWitch(Difficulty::Champion) => "Cultist Witch",
            Self::Cyst(Difficulty::Darkest) => "Mammoth Cyst",
            Self::DrownedCaptain(Difficulty::Apprentice) => "Sodden Crew",
            Self::DrownedCaptain(Difficulty::Veteran) => "Sunken Crew",
            Self::DrownedCaptain(Difficulty::Champion) => "Drowned Crew",
            Self::DrownedPirate(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Squiffy Ghast",
            Self::Ectoplasm(Difficulty::Apprentice) => "Ectoplasm",
            Self::Ectoplasm(Difficulty::Veteran) => "Quivering Ectoplasm",
            Self::Ectoplasm(Difficulty::Champion) => "Cohesive Ectoplasm",
            Self::EctoplasmLarge(Difficulty::Veteran) => "Towering Ectoplasm",
            Self::EctoplasmLarge(Difficulty::Champion) => "Giant Ectoplasm",
            Self::ErrantFleshBat(Difficulty::Darkest) => "Polyp",
            Self::ErrantFleshDog(Difficulty::Darkest) => "Flesh Hound",
            Self::FishmanCrabby(Difficulty::Veteran) => "Uca Crusher",
            Self::FishmanCrabby(Difficulty::Champion) => "Uca Savage",
            Self::FishmanHarpoon(Difficulty::Apprentice) => "Pelagic Grouper",
            Self::FishmanHarpoon(Difficulty::Veteran) => "Pelagic Widemouth",
            Self::FishmanHarpoon(Difficulty::Champion) => "Pelagic Piranha",
            Self::FishmanShaman(Difficulty::Apprentice) => "Pelagic Shaman",
            Self::FishmanShaman(Difficulty::Veteran) => "Pelagic Oracle",
            Self::FishmanShaman(Difficulty::Champion) => "Pelagic Tidemaster",
            Self::FormlessGuard(difficulty) | Self::FormlessMelee(difficulty) | Self::FormlessRanged(difficulty) | Self::FormlessWeak(difficulty)
                if matches!(difficulty, Difficulty::Apprentice) =>
            {
                "Inchoate Flesh"
            }
            Self::FormlessGuard(difficulty) | Self::FormlessMelee(difficulty) | Self::FormlessRanged(difficulty) | Self::FormlessWeak(difficulty)
                if matches!(difficulty, Difficulty::Veteran) =>
            {
                "Unstable Flesh"
            }
            Self::FormlessGuard(difficulty) | Self::FormlessMelee(difficulty) | Self::FormlessRanged(difficulty) | Self::FormlessWeak(difficulty)
                if matches!(difficulty, Difficulty::Champion) =>
            {
                "Formless Flesh"
            }
            Self::FungalArtillery(Difficulty::Apprentice) => "Fungal Artillery",
            Self::FungalArtillery(Difficulty::Veteran) => "Fungal Battery",
            Self::FungalArtillery(Difficulty::Champion) => "Fungal Bombard",
            Self::FungalBloat(Difficulty::Apprentice) => "Fungal Scratcher",
            Self::FungalBloat(Difficulty::Veteran) => "Fungal Grabber",
            Self::FungalBloat(Difficulty::Champion) => "Fungal Clawer",
            Self::Gargoyle(Difficulty::Apprentice | Difficulty::Veteran) => "Gargoyle",
            Self::Gargoyle(Difficulty::Champion) => "Menacing Gargoyle",
            Self::Ghoul(Difficulty::Veteran) => "Slavering Ghoul",
            Self::Ghoul(Difficulty::Champion) => "Insatiable Ghoul",
            Self::Hag(Difficulty::Apprentice) => "Wizened Hag",
            Self::Hag(Difficulty::Veteran) => "Hag",
            Self::Hag(Difficulty::Champion) => "Hag Witch",
            Self::Jellyfish(Difficulty::Apprentice) => "Deep Stinger",
            Self::Jellyfish(Difficulty::Veteran) => "Energized Stinger",
            Self::Jellyfish(Difficulty::Champion) => "Paralyzing Stinger",
            Self::Madman(Difficulty::Apprentice) => "Madman",
            Self::Madman(Difficulty::Veteran) => "Frothing Madman",
            Self::Madman(Difficulty::Champion) => "Raving Madman",
            Self::Maggot(Difficulty::Apprentice) => "Maggot",
            Self::Maggot(Difficulty::Veteran) => "Twitching Maggot",
            Self::Maggot(Difficulty::Champion) => "Bulbous Maggot",
            Self::Necromancer(Difficulty::Apprentice) => "Necromancer Apprentice",
            Self::Necromancer(Difficulty::Veteran) => "Necromancer",
            Self::Necromancer(Difficulty::Champion) => "Necromancer Lord",
            Self::Nest(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Shrieker's Nest",
            Self::Octotank(Difficulty::Apprentice) => "Pelagic Guardian",
            Self::Octotank(Difficulty::Veteran) => "Pelagic Bulwark",
            Self::Octotank(Difficulty::Champion) => "Pelagic Champion",
            Self::PewLarge(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Pew Blockade",
            Self::PewMedium(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Fractured Pew",
            Self::PewSmall(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Dashed Pew",
            Self::Prophet(Difficulty::Apprentice) => "Sonorous Prophet",
            Self::Prophet(Difficulty::Veteran) => "Fulminating Prophet",
            Self::Prophet(Difficulty::Champion) => "Gibbering Prophet",
            Self::RabidDog(Difficulty::Apprentice) => "Rabid Gnasher",
            Self::RabidDog(Difficulty::Veteran) => "Crazed Gnasher",
            Self::RabidDog(Difficulty::Champion) => "Harrying Gnasher",
            Self::Shambler(Difficulty::Apprentice) => "Shambler",
            Self::Shambler(Difficulty::Veteran) => "Grotesque Shambler",
            Self::Shambler(Difficulty::Champion) => "Abhorrent Shambler",
            Self::Shuffler(Difficulty::Darkest) => "Shuffling Horror",
            Self::Siren(Difficulty::Apprentice) => "Siren",
            Self::Siren(Difficulty::Veteran) => "Alluring Siren",
            Self::Siren(Difficulty::Champion) => "Beguiling Siren",
            Self::SkeletonArbalist(Difficulty::Apprentice) => "Bone Arbalist",
            Self::SkeletonArbalist(Difficulty::Veteran) => "Bone Marksman",
            Self::SkeletonArbalist(Difficulty::Champion) => "Bone Sharpshooter",
            Self::SkeletonBearer(Difficulty::Champion) => "Bone Bearer",
            Self::SkeletonCaptain(Difficulty::Veteran) => "Bone Commander",
            Self::SkeletonCaptain(Difficulty::Champion) => "Bone General",
            Self::SkeletonCommon(Difficulty::Apprentice) => "Bone Rabble",
            Self::SkeletonCommon(Difficulty::Veteran) => "Bone Conscript",
            Self::SkeletonCommon(Difficulty::Champion) => "Bone Militia",
            Self::SkeletonCourtier(Difficulty::Apprentice) => "Bone Courtier",
            Self::SkeletonCourtier(Difficulty::Veteran) => "Bone Noble",
            Self::SkeletonCourtier(Difficulty::Champion) => "Bone Royalty",
            Self::SkeletonDefender(Difficulty::Apprentice) => "Bone Defender",
            Self::SkeletonDefender(Difficulty::Veteran) => "Bone Bulwark",
            Self::SkeletonDefender(Difficulty::Champion) => "Bone Shieldwall",
            Self::SkeletonMilitia(Difficulty::Apprentice) => "Bone Soldier",
            Self::SkeletonMilitia(Difficulty::Veteran) => "Bone Veteran",
            Self::SkeletonMilitia(Difficulty::Champion) => "Bone Sergeant",
            Self::SkeletonSpear(Difficulty::Veteran) => "Bone Spearman",
            Self::SkeletonSpear(Difficulty::Champion) => "Bone Lancer",
            Self::SnailUrchin(Difficulty::Apprentice) => "Sea Maggot",
            Self::SnailUrchin(Difficulty::Veteran) => "Plated Maggot",
            Self::SnailUrchin(Difficulty::Champion) => "Armored Maggot",
            Self::SpiderSpitter(Difficulty::Apprentice) => "Spitter",
            Self::SpiderSpitter(Difficulty::Veteran) => "Overgrown Spitter",
            Self::SpiderSpitter(Difficulty::Champion) => "Ancient Spitter",
            Self::SpiderWebber(Difficulty::Apprentice) => "Webber",
            Self::SpiderWebber(Difficulty::Veteran) => "Overgrown Webber",
            Self::SpiderWebber(Difficulty::Champion) => "Ancient Webber",
            Self::SwineDrummer(Difficulty::Apprentice) => "Swine Drummer",
            Self::SwineDrummer(Difficulty::Veteran) => "Swine Marcher",
            Self::SwineDrummer(Difficulty::Champion) => "Swine Thunderer",
            Self::SwinePiglet(Difficulty::Apprentice | Difficulty::Veteran | Difficulty::Champion) => "Wilbur",
            Self::SwinePrince(Difficulty::Apprentice) => "Swine Prince",
            Self::SwinePrince(Difficulty::Veteran) => "Swine King",
            Self::SwinePrince(Difficulty::Champion) => "Swine God",
            Self::SwineReaver(Difficulty::Apprentice) => "Swine Chopper",
            Self::SwineReaver(Difficulty::Veteran) => "Swine Reaver",
            Self::SwineReaver(Difficulty::Champion) => "Swine Slayer",
            Self::SwineSkiver(Difficulty::Champion) => "Swine Skiver",
            Self::SwineSlasher(Difficulty::Apprentice) => "Swine Slasher",
            Self::SwineSlasher(Difficulty::Veteran) => "Swine Gorer",
            Self::SwineSlasher(Difficulty::Champion) => "Swine Meathooker",
            Self::Swinetaur(Difficulty::Veteran) => "Hulking Swinetaur",
            Self::Swinetaur(Difficulty::Champion) => "Swinetaur Champion",
            Self::SwineWretch(Difficulty::Apprentice) => "Swine Wretch",
            Self::SwineWretch(Difficulty::Veteran) => "Swine Spawn",
            Self::SwineWretch(Difficulty::Champion) => "Swine Heaver",
            Self::TemplarMelee(Difficulty::Darkest) => "Templar Gladiator",
            Self::TemplarMeleeMb(Difficulty::Darkest) => "Templar Impaler",
            Self::TemplarRanged(Difficulty::Darkest) => "Templar Sniper",
            Self::TemplarRangedMb(Difficulty::Darkest) => "Templar Warlord",
            Self::TotemAttack(Difficulty::Darkest) => "Malignant Growth",
            Self::TotemGuard(Difficulty::Darkest) => "Defensive Growth",
            Self::UncleanGiant(Difficulty::Veteran) => "Blighted Giant",
            Self::UncleanGiant(Difficulty::Champion) => "Corrupted Giant",
            Self::ViragoHateful(Difficulty::Champion) => "Hateful Virago",
            Self::AncestorSmall(_)
            | Self::BloatedCorpse(_)
            | Self::BrigandBlood(_)
            | Self::BrigandCannon(_)
            | Self::BrigandCutthroat(_)
            | Self::BrigandFuseman(_)
            | Self::BrigandFusilier(_)
            | Self::BrigandHunter(_)
            | Self::BrigandRaider(_)
            | Self::CarrionEater(_)
            | Self::CarrionEaterBig(_)
            | Self::CauldronEmpty(_)
            | Self::CellBattle(_)
            | Self::CellWhite(_)
            | Self::Collector(_)
            | Self::Crone(_)
            | Self::Crow(_)
            | Self::CultistBrawler(_)
            | Self::CultistHarpy(_)
            | Self::CultistOrgiastic(_)
            | Self::CultistShrouded(_)
            | Self::CultistWarlord(_)
            | Self::CultistWitch(_)
            | Self::Cyst(_)
            | Self::DrownedCaptain(_)
            | Self::DrownedPirate(_)
            | Self::Ectoplasm(_)
            | Self::EctoplasmLarge(_)
            | Self::ErrantFleshBat(_)
            | Self::ErrantFleshDog(_)
            | Self::FishmanCrabby(_)
            | Self::FishmanHarpoon(_)
            | Self::FishmanShaman(_)
            | Self::FormlessGuard(_)
            | Self::FormlessMelee(_)
            | Self::FormlessRanged(_)
            | Self::FormlessWeak(_)
            | Self::FungalArtillery(_)
            | Self::FungalBloat(_)
            | Self::Gargoyle(_)
            | Self::Ghoul(_)
            | Self::Hag(_)
            | Self::Jellyfish(_)
            | Self::Madman(_)
            | Self::Maggot(_)
            | Self::Necromancer(_)
            | Self::Nest(_)
            | Self::Octotank(_)
            | Self::PewLarge(_)
            | Self::PewMedium(_)
            | Self::PewSmall(_)
            | Self::Prophet(_)
            | Self::RabidDog(_)
            | Self::Shambler(_)
            | Self::Shuffler(_)
            | Self::Siren(_)
            | Self::SkeletonArbalist(_)
            | Self::SkeletonBearer(_)
            | Self::SkeletonCaptain(_)
            | Self::SkeletonCommon(_)
            | Self::SkeletonCourtier(_)
            | Self::SkeletonDefender(_)
            | Self::SkeletonMilitia(_)
            | Self::SkeletonSpear(_)
            | Self::SnailUrchin(_)
            | Self::SpiderSpitter(_)
            | Self::SpiderWebber(_)
            | Self::SwineDrummer(_)
            | Self::SwinePiglet(_)
            | Self::SwinePrince(_)
            | Self::SwineReaver(_)
            | Self::SwineSkiver(_)
            | Self::SwineSlasher(_)
            | Self::Swinetaur(_)
            | Self::SwineWretch(_)
            | Self::TemplarMelee(_)
            | Self::TemplarMeleeMb(_)
            | Self::TemplarRanged(_)
            | Self::TemplarRangedMb(_)
            | Self::TotemAttack(_)
            | Self::TotemGuard(_)
            | Self::UncleanGiant(_)
            | Self::ViragoHateful(_) => panic!("Matched enemy doesn't exist in the game"),
        }
    }
}
