use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq)]
pub enum EncounterId {
    #[serde(rename = "tutorial-a-h-1")]
    TutorialAH1,
    #[serde(rename = "tutorial-a-r-1")]
    TutorialAR1,
    #[serde(rename = "cove-a-h-1")]
    CoveAH1,
    #[serde(rename = "cove-a-h-2")]
    CoveAH2,
    #[serde(rename = "cove-a-h-3")]
    CoveAH3,
    #[serde(rename = "cove-a-h-4")]
    CoveAH4,
    #[serde(rename = "cove-a-h-5")]
    CoveAH5,
    #[serde(rename = "cove-a-h-6")]
    CoveAH6,
    #[serde(rename = "cove-a-h-7")]
    CoveAH7,
    #[serde(rename = "cove-a-h-8")]
    CoveAH8,
    #[serde(rename = "cove-a-h-9")]
    CoveAH9,
    #[serde(rename = "cove-a-h-10")]
    CoveAH10,
    #[serde(rename = "cove-a-h-11")]
    CoveAH11,
    #[serde(rename = "cove-a-h-12")]
    CoveAH12,
    #[serde(rename = "cove-a-h-13")]
    CoveAH13,
    #[serde(rename = "cove-a-h-14")]
    CoveAH14,
    #[serde(rename = "cove-a-h-15")]
    CoveAH15,
    #[serde(rename = "cove-a-h-16")]
    CoveAH16,
    #[serde(rename = "cove-a-h-17")]
    CoveAH17,
    #[serde(rename = "cove-a-h-18")]
    CoveAH18,
    #[serde(rename = "cove-a-h-19")]
    CoveAH19,
    #[serde(rename = "cove-a-h-20")]
    CoveAH20,
    #[serde(rename = "cove-a-h-21")]
    CoveAH21,
    #[serde(rename = "cove-a-h-22")]
    CoveAH22,
    #[serde(rename = "cove-a-r-1")]
    CoveAR1,
    #[serde(rename = "cove-a-r-2")]
    CoveAR2,
    #[serde(rename = "cove-a-r-3")]
    CoveAR3,
    #[serde(rename = "cove-a-r-4")]
    CoveAR4,
    #[serde(rename = "cove-a-r-5")]
    CoveAR5,
    #[serde(rename = "cove-a-r-6")]
    CoveAR6,
    #[serde(rename = "cove-a-r-7")]
    CoveAR7,
    #[serde(rename = "cove-a-r-8")]
    CoveAR8,
    #[serde(rename = "cove-a-r-9")]
    CoveAR9,
    #[serde(rename = "cove-a-r-10")]
    CoveAR10,
    #[serde(rename = "cove-a-r-11")]
    CoveAR11,
    #[serde(rename = "cove-a-r-12")]
    CoveAR12,
    #[serde(rename = "cove-a-r-13")]
    CoveAR13,
    #[serde(rename = "cove-a-r-14")]
    CoveAR14,
    #[serde(rename = "cove-a-r-15")]
    CoveAR15,
    #[serde(rename = "cove-a-r-16")]
    CoveAR16,
    #[serde(rename = "cove-a-r-17")]
    CoveAR17,
    #[serde(rename = "cove-a-r-18")]
    CoveAR18,
    #[serde(rename = "cove-a-r-19")]
    CoveAR19,
    #[serde(rename = "cove-a-r-20")]
    CoveAR20,
    #[serde(rename = "cove-v-h-1")]
    CoveVH1,
    #[serde(rename = "cove-v-h-2")]
    CoveVH2,
    #[serde(rename = "cove-v-h-3")]
    CoveVH3,
    #[serde(rename = "cove-v-h-4")]
    CoveVH4,
    #[serde(rename = "cove-v-h-5")]
    CoveVH5,
    #[serde(rename = "cove-v-h-6")]
    CoveVH6,
    #[serde(rename = "cove-v-h-7")]
    CoveVH7,
    #[serde(rename = "cove-v-h-8")]
    CoveVH8,
    #[serde(rename = "cove-v-h-9")]
    CoveVH9,
    #[serde(rename = "cove-v-h-10")]
    CoveVH10,
    #[serde(rename = "cove-v-h-11")]
    CoveVH11,
    #[serde(rename = "cove-v-h-12")]
    CoveVH12,
    #[serde(rename = "cove-v-h-13")]
    CoveVH13,
    #[serde(rename = "cove-v-h-14")]
    CoveVH14,
    #[serde(rename = "cove-v-h-15")]
    CoveVH15,
    #[serde(rename = "cove-v-h-16")]
    CoveVH16,
    #[serde(rename = "cove-v-h-17")]
    CoveVH17,
    #[serde(rename = "cove-v-h-18")]
    CoveVH18,
    #[serde(rename = "cove-v-h-19")]
    CoveVH19,
    #[serde(rename = "cove-v-h-20")]
    CoveVH20,
    #[serde(rename = "cove-v-h-21")]
    CoveVH21,
    #[serde(rename = "cove-v-h-22")]
    CoveVH22,
    #[serde(rename = "cove-v-h-23")]
    CoveVH23,
    #[serde(rename = "cove-v-h-24")]
    CoveVH24,
    #[serde(rename = "cove-v-h-25")]
    CoveVH25,
    #[serde(rename = "cove-v-h-26")]
    CoveVH26,
    #[serde(rename = "cove-v-h-27")]
    CoveVH27,
    #[serde(rename = "cove-v-h-28")]
    CoveVH28,
    #[serde(rename = "cove-v-h-29")]
    CoveVH29,
    #[serde(rename = "cove-v-r-1")]
    CoveVR1,
    #[serde(rename = "cove-v-r-2")]
    CoveVR2,
    #[serde(rename = "cove-v-r-3")]
    CoveVR3,
    #[serde(rename = "cove-v-r-4")]
    CoveVR4,
    #[serde(rename = "cove-v-r-5")]
    CoveVR5,
    #[serde(rename = "cove-v-r-6")]
    CoveVR6,
    #[serde(rename = "cove-v-r-7")]
    CoveVR7,
    #[serde(rename = "cove-v-r-8")]
    CoveVR8,
    #[serde(rename = "cove-v-r-9")]
    CoveVR9,
    #[serde(rename = "cove-v-r-10")]
    CoveVR10,
    #[serde(rename = "cove-v-r-11")]
    CoveVR11,
    #[serde(rename = "cove-v-r-12")]
    CoveVR12,
    #[serde(rename = "cove-v-r-13")]
    CoveVR13,
    #[serde(rename = "cove-v-r-14")]
    CoveVR14,
    #[serde(rename = "cove-v-r-15")]
    CoveVR15,
    #[serde(rename = "cove-v-r-16")]
    CoveVR16,
    #[serde(rename = "cove-v-r-17")]
    CoveVR17,
    #[serde(rename = "cove-v-r-18")]
    CoveVR18,
    #[serde(rename = "cove-v-r-19")]
    CoveVR19,
    #[serde(rename = "cove-v-r-20")]
    CoveVR20,
    #[serde(rename = "cove-v-r-21")]
    CoveVR21,
    #[serde(rename = "cove-v-r-22")]
    CoveVR22,
    #[serde(rename = "cove-v-r-23")]
    CoveVR23,
    #[serde(rename = "cove-v-r-24")]
    CoveVR24,
    #[serde(rename = "cove-c-h-1")]
    CoveCH1,
    #[serde(rename = "cove-c-h-2")]
    CoveCH2,
    #[serde(rename = "cove-c-h-3")]
    CoveCH3,
    #[serde(rename = "cove-c-h-4")]
    CoveCH4,
    #[serde(rename = "cove-c-h-5")]
    CoveCH5,
    #[serde(rename = "cove-c-h-6")]
    CoveCH6,
    #[serde(rename = "cove-c-h-7")]
    CoveCH7,
    #[serde(rename = "cove-c-h-8")]
    CoveCH8,
    #[serde(rename = "cove-c-h-9")]
    CoveCH9,
    #[serde(rename = "cove-c-h-10")]
    CoveCH10,
    #[serde(rename = "cove-c-h-11")]
    CoveCH11,
    #[serde(rename = "cove-c-h-12")]
    CoveCH12,
    #[serde(rename = "cove-c-h-13")]
    CoveCH13,
    #[serde(rename = "cove-c-h-14")]
    CoveCH14,
    #[serde(rename = "cove-c-h-15")]
    CoveCH15,
    #[serde(rename = "cove-c-h-16")]
    CoveCH16,
    #[serde(rename = "cove-c-h-17")]
    CoveCH17,
    #[serde(rename = "cove-c-h-18")]
    CoveCH18,
    #[serde(rename = "cove-c-h-19")]
    CoveCH19,
    #[serde(rename = "cove-c-h-20")]
    CoveCH20,
    #[serde(rename = "cove-c-h-21")]
    CoveCH21,
    #[serde(rename = "cove-c-h-22")]
    CoveCH22,
    #[serde(rename = "cove-c-h-23")]
    CoveCH23,
    #[serde(rename = "cove-c-h-24")]
    CoveCH24,
    #[serde(rename = "cove-c-h-25")]
    CoveCH25,
    #[serde(rename = "cove-c-h-26")]
    CoveCH26,
    #[serde(rename = "cove-c-h-27")]
    CoveCH27,
    #[serde(rename = "cove-c-h-28")]
    CoveCH28,
    #[serde(rename = "cove-c-h-29")]
    CoveCH29,
    #[serde(rename = "cove-c-h-30")]
    CoveCH30,
    #[serde(rename = "cove-c-h-31")]
    CoveCH31,
    #[serde(rename = "cove-c-h-32")]
    CoveCH32,
    #[serde(rename = "cove-c-h-33")]
    CoveCH33,
    #[serde(rename = "cove-c-h-34")]
    CoveCH34,
    #[serde(rename = "cove-c-r-1")]
    CoveCR1,
    #[serde(rename = "cove-c-r-2")]
    CoveCR2,
    #[serde(rename = "cove-c-r-3")]
    CoveCR3,
    #[serde(rename = "cove-c-r-4")]
    CoveCR4,
    #[serde(rename = "cove-c-r-5")]
    CoveCR5,
    #[serde(rename = "cove-c-r-6")]
    CoveCR6,
    #[serde(rename = "cove-c-r-7")]
    CoveCR7,
    #[serde(rename = "cove-c-r-8")]
    CoveCR8,
    #[serde(rename = "cove-c-r-9")]
    CoveCR9,
    #[serde(rename = "cove-c-r-10")]
    CoveCR10,
    #[serde(rename = "cove-c-r-11")]
    CoveCR11,
    #[serde(rename = "cove-c-r-12")]
    CoveCR12,
    #[serde(rename = "cove-c-r-13")]
    CoveCR13,
    #[serde(rename = "cove-c-r-14")]
    CoveCR14,
    #[serde(rename = "cove-c-r-15")]
    CoveCR15,
    #[serde(rename = "cove-c-r-16")]
    CoveCR16,
    #[serde(rename = "cove-c-r-17")]
    CoveCR17,
    #[serde(rename = "cove-c-r-18")]
    CoveCR18,
    #[serde(rename = "cove-c-r-19")]
    CoveCR19,
    #[serde(rename = "cove-c-r-20")]
    CoveCR20,
    #[serde(rename = "cove-c-r-21")]
    CoveCR21,
    #[serde(rename = "cove-c-r-22")]
    CoveCR22,
    #[serde(rename = "cove-c-r-23")]
    CoveCR23,
    #[serde(rename = "cove-c-r-24")]
    CoveCR24,
    #[serde(rename = "cove-c-r-25")]
    CoveCR25,
    #[serde(rename = "cove-c-r-26")]
    CoveCR26,
    #[serde(rename = "cove-c-r-27")]
    CoveCR27,
    #[serde(rename = "cove-c-r-28")]
    CoveCR28,
    #[serde(rename = "cove-c-r-29")]
    CoveCR29,
    #[serde(rename = "cove-c-r-30")]
    CoveCR30,
    #[serde(rename = "cove-c-r-31")]
    CoveCR31,
    #[serde(rename = "crypts-a-h-1")]
    CryptsAH1,
    #[serde(rename = "crypts-a-h-2")]
    CryptsAH2,
    #[serde(rename = "crypts-a-h-3")]
    CryptsAH3,
    #[serde(rename = "crypts-a-h-4")]
    CryptsAH4,
    #[serde(rename = "crypts-a-h-5")]
    CryptsAH5,
    #[serde(rename = "crypts-a-h-6")]
    CryptsAH6,
    #[serde(rename = "crypts-a-h-7")]
    CryptsAH7,
    #[serde(rename = "crypts-a-h-8")]
    CryptsAH8,
    #[serde(rename = "crypts-a-h-9")]
    CryptsAH9,
    #[serde(rename = "crypts-a-h-10")]
    CryptsAH10,
    #[serde(rename = "crypts-a-h-11")]
    CryptsAH11,
    #[serde(rename = "crypts-a-h-12")]
    CryptsAH12,
    #[serde(rename = "crypts-a-h-13")]
    CryptsAH13,
    #[serde(rename = "crypts-a-h-14")]
    CryptsAH14,
    #[serde(rename = "crypts-a-h-15")]
    CryptsAH15,
    #[serde(rename = "crypts-a-h-16")]
    CryptsAH16,
    #[serde(rename = "crypts-a-h-17")]
    CryptsAH17,
    #[serde(rename = "crypts-a-h-18")]
    CryptsAH18,
    #[serde(rename = "crypts-a-h-19")]
    CryptsAH19,
    #[serde(rename = "crypts-a-h-20")]
    CryptsAH20,
    #[serde(rename = "crypts-a-h-21")]
    CryptsAH21,
    #[serde(rename = "crypts-a-h-22")]
    CryptsAH22,
    #[serde(rename = "crypts-a-r-1")]
    CryptsAR1,
    #[serde(rename = "crypts-a-r-2")]
    CryptsAR2,
    #[serde(rename = "crypts-a-r-3")]
    CryptsAR3,
    #[serde(rename = "crypts-a-r-4")]
    CryptsAR4,
    #[serde(rename = "crypts-a-r-5")]
    CryptsAR5,
    #[serde(rename = "crypts-a-r-6")]
    CryptsAR6,
    #[serde(rename = "crypts-a-r-7")]
    CryptsAR7,
    #[serde(rename = "crypts-a-r-8")]
    CryptsAR8,
    #[serde(rename = "crypts-a-r-9")]
    CryptsAR9,
    #[serde(rename = "crypts-a-r-10")]
    CryptsAR10,
    #[serde(rename = "crypts-a-r-11")]
    CryptsAR11,
    #[serde(rename = "crypts-a-r-12")]
    CryptsAR12,
    #[serde(rename = "crypts-a-r-13")]
    CryptsAR13,
    #[serde(rename = "crypts-a-r-14")]
    CryptsAR14,
    #[serde(rename = "crypts-a-r-15")]
    CryptsAR15,
    #[serde(rename = "crypts-a-r-16")]
    CryptsAR16,
    #[serde(rename = "crypts-v-h-1")]
    CryptsVH1,
    #[serde(rename = "crypts-v-h-2")]
    CryptsVH2,
    #[serde(rename = "crypts-v-h-3")]
    CryptsVH3,
    #[serde(rename = "crypts-v-h-4")]
    CryptsVH4,
    #[serde(rename = "crypts-v-h-5")]
    CryptsVH5,
    #[serde(rename = "crypts-v-h-6")]
    CryptsVH6,
    #[serde(rename = "crypts-v-h-7")]
    CryptsVH7,
    #[serde(rename = "crypts-v-h-8")]
    CryptsVH8,
    #[serde(rename = "crypts-v-h-9")]
    CryptsVH9,
    #[serde(rename = "crypts-v-h-10")]
    CryptsVH10,
    #[serde(rename = "crypts-v-h-11")]
    CryptsVH11,
    #[serde(rename = "crypts-v-h-12")]
    CryptsVH12,
    #[serde(rename = "crypts-v-h-13")]
    CryptsVH13,
    #[serde(rename = "crypts-v-h-14")]
    CryptsVH14,
    #[serde(rename = "crypts-v-h-15")]
    CryptsVH15,
    #[serde(rename = "crypts-v-h-16")]
    CryptsVH16,
    #[serde(rename = "crypts-v-h-17")]
    CryptsVH17,
    #[serde(rename = "crypts-v-h-18")]
    CryptsVH18,
    #[serde(rename = "crypts-v-h-19")]
    CryptsVH19,
    #[serde(rename = "crypts-v-h-20")]
    CryptsVH20,
    #[serde(rename = "crypts-v-h-21")]
    CryptsVH21,
    #[serde(rename = "crypts-v-h-22")]
    CryptsVH22,
    #[serde(rename = "crypts-v-h-23")]
    CryptsVH23,
    #[serde(rename = "crypts-v-h-24")]
    CryptsVH24,
    #[serde(rename = "crypts-v-h-25")]
    CryptsVH25,
    #[serde(rename = "crypts-v-h-26")]
    CryptsVH26,
    #[serde(rename = "crypts-v-h-27")]
    CryptsVH27,
    #[serde(rename = "crypts-v-h-28")]
    CryptsVH28,
    #[serde(rename = "crypts-v-h-29")]
    CryptsVH29,
    #[serde(rename = "crypts-v-h-30")]
    CryptsVH30,
    #[serde(rename = "crypts-v-h-31")]
    CryptsVH31,
    #[serde(rename = "crypts-v-r-1")]
    CryptsVR1,
    #[serde(rename = "crypts-v-r-2")]
    CryptsVR2,
    #[serde(rename = "crypts-v-r-3")]
    CryptsVR3,
    #[serde(rename = "crypts-v-r-4")]
    CryptsVR4,
    #[serde(rename = "crypts-v-r-5")]
    CryptsVR5,
    #[serde(rename = "crypts-v-r-6")]
    CryptsVR6,
    #[serde(rename = "crypts-v-r-7")]
    CryptsVR7,
    #[serde(rename = "crypts-v-r-8")]
    CryptsVR8,
    #[serde(rename = "crypts-v-r-9")]
    CryptsVR9,
    #[serde(rename = "crypts-v-r-10")]
    CryptsVR10,
    #[serde(rename = "crypts-v-r-11")]
    CryptsVR11,
    #[serde(rename = "crypts-v-r-12")]
    CryptsVR12,
    #[serde(rename = "crypts-v-r-13")]
    CryptsVR13,
    #[serde(rename = "crypts-v-r-14")]
    CryptsVR14,
    #[serde(rename = "crypts-v-r-15")]
    CryptsVR15,
    #[serde(rename = "crypts-v-r-16")]
    CryptsVR16,
    #[serde(rename = "crypts-v-r-17")]
    CryptsVR17,
    #[serde(rename = "crypts-v-r-18")]
    CryptsVR18,
    #[serde(rename = "crypts-v-r-19")]
    CryptsVR19,
    #[serde(rename = "crypts-v-r-20")]
    CryptsVR20,
    #[serde(rename = "crypts-v-r-21")]
    CryptsVR21,
    #[serde(rename = "crypts-v-r-22")]
    CryptsVR22,
    #[serde(rename = "crypts-v-r-23")]
    CryptsVR23,
    #[serde(rename = "crypts-v-r-24")]
    CryptsVR24,
    #[serde(rename = "crypts-v-r-25")]
    CryptsVR25,
    #[serde(rename = "crypts-v-r-26")]
    CryptsVR26,
    #[serde(rename = "crypts-v-r-27")]
    CryptsVR27,
    #[serde(rename = "crypts-v-r-28")]
    CryptsVR28,
    #[serde(rename = "crypts-c-h-1")]
    CryptsCH1,
    #[serde(rename = "crypts-c-h-2")]
    CryptsCH2,
    #[serde(rename = "crypts-c-h-3")]
    CryptsCH3,
    #[serde(rename = "crypts-c-h-4")]
    CryptsCH4,
    #[serde(rename = "crypts-c-h-5")]
    CryptsCH5,
    #[serde(rename = "crypts-c-h-6")]
    CryptsCH6,
    #[serde(rename = "crypts-c-h-7")]
    CryptsCH7,
    #[serde(rename = "crypts-c-h-8")]
    CryptsCH8,
    #[serde(rename = "crypts-c-h-9")]
    CryptsCH9,
    #[serde(rename = "crypts-c-h-10")]
    CryptsCH10,
    #[serde(rename = "crypts-c-h-11")]
    CryptsCH11,
    #[serde(rename = "crypts-c-h-12")]
    CryptsCH12,
    #[serde(rename = "crypts-c-h-13")]
    CryptsCH13,
    #[serde(rename = "crypts-c-h-14")]
    CryptsCH14,
    #[serde(rename = "crypts-c-h-15")]
    CryptsCH15,
    #[serde(rename = "crypts-c-h-16")]
    CryptsCH16,
    #[serde(rename = "crypts-c-h-17")]
    CryptsCH17,
    #[serde(rename = "crypts-c-h-18")]
    CryptsCH18,
    #[serde(rename = "crypts-c-h-19")]
    CryptsCH19,
    #[serde(rename = "crypts-c-h-20")]
    CryptsCH20,
    #[serde(rename = "crypts-c-h-21")]
    CryptsCH21,
    #[serde(rename = "crypts-c-h-22")]
    CryptsCH22,
    #[serde(rename = "crypts-c-h-23")]
    CryptsCH23,
    #[serde(rename = "crypts-c-h-24")]
    CryptsCH24,
    #[serde(rename = "crypts-c-h-25")]
    CryptsCH25,
    #[serde(rename = "crypts-c-h-26")]
    CryptsCH26,
    #[serde(rename = "crypts-c-h-27")]
    CryptsCH27,
    #[serde(rename = "crypts-c-h-28")]
    CryptsCH28,
    #[serde(rename = "crypts-c-h-29")]
    CryptsCH29,
    #[serde(rename = "crypts-c-h-30")]
    CryptsCH30,
    #[serde(rename = "crypts-c-h-31")]
    CryptsCH31,
    #[serde(rename = "crypts-c-h-32")]
    CryptsCH32,
    #[serde(rename = "crypts-c-h-33")]
    CryptsCH33,
    #[serde(rename = "crypts-c-h-34")]
    CryptsCH34,
    #[serde(rename = "crypts-c-h-35")]
    CryptsCH35,
    #[serde(rename = "crypts-c-h-36")]
    CryptsCH36,
    #[serde(rename = "crypts-c-h-37")]
    CryptsCH37,
    #[serde(rename = "crypts-c-h-38")]
    CryptsCH38,
    #[serde(rename = "crypts-c-h-39")]
    CryptsCH39,
    #[serde(rename = "crypts-c-r-1")]
    CryptsCR1,
    #[serde(rename = "crypts-c-r-2")]
    CryptsCR2,
    #[serde(rename = "crypts-c-r-3")]
    CryptsCR3,
    #[serde(rename = "crypts-c-r-4")]
    CryptsCR4,
    #[serde(rename = "crypts-c-r-5")]
    CryptsCR5,
    #[serde(rename = "crypts-c-r-6")]
    CryptsCR6,
    #[serde(rename = "crypts-c-r-7")]
    CryptsCR7,
    #[serde(rename = "crypts-c-r-8")]
    CryptsCR8,
    #[serde(rename = "crypts-c-r-9")]
    CryptsCR9,
    #[serde(rename = "crypts-c-r-10")]
    CryptsCR10,
    #[serde(rename = "crypts-c-r-11")]
    CryptsCR11,
    #[serde(rename = "crypts-c-r-12")]
    CryptsCR12,
    #[serde(rename = "crypts-c-r-13")]
    CryptsCR13,
    #[serde(rename = "crypts-c-r-14")]
    CryptsCR14,
    #[serde(rename = "crypts-c-r-15")]
    CryptsCR15,
    #[serde(rename = "crypts-c-r-16")]
    CryptsCR16,
    #[serde(rename = "crypts-c-r-17")]
    CryptsCR17,
    #[serde(rename = "crypts-c-r-18")]
    CryptsCR18,
    #[serde(rename = "crypts-c-r-19")]
    CryptsCR19,
    #[serde(rename = "crypts-c-r-20")]
    CryptsCR20,
    #[serde(rename = "crypts-c-r-21")]
    CryptsCR21,
    #[serde(rename = "crypts-c-r-22")]
    CryptsCR22,
    #[serde(rename = "crypts-c-r-23")]
    CryptsCR23,
    #[serde(rename = "crypts-c-r-24")]
    CryptsCR24,
    #[serde(rename = "crypts-c-r-25")]
    CryptsCR25,
    #[serde(rename = "crypts-c-r-26")]
    CryptsCR26,
    #[serde(rename = "crypts-c-r-27")]
    CryptsCR27,
    #[serde(rename = "crypts-c-r-28")]
    CryptsCR28,
    #[serde(rename = "crypts-c-r-29")]
    CryptsCR29,
    #[serde(rename = "crypts-c-r-30")]
    CryptsCR30,
    #[serde(rename = "crypts-c-r-31")]
    CryptsCR31,
    #[serde(rename = "crypts-c-r-32")]
    CryptsCR32,
    #[serde(rename = "crypts-c-r-33")]
    CryptsCR33,
    #[serde(rename = "crypts-c-r-34")]
    CryptsCR34,
    #[serde(rename = "warrens-a-h-1")]
    WarrensAH1,
    #[serde(rename = "warrens-a-h-2")]
    WarrensAH2,
    #[serde(rename = "warrens-a-h-3")]
    WarrensAH3,
    #[serde(rename = "warrens-a-h-4")]
    WarrensAH4,
    #[serde(rename = "warrens-a-h-5")]
    WarrensAH5,
    #[serde(rename = "warrens-a-h-6")]
    WarrensAH6,
    #[serde(rename = "warrens-a-h-7")]
    WarrensAH7,
    #[serde(rename = "warrens-a-h-8")]
    WarrensAH8,
    #[serde(rename = "warrens-a-h-9")]
    WarrensAH9,
    #[serde(rename = "warrens-a-h-10")]
    WarrensAH10,
    #[serde(rename = "warrens-a-h-11")]
    WarrensAH11,
    #[serde(rename = "warrens-a-h-12")]
    WarrensAH12,
    #[serde(rename = "warrens-a-h-13")]
    WarrensAH13,
    #[serde(rename = "warrens-a-h-14")]
    WarrensAH14,
    #[serde(rename = "warrens-a-h-15")]
    WarrensAH15,
    #[serde(rename = "warrens-a-h-16")]
    WarrensAH16,
    #[serde(rename = "warrens-a-h-17")]
    WarrensAH17,
    #[serde(rename = "warrens-a-h-18")]
    WarrensAH18,
    #[serde(rename = "warrens-a-h-19")]
    WarrensAH19,
    #[serde(rename = "warrens-a-h-20")]
    WarrensAH20,
    #[serde(rename = "warrens-a-h-21")]
    WarrensAH21,
    #[serde(rename = "warrens-a-h-22")]
    WarrensAH22,
    #[serde(rename = "warrens-a-h-23")]
    WarrensAH23,
    #[serde(rename = "warrens-a-h-24")]
    WarrensAH24,
    #[serde(rename = "warrens-a-h-25")]
    WarrensAH25,
    #[serde(rename = "warrens-a-h-26")]
    WarrensAH26,
    #[serde(rename = "warrens-a-r-1")]
    WarrensAR1,
    #[serde(rename = "warrens-a-r-2")]
    WarrensAR2,
    #[serde(rename = "warrens-a-r-3")]
    WarrensAR3,
    #[serde(rename = "warrens-a-r-4")]
    WarrensAR4,
    #[serde(rename = "warrens-a-r-5")]
    WarrensAR5,
    #[serde(rename = "warrens-a-r-6")]
    WarrensAR6,
    #[serde(rename = "warrens-a-r-7")]
    WarrensAR7,
    #[serde(rename = "warrens-a-r-8")]
    WarrensAR8,
    #[serde(rename = "warrens-a-r-9")]
    WarrensAR9,
    #[serde(rename = "warrens-a-r-10")]
    WarrensAR10,
    #[serde(rename = "warrens-a-r-11")]
    WarrensAR11,
    #[serde(rename = "warrens-a-r-12")]
    WarrensAR12,
    #[serde(rename = "warrens-a-r-13")]
    WarrensAR13,
    #[serde(rename = "warrens-v-h-1")]
    WarrensVH1,
    #[serde(rename = "warrens-v-h-2")]
    WarrensVH2,
    #[serde(rename = "warrens-v-h-3")]
    WarrensVH3,
    #[serde(rename = "warrens-v-h-4")]
    WarrensVH4,
    #[serde(rename = "warrens-v-h-5")]
    WarrensVH5,
    #[serde(rename = "warrens-v-h-6")]
    WarrensVH6,
    #[serde(rename = "warrens-v-h-7")]
    WarrensVH7,
    #[serde(rename = "warrens-v-h-8")]
    WarrensVH8,
    #[serde(rename = "warrens-v-h-9")]
    WarrensVH9,
    #[serde(rename = "warrens-v-h-10")]
    WarrensVH10,
    #[serde(rename = "warrens-v-h-11")]
    WarrensVH11,
    #[serde(rename = "warrens-v-h-12")]
    WarrensVH12,
    #[serde(rename = "warrens-v-h-13")]
    WarrensVH13,
    #[serde(rename = "warrens-v-h-14")]
    WarrensVH14,
    #[serde(rename = "warrens-v-h-15")]
    WarrensVH15,
    #[serde(rename = "warrens-v-h-16")]
    WarrensVH16,
    #[serde(rename = "warrens-v-h-17")]
    WarrensVH17,
    #[serde(rename = "warrens-v-h-18")]
    WarrensVH18,
    #[serde(rename = "warrens-v-h-19")]
    WarrensVH19,
    #[serde(rename = "warrens-v-h-20")]
    WarrensVH20,
    #[serde(rename = "warrens-v-h-21")]
    WarrensVH21,
    #[serde(rename = "warrens-v-h-22")]
    WarrensVH22,
    #[serde(rename = "warrens-v-h-23")]
    WarrensVH23,
    #[serde(rename = "warrens-v-h-24")]
    WarrensVH24,
    #[serde(rename = "warrens-v-h-25")]
    WarrensVH25,
    #[serde(rename = "warrens-v-h-26")]
    WarrensVH26,
    #[serde(rename = "warrens-v-h-27")]
    WarrensVH27,
    #[serde(rename = "warrens-v-h-28")]
    WarrensVH28,
    #[serde(rename = "warrens-v-h-29")]
    WarrensVH29,
    #[serde(rename = "warrens-v-h-30")]
    WarrensVH30,
    #[serde(rename = "warrens-v-h-31")]
    WarrensVH31,
    #[serde(rename = "warrens-v-h-32")]
    WarrensVH32,
    #[serde(rename = "warrens-v-h-33")]
    WarrensVH33,
    #[serde(rename = "warrens-v-h-34")]
    WarrensVH34,
    #[serde(rename = "warrens-v-h-35")]
    WarrensVH35,
    #[serde(rename = "warrens-v-r-1")]
    WarrensVR1,
    #[serde(rename = "warrens-v-r-2")]
    WarrensVR2,
    #[serde(rename = "warrens-v-r-3")]
    WarrensVR3,
    #[serde(rename = "warrens-v-r-4")]
    WarrensVR4,
    #[serde(rename = "warrens-v-r-5")]
    WarrensVR5,
    #[serde(rename = "warrens-v-r-6")]
    WarrensVR6,
    #[serde(rename = "warrens-v-r-7")]
    WarrensVR7,
    #[serde(rename = "warrens-v-r-8")]
    WarrensVR8,
    #[serde(rename = "warrens-v-r-9")]
    WarrensVR9,
    #[serde(rename = "warrens-v-r-10")]
    WarrensVR10,
    #[serde(rename = "warrens-v-r-11")]
    WarrensVR11,
    #[serde(rename = "warrens-v-r-12")]
    WarrensVR12,
    #[serde(rename = "warrens-v-r-13")]
    WarrensVR13,
    #[serde(rename = "warrens-v-r-14")]
    WarrensVR14,
    #[serde(rename = "warrens-v-r-15")]
    WarrensVR15,
    #[serde(rename = "warrens-v-r-16")]
    WarrensVR16,
    #[serde(rename = "warrens-v-r-17")]
    WarrensVR17,
    #[serde(rename = "warrens-v-r-18")]
    WarrensVR18,
    #[serde(rename = "warrens-v-r-19")]
    WarrensVR19,
    #[serde(rename = "warrens-v-r-20")]
    WarrensVR20,
    #[serde(rename = "warrens-v-r-21")]
    WarrensVR21,
    #[serde(rename = "warrens-v-r-22")]
    WarrensVR22,
    #[serde(rename = "warrens-c-h-1")]
    WarrensCH1,
    #[serde(rename = "warrens-c-h-2")]
    WarrensCH2,
    #[serde(rename = "warrens-c-h-3")]
    WarrensCH3,
    #[serde(rename = "warrens-c-h-4")]
    WarrensCH4,
    #[serde(rename = "warrens-c-h-5")]
    WarrensCH5,
    #[serde(rename = "warrens-c-h-6")]
    WarrensCH6,
    #[serde(rename = "warrens-c-h-7")]
    WarrensCH7,
    #[serde(rename = "warrens-c-h-8")]
    WarrensCH8,
    #[serde(rename = "warrens-c-h-9")]
    WarrensCH9,
    #[serde(rename = "warrens-c-h-10")]
    WarrensCH10,
    #[serde(rename = "warrens-c-h-11")]
    WarrensCH11,
    #[serde(rename = "warrens-c-h-12")]
    WarrensCH12,
    #[serde(rename = "warrens-c-h-13")]
    WarrensCH13,
    #[serde(rename = "warrens-c-h-14")]
    WarrensCH14,
    #[serde(rename = "warrens-c-h-15")]
    WarrensCH15,
    #[serde(rename = "warrens-c-h-16")]
    WarrensCH16,
    #[serde(rename = "warrens-c-h-17")]
    WarrensCH17,
    #[serde(rename = "warrens-c-h-18")]
    WarrensCH18,
    #[serde(rename = "warrens-c-h-19")]
    WarrensCH19,
    #[serde(rename = "warrens-c-h-20")]
    WarrensCH20,
    #[serde(rename = "warrens-c-h-21")]
    WarrensCH21,
    #[serde(rename = "warrens-c-h-22")]
    WarrensCH22,
    #[serde(rename = "warrens-c-h-23")]
    WarrensCH23,
    #[serde(rename = "warrens-c-h-24")]
    WarrensCH24,
    #[serde(rename = "warrens-c-h-25")]
    WarrensCH25,
    #[serde(rename = "warrens-c-h-26")]
    WarrensCH26,
    #[serde(rename = "warrens-c-h-27")]
    WarrensCH27,
    #[serde(rename = "warrens-c-h-28")]
    WarrensCH28,
    #[serde(rename = "warrens-c-h-29")]
    WarrensCH29,
    #[serde(rename = "warrens-c-h-30")]
    WarrensCH30,
    #[serde(rename = "warrens-c-h-31")]
    WarrensCH31,
    #[serde(rename = "warrens-c-h-32")]
    WarrensCH32,
    #[serde(rename = "warrens-c-h-33")]
    WarrensCH33,
    #[serde(rename = "warrens-c-h-34")]
    WarrensCH34,
    #[serde(rename = "warrens-c-h-35")]
    WarrensCH35,
    #[serde(rename = "warrens-c-h-36")]
    WarrensCH36,
    #[serde(rename = "warrens-c-h-37")]
    WarrensCH37,
    #[serde(rename = "warrens-c-h-38")]
    WarrensCH38,
    #[serde(rename = "warrens-c-h-39")]
    WarrensCH39,
    #[serde(rename = "warrens-c-r-1")]
    WarrensCR1,
    #[serde(rename = "warrens-c-r-2")]
    WarrensCR2,
    #[serde(rename = "warrens-c-r-3")]
    WarrensCR3,
    #[serde(rename = "warrens-c-r-4")]
    WarrensCR4,
    #[serde(rename = "warrens-c-r-5")]
    WarrensCR5,
    #[serde(rename = "warrens-c-r-6")]
    WarrensCR6,
    #[serde(rename = "warrens-c-r-7")]
    WarrensCR7,
    #[serde(rename = "warrens-c-r-8")]
    WarrensCR8,
    #[serde(rename = "warrens-c-r-9")]
    WarrensCR9,
    #[serde(rename = "warrens-c-r-10")]
    WarrensCR10,
    #[serde(rename = "warrens-c-r-11")]
    WarrensCR11,
    #[serde(rename = "warrens-c-r-12")]
    WarrensCR12,
    #[serde(rename = "warrens-c-r-13")]
    WarrensCR13,
    #[serde(rename = "warrens-c-r-14")]
    WarrensCR14,
    #[serde(rename = "warrens-c-r-15")]
    WarrensCR15,
    #[serde(rename = "warrens-c-r-16")]
    WarrensCR16,
    #[serde(rename = "warrens-c-r-17")]
    WarrensCR17,
    #[serde(rename = "warrens-c-r-18")]
    WarrensCR18,
    #[serde(rename = "warrens-c-r-19")]
    WarrensCR19,
    #[serde(rename = "warrens-c-r-20")]
    WarrensCR20,
    #[serde(rename = "warrens-c-r-21")]
    WarrensCR21,
    #[serde(rename = "warrens-c-r-22")]
    WarrensCR22,
    #[serde(rename = "warrens-c-r-23")]
    WarrensCR23,
    #[serde(rename = "warrens-c-r-24")]
    WarrensCR24,
    #[serde(rename = "warrens-c-r-25")]
    WarrensCR25,
    #[serde(rename = "warrens-c-r-26")]
    WarrensCR26,
    #[serde(rename = "warrens-c-r-27")]
    WarrensCR27,
    #[serde(rename = "warrens-c-r-28")]
    WarrensCR28,
    #[serde(rename = "weald-a-h-1")]
    WealdAH1,
    #[serde(rename = "weald-a-h-2")]
    WealdAH2,
    #[serde(rename = "weald-a-h-3")]
    WealdAH3,
    #[serde(rename = "weald-a-h-4")]
    WealdAH4,
    #[serde(rename = "weald-a-h-5")]
    WealdAH5,
    #[serde(rename = "weald-a-h-6")]
    WealdAH6,
    #[serde(rename = "weald-a-h-7")]
    WealdAH7,
    #[serde(rename = "weald-a-h-8")]
    WealdAH8,
    #[serde(rename = "weald-a-h-9")]
    WealdAH9,
    #[serde(rename = "weald-a-h-10")]
    WealdAH10,
    #[serde(rename = "weald-a-h-11")]
    WealdAH11,
    #[serde(rename = "weald-a-h-12")]
    WealdAH12,
    #[serde(rename = "weald-a-h-13")]
    WealdAH13,
    #[serde(rename = "weald-a-h-14")]
    WealdAH14,
    #[serde(rename = "weald-a-h-15")]
    WealdAH15,
    #[serde(rename = "weald-a-h-16")]
    WealdAH16,
    #[serde(rename = "weald-a-h-17")]
    WealdAH17,
    #[serde(rename = "weald-a-h-18")]
    WealdAH18,
    #[serde(rename = "weald-a-h-19")]
    WealdAH19,
    #[serde(rename = "weald-a-h-20")]
    WealdAH20,
    #[serde(rename = "weald-a-h-21")]
    WealdAH21,
    #[serde(rename = "weald-a-r-1")]
    WealdAR1,
    #[serde(rename = "weald-a-r-2")]
    WealdAR2,
    #[serde(rename = "weald-a-r-3")]
    WealdAR3,
    #[serde(rename = "weald-a-r-4")]
    WealdAR4,
    #[serde(rename = "weald-a-r-5")]
    WealdAR5,
    #[serde(rename = "weald-a-r-6")]
    WealdAR6,
    #[serde(rename = "weald-a-r-7")]
    WealdAR7,
    #[serde(rename = "weald-a-r-8")]
    WealdAR8,
    #[serde(rename = "weald-a-r-9")]
    WealdAR9,
    #[serde(rename = "weald-a-r-10")]
    WealdAR10,
    #[serde(rename = "weald-a-r-11")]
    WealdAR11,
    #[serde(rename = "weald-a-r-12")]
    WealdAR12,
    #[serde(rename = "weald-a-r-13")]
    WealdAR13,
    #[serde(rename = "weald-a-r-14")]
    WealdAR14,
    #[serde(rename = "weald-a-r-15")]
    WealdAR15,
    #[serde(rename = "weald-a-r-16")]
    WealdAR16,
    #[serde(rename = "weald-v-h-1")]
    WealdVH1,
    #[serde(rename = "weald-v-h-2")]
    WealdVH2,
    #[serde(rename = "weald-v-h-3")]
    WealdVH3,
    #[serde(rename = "weald-v-h-4")]
    WealdVH4,
    #[serde(rename = "weald-v-h-5")]
    WealdVH5,
    #[serde(rename = "weald-v-h-6")]
    WealdVH6,
    #[serde(rename = "weald-v-h-7")]
    WealdVH7,
    #[serde(rename = "weald-v-h-8")]
    WealdVH8,
    #[serde(rename = "weald-v-h-9")]
    WealdVH9,
    #[serde(rename = "weald-v-h-10")]
    WealdVH10,
    #[serde(rename = "weald-v-h-11")]
    WealdVH11,
    #[serde(rename = "weald-v-h-12")]
    WealdVH12,
    #[serde(rename = "weald-v-h-13")]
    WealdVH13,
    #[serde(rename = "weald-v-h-14")]
    WealdVH14,
    #[serde(rename = "weald-v-h-15")]
    WealdVH15,
    #[serde(rename = "weald-v-h-16")]
    WealdVH16,
    #[serde(rename = "weald-v-h-17")]
    WealdVH17,
    #[serde(rename = "weald-v-h-18")]
    WealdVH18,
    #[serde(rename = "weald-v-h-19")]
    WealdVH19,
    #[serde(rename = "weald-v-h-20")]
    WealdVH20,
    #[serde(rename = "weald-v-h-21")]
    WealdVH21,
    #[serde(rename = "weald-v-h-22")]
    WealdVH22,
    #[serde(rename = "weald-v-h-23")]
    WealdVH23,
    #[serde(rename = "weald-v-h-24")]
    WealdVH24,
    #[serde(rename = "weald-v-h-25")]
    WealdVH25,
    #[serde(rename = "weald-v-h-26")]
    WealdVH26,
    #[serde(rename = "weald-v-h-27")]
    WealdVH27,
    #[serde(rename = "weald-v-h-28")]
    WealdVH28,
    #[serde(rename = "weald-v-h-29")]
    WealdVH29,
    #[serde(rename = "weald-v-h-30")]
    WealdVH30,
    #[serde(rename = "weald-v-h-31")]
    WealdVH31,
    #[serde(rename = "weald-v-h-32")]
    WealdVH32,
    #[serde(rename = "weald-v-h-33")]
    WealdVH33,
    #[serde(rename = "weald-v-h-34")]
    WealdVH34,
    #[serde(rename = "weald-v-h-35")]
    WealdVH35,
    #[serde(rename = "weald-v-h-36")]
    WealdVH36,
    #[serde(rename = "weald-v-r-1")]
    WealdVR1,
    #[serde(rename = "weald-v-r-2")]
    WealdVR2,
    #[serde(rename = "weald-v-r-3")]
    WealdVR3,
    #[serde(rename = "weald-v-r-4")]
    WealdVR4,
    #[serde(rename = "weald-v-r-5")]
    WealdVR5,
    #[serde(rename = "weald-v-r-6")]
    WealdVR6,
    #[serde(rename = "weald-v-r-7")]
    WealdVR7,
    #[serde(rename = "weald-v-r-8")]
    WealdVR8,
    #[serde(rename = "weald-v-r-9")]
    WealdVR9,
    #[serde(rename = "weald-v-r-10")]
    WealdVR10,
    #[serde(rename = "weald-v-r-11")]
    WealdVR11,
    #[serde(rename = "weald-v-r-12")]
    WealdVR12,
    #[serde(rename = "weald-v-r-13")]
    WealdVR13,
    #[serde(rename = "weald-v-r-14")]
    WealdVR14,
    #[serde(rename = "weald-v-r-15")]
    WealdVR15,
    #[serde(rename = "weald-v-r-16")]
    WealdVR16,
    #[serde(rename = "weald-v-r-17")]
    WealdVR17,
    #[serde(rename = "weald-v-r-18")]
    WealdVR18,
    #[serde(rename = "weald-v-r-19")]
    WealdVR19,
    #[serde(rename = "weald-v-r-20")]
    WealdVR20,
    #[serde(rename = "weald-v-r-21")]
    WealdVR21,
    #[serde(rename = "weald-v-r-22")]
    WealdVR22,
    #[serde(rename = "weald-v-r-23")]
    WealdVR23,
    #[serde(rename = "weald-c-h-1")]
    WealdCH1,
    #[serde(rename = "weald-c-h-2")]
    WealdCH2,
    #[serde(rename = "weald-c-h-3")]
    WealdCH3,
    #[serde(rename = "weald-c-h-4")]
    WealdCH4,
    #[serde(rename = "weald-c-h-5")]
    WealdCH5,
    #[serde(rename = "weald-c-h-6")]
    WealdCH6,
    #[serde(rename = "weald-c-h-7")]
    WealdCH7,
    #[serde(rename = "weald-c-h-8")]
    WealdCH8,
    #[serde(rename = "weald-c-h-9")]
    WealdCH9,
    #[serde(rename = "weald-c-h-10")]
    WealdCH10,
    #[serde(rename = "weald-c-h-11")]
    WealdCH11,
    #[serde(rename = "weald-c-h-12")]
    WealdCH12,
    #[serde(rename = "weald-c-h-13")]
    WealdCH13,
    #[serde(rename = "weald-c-h-14")]
    WealdCH14,
    #[serde(rename = "weald-c-h-15")]
    WealdCH15,
    #[serde(rename = "weald-c-h-16")]
    WealdCH16,
    #[serde(rename = "weald-c-h-17")]
    WealdCH17,
    #[serde(rename = "weald-c-h-18")]
    WealdCH18,
    #[serde(rename = "weald-c-h-19")]
    WealdCH19,
    #[serde(rename = "weald-c-h-20")]
    WealdCH20,
    #[serde(rename = "weald-c-h-21")]
    WealdCH21,
    #[serde(rename = "weald-c-h-22")]
    WealdCH22,
    #[serde(rename = "weald-c-h-23")]
    WealdCH23,
    #[serde(rename = "weald-c-h-24")]
    WealdCH24,
    #[serde(rename = "weald-c-h-25")]
    WealdCH25,
    #[serde(rename = "weald-c-h-26")]
    WealdCH26,
    #[serde(rename = "weald-c-h-27")]
    WealdCH27,
    #[serde(rename = "weald-c-h-28")]
    WealdCH28,
    #[serde(rename = "weald-c-h-29")]
    WealdCH29,
    #[serde(rename = "weald-c-h-30")]
    WealdCH30,
    #[serde(rename = "weald-c-h-31")]
    WealdCH31,
    #[serde(rename = "weald-c-h-32")]
    WealdCH32,
    #[serde(rename = "weald-c-h-33")]
    WealdCH33,
    #[serde(rename = "weald-c-h-34")]
    WealdCH34,
    #[serde(rename = "weald-c-h-35")]
    WealdCH35,
    #[serde(rename = "weald-c-h-36")]
    WealdCH36,
    #[serde(rename = "weald-c-r-1")]
    WealdCR1,
    #[serde(rename = "weald-c-r-2")]
    WealdCR2,
    #[serde(rename = "weald-c-r-3")]
    WealdCR3,
    #[serde(rename = "weald-c-r-4")]
    WealdCR4,
    #[serde(rename = "weald-c-r-5")]
    WealdCR5,
    #[serde(rename = "weald-c-r-6")]
    WealdCR6,
    #[serde(rename = "weald-c-r-7")]
    WealdCR7,
    #[serde(rename = "weald-c-r-8")]
    WealdCR8,
    #[serde(rename = "weald-c-r-9")]
    WealdCR9,
    #[serde(rename = "weald-c-r-10")]
    WealdCR10,
    #[serde(rename = "weald-c-r-11")]
    WealdCR11,
    #[serde(rename = "weald-c-r-12")]
    WealdCR12,
    #[serde(rename = "weald-c-r-13")]
    WealdCR13,
    #[serde(rename = "weald-c-r-14")]
    WealdCR14,
    #[serde(rename = "weald-c-r-15")]
    WealdCR15,
    #[serde(rename = "weald-c-r-16")]
    WealdCR16,
    #[serde(rename = "weald-c-r-17")]
    WealdCR17,
    #[serde(rename = "weald-c-r-18")]
    WealdCR18,
    #[serde(rename = "weald-c-r-19")]
    WealdCR19,
    #[serde(rename = "weald-c-r-20")]
    WealdCR20,
    #[serde(rename = "weald-c-r-21")]
    WealdCR21,
    #[serde(rename = "weald-c-r-22")]
    WealdCR22,
    #[serde(rename = "weald-c-r-23")]
    WealdCR23,
    #[serde(rename = "weald-c-r-24")]
    WealdCR24,
    #[serde(rename = "weald-c-r-25")]
    WealdCR25,
    #[serde(rename = "weald-c-r-26")]
    WealdCR26,
    #[serde(rename = "weald-c-r-27")]
    WealdCR27,
    #[serde(rename = "weald-c-r-28")]
    WealdCR28,
    #[serde(rename = "weald-c-r-29")]
    WealdCR29,
    #[serde(rename = "darkest-d-h-1")]
    DarkestDH1,
    #[serde(rename = "darkest-d-h-2")]
    DarkestDH2,
    #[serde(rename = "darkest-d-h-3")]
    DarkestDH3,
    #[serde(rename = "darkest-d-h-4")]
    DarkestDH4,
    #[serde(rename = "darkest-d-h-5")]
    DarkestDH5,
    #[serde(rename = "darkest-d-h-6")]
    DarkestDH6,
    #[serde(rename = "darkest-d-h-7")]
    DarkestDH7,
    #[serde(rename = "darkest-d-h-8")]
    DarkestDH8,
    #[serde(rename = "darkest-d-h-9")]
    DarkestDH9,
    #[serde(rename = "darkest-d-h-10")]
    DarkestDH10,
    #[serde(rename = "darkest-d-h-11")]
    DarkestDH11,
    #[serde(rename = "darkest-d-h-12")]
    DarkestDH12,
    #[serde(rename = "darkest-d-h-13")]
    DarkestDH13,
    #[serde(rename = "darkest-d-h-14")]
    DarkestDH14,
    #[serde(rename = "darkest-d-h-15")]
    DarkestDH15,
    #[serde(rename = "darkest-d-h-16")]
    DarkestDH16,
    #[serde(rename = "darkest-d-h-17")]
    DarkestDH17,
    #[serde(rename = "darkest-d-h-18")]
    DarkestDH18,
    #[serde(rename = "darkest-d-h-19")]
    DarkestDH19,
    #[serde(rename = "darkest-d-h-20")]
    DarkestDH20,
    #[serde(rename = "darkest-d-h-21")]
    DarkestDH21,
    #[serde(rename = "darkest-d-h-22")]
    DarkestDH22,
    #[serde(rename = "darkest-d-h-23")]
    DarkestDH23,
    #[serde(rename = "darkest-d-h-24")]
    DarkestDH24,
    #[serde(rename = "darkest-d-r-1")]
    DarkestDR1,
    #[serde(rename = "darkest-d-r-2")]
    DarkestDR2,
    #[serde(rename = "darkest-d-r-3")]
    DarkestDR3,
    #[serde(rename = "darkest-d-r-4")]
    DarkestDR4,
    #[serde(rename = "darkest-d-r-5")]
    DarkestDR5,
    #[serde(rename = "darkest-d-r-6")]
    DarkestDR6,
    #[serde(rename = "darkest-d-r-7")]
    DarkestDR7,
    #[serde(rename = "darkest-d-r-8")]
    DarkestDR8,
    #[serde(rename = "darkest-d-r-9")]
    DarkestDR9,
    #[serde(rename = "darkest-d-r-10")]
    DarkestDR10,
    #[serde(rename = "darkest-d-r-11")]
    DarkestDR11,
    #[serde(rename = "darkest-d-r-12")]
    DarkestDR12,
    #[serde(rename = "darkest-d-r-13")]
    DarkestDR13,
    #[serde(rename = "darkest-d-r-14")]
    DarkestDR14,
    #[serde(rename = "darkest-d-r-15")]
    DarkestDR15,
    #[serde(rename = "darkest-d-r-16")]
    DarkestDR16,
    #[serde(rename = "darkest-d-r-17")]
    DarkestDR17,
    #[serde(rename = "darkest-d-r-18")]
    DarkestDR18,
    #[serde(rename = "darkest-d-r-19")]
    DarkestDR19,
    #[serde(rename = "darkest-d-r-20")]
    DarkestDR20,
    #[serde(rename = "darkest-d-r-21")]
    DarkestDR21,
    #[serde(rename = "darkest-d-r-22")]
    DarkestDR22,
    #[serde(rename = "darkest-d-r-23")]
    DarkestDR23,
}

impl EncounterId {
    pub const fn value(&self) -> &'static str {
        match self {
            Self::TutorialAH1 => "tutorial-a-h-1",
            Self::TutorialAR1 => "tutorial-a-r-1",
            Self::CoveAH1 => "cove-a-h-1",
            Self::CoveAH2 => "cove-a-h-2",
            Self::CoveAH3 => "cove-a-h-3",
            Self::CoveAH4 => "cove-a-h-4",
            Self::CoveAH5 => "cove-a-h-5",
            Self::CoveAH6 => "cove-a-h-6",
            Self::CoveAH7 => "cove-a-h-7",
            Self::CoveAH8 => "cove-a-h-8",
            Self::CoveAH9 => "cove-a-h-9",
            Self::CoveAH10 => "cove-a-h-10",
            Self::CoveAH11 => "cove-a-h-11",
            Self::CoveAH12 => "cove-a-h-12",
            Self::CoveAH13 => "cove-a-h-13",
            Self::CoveAH14 => "cove-a-h-14",
            Self::CoveAH15 => "cove-a-h-15",
            Self::CoveAH16 => "cove-a-h-16",
            Self::CoveAH17 => "cove-a-h-17",
            Self::CoveAH18 => "cove-a-h-18",
            Self::CoveAH19 => "cove-a-h-19",
            Self::CoveAH20 => "cove-a-h-20",
            Self::CoveAH21 => "cove-a-h-21",
            Self::CoveAH22 => "cove-a-h-22",
            Self::CoveAR1 => "cove-a-r-1",
            Self::CoveAR2 => "cove-a-r-2",
            Self::CoveAR3 => "cove-a-r-3",
            Self::CoveAR4 => "cove-a-r-4",
            Self::CoveAR5 => "cove-a-r-5",
            Self::CoveAR6 => "cove-a-r-6",
            Self::CoveAR7 => "cove-a-r-7",
            Self::CoveAR8 => "cove-a-r-8",
            Self::CoveAR9 => "cove-a-r-9",
            Self::CoveAR10 => "cove-a-r-10",
            Self::CoveAR11 => "cove-a-r-11",
            Self::CoveAR12 => "cove-a-r-12",
            Self::CoveAR13 => "cove-a-r-13",
            Self::CoveAR14 => "cove-a-r-14",
            Self::CoveAR15 => "cove-a-r-15",
            Self::CoveAR16 => "cove-a-r-16",
            Self::CoveAR17 => "cove-a-r-17",
            Self::CoveAR18 => "cove-a-r-18",
            Self::CoveAR19 => "cove-a-r-19",
            Self::CoveAR20 => "cove-a-r-20",
            Self::CoveVH1 => "cove-v-h-1",
            Self::CoveVH2 => "cove-v-h-2",
            Self::CoveVH3 => "cove-v-h-3",
            Self::CoveVH4 => "cove-v-h-4",
            Self::CoveVH5 => "cove-v-h-5",
            Self::CoveVH6 => "cove-v-h-6",
            Self::CoveVH7 => "cove-v-h-7",
            Self::CoveVH8 => "cove-v-h-8",
            Self::CoveVH9 => "cove-v-h-9",
            Self::CoveVH10 => "cove-v-h-10",
            Self::CoveVH11 => "cove-v-h-11",
            Self::CoveVH12 => "cove-v-h-12",
            Self::CoveVH13 => "cove-v-h-13",
            Self::CoveVH14 => "cove-v-h-14",
            Self::CoveVH15 => "cove-v-h-15",
            Self::CoveVH16 => "cove-v-h-16",
            Self::CoveVH17 => "cove-v-h-17",
            Self::CoveVH18 => "cove-v-h-18",
            Self::CoveVH19 => "cove-v-h-19",
            Self::CoveVH20 => "cove-v-h-20",
            Self::CoveVH21 => "cove-v-h-21",
            Self::CoveVH22 => "cove-v-h-22",
            Self::CoveVH23 => "cove-v-h-23",
            Self::CoveVH24 => "cove-v-h-24",
            Self::CoveVH25 => "cove-v-h-25",
            Self::CoveVH26 => "cove-v-h-26",
            Self::CoveVH27 => "cove-v-h-27",
            Self::CoveVH28 => "cove-v-h-28",
            Self::CoveVH29 => "cove-v-h-29",
            Self::CoveVR1 => "cove-v-r-1",
            Self::CoveVR2 => "cove-v-r-2",
            Self::CoveVR3 => "cove-v-r-3",
            Self::CoveVR4 => "cove-v-r-4",
            Self::CoveVR5 => "cove-v-r-5",
            Self::CoveVR6 => "cove-v-r-6",
            Self::CoveVR7 => "cove-v-r-7",
            Self::CoveVR8 => "cove-v-r-8",
            Self::CoveVR9 => "cove-v-r-9",
            Self::CoveVR10 => "cove-v-r-10",
            Self::CoveVR11 => "cove-v-r-11",
            Self::CoveVR12 => "cove-v-r-12",
            Self::CoveVR13 => "cove-v-r-13",
            Self::CoveVR14 => "cove-v-r-14",
            Self::CoveVR15 => "cove-v-r-15",
            Self::CoveVR16 => "cove-v-r-16",
            Self::CoveVR17 => "cove-v-r-17",
            Self::CoveVR18 => "cove-v-r-18",
            Self::CoveVR19 => "cove-v-r-19",
            Self::CoveVR20 => "cove-v-r-20",
            Self::CoveVR21 => "cove-v-r-21",
            Self::CoveVR22 => "cove-v-r-22",
            Self::CoveVR23 => "cove-v-r-23",
            Self::CoveVR24 => "cove-v-r-24",
            Self::CoveCH1 => "cove-c-h-1",
            Self::CoveCH2 => "cove-c-h-2",
            Self::CoveCH3 => "cove-c-h-3",
            Self::CoveCH4 => "cove-c-h-4",
            Self::CoveCH5 => "cove-c-h-5",
            Self::CoveCH6 => "cove-c-h-6",
            Self::CoveCH7 => "cove-c-h-7",
            Self::CoveCH8 => "cove-c-h-8",
            Self::CoveCH9 => "cove-c-h-9",
            Self::CoveCH10 => "cove-c-h-10",
            Self::CoveCH11 => "cove-c-h-11",
            Self::CoveCH12 => "cove-c-h-12",
            Self::CoveCH13 => "cove-c-h-13",
            Self::CoveCH14 => "cove-c-h-14",
            Self::CoveCH15 => "cove-c-h-15",
            Self::CoveCH16 => "cove-c-h-16",
            Self::CoveCH17 => "cove-c-h-17",
            Self::CoveCH18 => "cove-c-h-18",
            Self::CoveCH19 => "cove-c-h-19",
            Self::CoveCH20 => "cove-c-h-20",
            Self::CoveCH21 => "cove-c-h-21",
            Self::CoveCH22 => "cove-c-h-22",
            Self::CoveCH23 => "cove-c-h-23",
            Self::CoveCH24 => "cove-c-h-24",
            Self::CoveCH25 => "cove-c-h-25",
            Self::CoveCH26 => "cove-c-h-26",
            Self::CoveCH27 => "cove-c-h-27",
            Self::CoveCH28 => "cove-c-h-28",
            Self::CoveCH29 => "cove-c-h-29",
            Self::CoveCH30 => "cove-c-h-30",
            Self::CoveCH31 => "cove-c-h-31",
            Self::CoveCH32 => "cove-c-h-32",
            Self::CoveCH33 => "cove-c-h-33",
            Self::CoveCH34 => "cove-c-h-34",
            Self::CoveCR1 => "cove-c-r-1",
            Self::CoveCR2 => "cove-c-r-2",
            Self::CoveCR3 => "cove-c-r-3",
            Self::CoveCR4 => "cove-c-r-4",
            Self::CoveCR5 => "cove-c-r-5",
            Self::CoveCR6 => "cove-c-r-6",
            Self::CoveCR7 => "cove-c-r-7",
            Self::CoveCR8 => "cove-c-r-8",
            Self::CoveCR9 => "cove-c-r-9",
            Self::CoveCR10 => "cove-c-r-10",
            Self::CoveCR11 => "cove-c-r-11",
            Self::CoveCR12 => "cove-c-r-12",
            Self::CoveCR13 => "cove-c-r-13",
            Self::CoveCR14 => "cove-c-r-14",
            Self::CoveCR15 => "cove-c-r-15",
            Self::CoveCR16 => "cove-c-r-16",
            Self::CoveCR17 => "cove-c-r-17",
            Self::CoveCR18 => "cove-c-r-18",
            Self::CoveCR19 => "cove-c-r-19",
            Self::CoveCR20 => "cove-c-r-20",
            Self::CoveCR21 => "cove-c-r-21",
            Self::CoveCR22 => "cove-c-r-22",
            Self::CoveCR23 => "cove-c-r-23",
            Self::CoveCR24 => "cove-c-r-24",
            Self::CoveCR25 => "cove-c-r-25",
            Self::CoveCR26 => "cove-c-r-26",
            Self::CoveCR27 => "cove-c-r-27",
            Self::CoveCR28 => "cove-c-r-28",
            Self::CoveCR29 => "cove-c-r-29",
            Self::CoveCR30 => "cove-c-r-30",
            Self::CoveCR31 => "cove-c-r-31",
            Self::CryptsAH1 => "crypts-a-h-1",
            Self::CryptsAH2 => "crypts-a-h-2",
            Self::CryptsAH3 => "crypts-a-h-3",
            Self::CryptsAH4 => "crypts-a-h-4",
            Self::CryptsAH5 => "crypts-a-h-5",
            Self::CryptsAH6 => "crypts-a-h-6",
            Self::CryptsAH7 => "crypts-a-h-7",
            Self::CryptsAH8 => "crypts-a-h-8",
            Self::CryptsAH9 => "crypts-a-h-9",
            Self::CryptsAH10 => "crypts-a-h-10",
            Self::CryptsAH11 => "crypts-a-h-11",
            Self::CryptsAH12 => "crypts-a-h-12",
            Self::CryptsAH13 => "crypts-a-h-13",
            Self::CryptsAH14 => "crypts-a-h-14",
            Self::CryptsAH15 => "crypts-a-h-15",
            Self::CryptsAH16 => "crypts-a-h-16",
            Self::CryptsAH17 => "crypts-a-h-17",
            Self::CryptsAH18 => "crypts-a-h-18",
            Self::CryptsAH19 => "crypts-a-h-19",
            Self::CryptsAH20 => "crypts-a-h-20",
            Self::CryptsAH21 => "crypts-a-h-21",
            Self::CryptsAH22 => "crypts-a-h-22",
            Self::CryptsAR1 => "crypts-a-r-1",
            Self::CryptsAR2 => "crypts-a-r-2",
            Self::CryptsAR3 => "crypts-a-r-3",
            Self::CryptsAR4 => "crypts-a-r-4",
            Self::CryptsAR5 => "crypts-a-r-5",
            Self::CryptsAR6 => "crypts-a-r-6",
            Self::CryptsAR7 => "crypts-a-r-7",
            Self::CryptsAR8 => "crypts-a-r-8",
            Self::CryptsAR9 => "crypts-a-r-9",
            Self::CryptsAR10 => "crypts-a-r-10",
            Self::CryptsAR11 => "crypts-a-r-11",
            Self::CryptsAR12 => "crypts-a-r-12",
            Self::CryptsAR13 => "crypts-a-r-13",
            Self::CryptsAR14 => "crypts-a-r-14",
            Self::CryptsAR15 => "crypts-a-r-15",
            Self::CryptsAR16 => "crypts-a-r-16",
            Self::CryptsVH1 => "crypts-v-h-1",
            Self::CryptsVH2 => "crypts-v-h-2",
            Self::CryptsVH3 => "crypts-v-h-3",
            Self::CryptsVH4 => "crypts-v-h-4",
            Self::CryptsVH5 => "crypts-v-h-5",
            Self::CryptsVH6 => "crypts-v-h-6",
            Self::CryptsVH7 => "crypts-v-h-7",
            Self::CryptsVH8 => "crypts-v-h-8",
            Self::CryptsVH9 => "crypts-v-h-9",
            Self::CryptsVH10 => "crypts-v-h-10",
            Self::CryptsVH11 => "crypts-v-h-11",
            Self::CryptsVH12 => "crypts-v-h-12",
            Self::CryptsVH13 => "crypts-v-h-13",
            Self::CryptsVH14 => "crypts-v-h-14",
            Self::CryptsVH15 => "crypts-v-h-15",
            Self::CryptsVH16 => "crypts-v-h-16",
            Self::CryptsVH17 => "crypts-v-h-17",
            Self::CryptsVH18 => "crypts-v-h-18",
            Self::CryptsVH19 => "crypts-v-h-19",
            Self::CryptsVH20 => "crypts-v-h-20",
            Self::CryptsVH21 => "crypts-v-h-21",
            Self::CryptsVH22 => "crypts-v-h-22",
            Self::CryptsVH23 => "crypts-v-h-23",
            Self::CryptsVH24 => "crypts-v-h-24",
            Self::CryptsVH25 => "crypts-v-h-25",
            Self::CryptsVH26 => "crypts-v-h-26",
            Self::CryptsVH27 => "crypts-v-h-27",
            Self::CryptsVH28 => "crypts-v-h-28",
            Self::CryptsVH29 => "crypts-v-h-29",
            Self::CryptsVH30 => "crypts-v-h-30",
            Self::CryptsVH31 => "crypts-v-h-31",
            Self::CryptsVR1 => "crypts-v-r-1",
            Self::CryptsVR2 => "crypts-v-r-2",
            Self::CryptsVR3 => "crypts-v-r-3",
            Self::CryptsVR4 => "crypts-v-r-4",
            Self::CryptsVR5 => "crypts-v-r-5",
            Self::CryptsVR6 => "crypts-v-r-6",
            Self::CryptsVR7 => "crypts-v-r-7",
            Self::CryptsVR8 => "crypts-v-r-8",
            Self::CryptsVR9 => "crypts-v-r-9",
            Self::CryptsVR10 => "crypts-v-r-10",
            Self::CryptsVR11 => "crypts-v-r-11",
            Self::CryptsVR12 => "crypts-v-r-12",
            Self::CryptsVR13 => "crypts-v-r-13",
            Self::CryptsVR14 => "crypts-v-r-14",
            Self::CryptsVR15 => "crypts-v-r-15",
            Self::CryptsVR16 => "crypts-v-r-16",
            Self::CryptsVR17 => "crypts-v-r-17",
            Self::CryptsVR18 => "crypts-v-r-18",
            Self::CryptsVR19 => "crypts-v-r-19",
            Self::CryptsVR20 => "crypts-v-r-20",
            Self::CryptsVR21 => "crypts-v-r-21",
            Self::CryptsVR22 => "crypts-v-r-22",
            Self::CryptsVR23 => "crypts-v-r-23",
            Self::CryptsVR24 => "crypts-v-r-24",
            Self::CryptsVR25 => "crypts-v-r-25",
            Self::CryptsVR26 => "crypts-v-r-26",
            Self::CryptsVR27 => "crypts-v-r-27",
            Self::CryptsVR28 => "crypts-v-r-28",
            Self::CryptsCH1 => "crypts-c-h-1",
            Self::CryptsCH2 => "crypts-c-h-2",
            Self::CryptsCH3 => "crypts-c-h-3",
            Self::CryptsCH4 => "crypts-c-h-4",
            Self::CryptsCH5 => "crypts-c-h-5",
            Self::CryptsCH6 => "crypts-c-h-6",
            Self::CryptsCH7 => "crypts-c-h-7",
            Self::CryptsCH8 => "crypts-c-h-8",
            Self::CryptsCH9 => "crypts-c-h-9",
            Self::CryptsCH10 => "crypts-c-h-10",
            Self::CryptsCH11 => "crypts-c-h-11",
            Self::CryptsCH12 => "crypts-c-h-12",
            Self::CryptsCH13 => "crypts-c-h-13",
            Self::CryptsCH14 => "crypts-c-h-14",
            Self::CryptsCH15 => "crypts-c-h-15",
            Self::CryptsCH16 => "crypts-c-h-16",
            Self::CryptsCH17 => "crypts-c-h-17",
            Self::CryptsCH18 => "crypts-c-h-18",
            Self::CryptsCH19 => "crypts-c-h-19",
            Self::CryptsCH20 => "crypts-c-h-20",
            Self::CryptsCH21 => "crypts-c-h-21",
            Self::CryptsCH22 => "crypts-c-h-22",
            Self::CryptsCH23 => "crypts-c-h-23",
            Self::CryptsCH24 => "crypts-c-h-24",
            Self::CryptsCH25 => "crypts-c-h-25",
            Self::CryptsCH26 => "crypts-c-h-26",
            Self::CryptsCH27 => "crypts-c-h-27",
            Self::CryptsCH28 => "crypts-c-h-28",
            Self::CryptsCH29 => "crypts-c-h-29",
            Self::CryptsCH30 => "crypts-c-h-30",
            Self::CryptsCH31 => "crypts-c-h-31",
            Self::CryptsCH32 => "crypts-c-h-32",
            Self::CryptsCH33 => "crypts-c-h-33",
            Self::CryptsCH34 => "crypts-c-h-34",
            Self::CryptsCH35 => "crypts-c-h-35",
            Self::CryptsCH36 => "crypts-c-h-36",
            Self::CryptsCH37 => "crypts-c-h-37",
            Self::CryptsCH38 => "crypts-c-h-38",
            Self::CryptsCH39 => "crypts-c-h-39",
            Self::CryptsCR1 => "crypts-c-r-1",
            Self::CryptsCR2 => "crypts-c-r-2",
            Self::CryptsCR3 => "crypts-c-r-3",
            Self::CryptsCR4 => "crypts-c-r-4",
            Self::CryptsCR5 => "crypts-c-r-5",
            Self::CryptsCR6 => "crypts-c-r-6",
            Self::CryptsCR7 => "crypts-c-r-7",
            Self::CryptsCR8 => "crypts-c-r-8",
            Self::CryptsCR9 => "crypts-c-r-9",
            Self::CryptsCR10 => "crypts-c-r-10",
            Self::CryptsCR11 => "crypts-c-r-11",
            Self::CryptsCR12 => "crypts-c-r-12",
            Self::CryptsCR13 => "crypts-c-r-13",
            Self::CryptsCR14 => "crypts-c-r-14",
            Self::CryptsCR15 => "crypts-c-r-15",
            Self::CryptsCR16 => "crypts-c-r-16",
            Self::CryptsCR17 => "crypts-c-r-17",
            Self::CryptsCR18 => "crypts-c-r-18",
            Self::CryptsCR19 => "crypts-c-r-19",
            Self::CryptsCR20 => "crypts-c-r-20",
            Self::CryptsCR21 => "crypts-c-r-21",
            Self::CryptsCR22 => "crypts-c-r-22",
            Self::CryptsCR23 => "crypts-c-r-23",
            Self::CryptsCR24 => "crypts-c-r-24",
            Self::CryptsCR25 => "crypts-c-r-25",
            Self::CryptsCR26 => "crypts-c-r-26",
            Self::CryptsCR27 => "crypts-c-r-27",
            Self::CryptsCR28 => "crypts-c-r-28",
            Self::CryptsCR29 => "crypts-c-r-29",
            Self::CryptsCR30 => "crypts-c-r-30",
            Self::CryptsCR31 => "crypts-c-r-31",
            Self::CryptsCR32 => "crypts-c-r-32",
            Self::CryptsCR33 => "crypts-c-r-33",
            Self::CryptsCR34 => "crypts-c-r-34",
            Self::WarrensAH1 => "warrens-a-h-1",
            Self::WarrensAH2 => "warrens-a-h-2",
            Self::WarrensAH3 => "warrens-a-h-3",
            Self::WarrensAH4 => "warrens-a-h-4",
            Self::WarrensAH5 => "warrens-a-h-5",
            Self::WarrensAH6 => "warrens-a-h-6",
            Self::WarrensAH7 => "warrens-a-h-7",
            Self::WarrensAH8 => "warrens-a-h-8",
            Self::WarrensAH9 => "warrens-a-h-9",
            Self::WarrensAH10 => "warrens-a-h-10",
            Self::WarrensAH11 => "warrens-a-h-11",
            Self::WarrensAH12 => "warrens-a-h-12",
            Self::WarrensAH13 => "warrens-a-h-13",
            Self::WarrensAH14 => "warrens-a-h-14",
            Self::WarrensAH15 => "warrens-a-h-15",
            Self::WarrensAH16 => "warrens-a-h-16",
            Self::WarrensAH17 => "warrens-a-h-17",
            Self::WarrensAH18 => "warrens-a-h-18",
            Self::WarrensAH19 => "warrens-a-h-19",
            Self::WarrensAH20 => "warrens-a-h-20",
            Self::WarrensAH21 => "warrens-a-h-21",
            Self::WarrensAH22 => "warrens-a-h-22",
            Self::WarrensAH23 => "warrens-a-h-23",
            Self::WarrensAH24 => "warrens-a-h-24",
            Self::WarrensAH25 => "warrens-a-h-25",
            Self::WarrensAH26 => "warrens-a-h-26",
            Self::WarrensAR1 => "warrens-a-r-1",
            Self::WarrensAR2 => "warrens-a-r-2",
            Self::WarrensAR3 => "warrens-a-r-3",
            Self::WarrensAR4 => "warrens-a-r-4",
            Self::WarrensAR5 => "warrens-a-r-5",
            Self::WarrensAR6 => "warrens-a-r-6",
            Self::WarrensAR7 => "warrens-a-r-7",
            Self::WarrensAR8 => "warrens-a-r-8",
            Self::WarrensAR9 => "warrens-a-r-9",
            Self::WarrensAR10 => "warrens-a-r-10",
            Self::WarrensAR11 => "warrens-a-r-11",
            Self::WarrensAR12 => "warrens-a-r-12",
            Self::WarrensAR13 => "warrens-a-r-13",
            Self::WarrensVH1 => "warrens-v-h-1",
            Self::WarrensVH2 => "warrens-v-h-2",
            Self::WarrensVH3 => "warrens-v-h-3",
            Self::WarrensVH4 => "warrens-v-h-4",
            Self::WarrensVH5 => "warrens-v-h-5",
            Self::WarrensVH6 => "warrens-v-h-6",
            Self::WarrensVH7 => "warrens-v-h-7",
            Self::WarrensVH8 => "warrens-v-h-8",
            Self::WarrensVH9 => "warrens-v-h-9",
            Self::WarrensVH10 => "warrens-v-h-10",
            Self::WarrensVH11 => "warrens-v-h-11",
            Self::WarrensVH12 => "warrens-v-h-12",
            Self::WarrensVH13 => "warrens-v-h-13",
            Self::WarrensVH14 => "warrens-v-h-14",
            Self::WarrensVH15 => "warrens-v-h-15",
            Self::WarrensVH16 => "warrens-v-h-16",
            Self::WarrensVH17 => "warrens-v-h-17",
            Self::WarrensVH18 => "warrens-v-h-18",
            Self::WarrensVH19 => "warrens-v-h-19",
            Self::WarrensVH20 => "warrens-v-h-20",
            Self::WarrensVH21 => "warrens-v-h-21",
            Self::WarrensVH22 => "warrens-v-h-22",
            Self::WarrensVH23 => "warrens-v-h-23",
            Self::WarrensVH24 => "warrens-v-h-24",
            Self::WarrensVH25 => "warrens-v-h-25",
            Self::WarrensVH26 => "warrens-v-h-26",
            Self::WarrensVH27 => "warrens-v-h-27",
            Self::WarrensVH28 => "warrens-v-h-28",
            Self::WarrensVH29 => "warrens-v-h-29",
            Self::WarrensVH30 => "warrens-v-h-30",
            Self::WarrensVH31 => "warrens-v-h-31",
            Self::WarrensVH32 => "warrens-v-h-32",
            Self::WarrensVH33 => "warrens-v-h-33",
            Self::WarrensVH34 => "warrens-v-h-34",
            Self::WarrensVH35 => "warrens-v-h-35",
            Self::WarrensVR1 => "warrens-v-r-1",
            Self::WarrensVR2 => "warrens-v-r-2",
            Self::WarrensVR3 => "warrens-v-r-3",
            Self::WarrensVR4 => "warrens-v-r-4",
            Self::WarrensVR5 => "warrens-v-r-5",
            Self::WarrensVR6 => "warrens-v-r-6",
            Self::WarrensVR7 => "warrens-v-r-7",
            Self::WarrensVR8 => "warrens-v-r-8",
            Self::WarrensVR9 => "warrens-v-r-9",
            Self::WarrensVR10 => "warrens-v-r-10",
            Self::WarrensVR11 => "warrens-v-r-11",
            Self::WarrensVR12 => "warrens-v-r-12",
            Self::WarrensVR13 => "warrens-v-r-13",
            Self::WarrensVR14 => "warrens-v-r-14",
            Self::WarrensVR15 => "warrens-v-r-15",
            Self::WarrensVR16 => "warrens-v-r-16",
            Self::WarrensVR17 => "warrens-v-r-17",
            Self::WarrensVR18 => "warrens-v-r-18",
            Self::WarrensVR19 => "warrens-v-r-19",
            Self::WarrensVR20 => "warrens-v-r-20",
            Self::WarrensVR21 => "warrens-v-r-21",
            Self::WarrensVR22 => "warrens-v-r-22",
            Self::WarrensCH1 => "warrens-c-h-1",
            Self::WarrensCH2 => "warrens-c-h-2",
            Self::WarrensCH3 => "warrens-c-h-3",
            Self::WarrensCH4 => "warrens-c-h-4",
            Self::WarrensCH5 => "warrens-c-h-5",
            Self::WarrensCH6 => "warrens-c-h-6",
            Self::WarrensCH7 => "warrens-c-h-7",
            Self::WarrensCH8 => "warrens-c-h-8",
            Self::WarrensCH9 => "warrens-c-h-9",
            Self::WarrensCH10 => "warrens-c-h-10",
            Self::WarrensCH11 => "warrens-c-h-11",
            Self::WarrensCH12 => "warrens-c-h-12",
            Self::WarrensCH13 => "warrens-c-h-13",
            Self::WarrensCH14 => "warrens-c-h-14",
            Self::WarrensCH15 => "warrens-c-h-15",
            Self::WarrensCH16 => "warrens-c-h-16",
            Self::WarrensCH17 => "warrens-c-h-17",
            Self::WarrensCH18 => "warrens-c-h-18",
            Self::WarrensCH19 => "warrens-c-h-19",
            Self::WarrensCH20 => "warrens-c-h-20",
            Self::WarrensCH21 => "warrens-c-h-21",
            Self::WarrensCH22 => "warrens-c-h-22",
            Self::WarrensCH23 => "warrens-c-h-23",
            Self::WarrensCH24 => "warrens-c-h-24",
            Self::WarrensCH25 => "warrens-c-h-25",
            Self::WarrensCH26 => "warrens-c-h-26",
            Self::WarrensCH27 => "warrens-c-h-27",
            Self::WarrensCH28 => "warrens-c-h-28",
            Self::WarrensCH29 => "warrens-c-h-29",
            Self::WarrensCH30 => "warrens-c-h-30",
            Self::WarrensCH31 => "warrens-c-h-31",
            Self::WarrensCH32 => "warrens-c-h-32",
            Self::WarrensCH33 => "warrens-c-h-33",
            Self::WarrensCH34 => "warrens-c-h-34",
            Self::WarrensCH35 => "warrens-c-h-35",
            Self::WarrensCH36 => "warrens-c-h-36",
            Self::WarrensCH37 => "warrens-c-h-37",
            Self::WarrensCH38 => "warrens-c-h-38",
            Self::WarrensCH39 => "warrens-c-h-39",
            Self::WarrensCR1 => "warrens-c-r-1",
            Self::WarrensCR2 => "warrens-c-r-2",
            Self::WarrensCR3 => "warrens-c-r-3",
            Self::WarrensCR4 => "warrens-c-r-4",
            Self::WarrensCR5 => "warrens-c-r-5",
            Self::WarrensCR6 => "warrens-c-r-6",
            Self::WarrensCR7 => "warrens-c-r-7",
            Self::WarrensCR8 => "warrens-c-r-8",
            Self::WarrensCR9 => "warrens-c-r-9",
            Self::WarrensCR10 => "warrens-c-r-10",
            Self::WarrensCR11 => "warrens-c-r-11",
            Self::WarrensCR12 => "warrens-c-r-12",
            Self::WarrensCR13 => "warrens-c-r-13",
            Self::WarrensCR14 => "warrens-c-r-14",
            Self::WarrensCR15 => "warrens-c-r-15",
            Self::WarrensCR16 => "warrens-c-r-16",
            Self::WarrensCR17 => "warrens-c-r-17",
            Self::WarrensCR18 => "warrens-c-r-18",
            Self::WarrensCR19 => "warrens-c-r-19",
            Self::WarrensCR20 => "warrens-c-r-20",
            Self::WarrensCR21 => "warrens-c-r-21",
            Self::WarrensCR22 => "warrens-c-r-22",
            Self::WarrensCR23 => "warrens-c-r-23",
            Self::WarrensCR24 => "warrens-c-r-24",
            Self::WarrensCR25 => "warrens-c-r-25",
            Self::WarrensCR26 => "warrens-c-r-26",
            Self::WarrensCR27 => "warrens-c-r-27",
            Self::WarrensCR28 => "warrens-c-r-28",
            Self::WealdAH1 => "weald-a-h-1",
            Self::WealdAH2 => "weald-a-h-2",
            Self::WealdAH3 => "weald-a-h-3",
            Self::WealdAH4 => "weald-a-h-4",
            Self::WealdAH5 => "weald-a-h-5",
            Self::WealdAH6 => "weald-a-h-6",
            Self::WealdAH7 => "weald-a-h-7",
            Self::WealdAH8 => "weald-a-h-8",
            Self::WealdAH9 => "weald-a-h-9",
            Self::WealdAH10 => "weald-a-h-10",
            Self::WealdAH11 => "weald-a-h-11",
            Self::WealdAH12 => "weald-a-h-12",
            Self::WealdAH13 => "weald-a-h-13",
            Self::WealdAH14 => "weald-a-h-14",
            Self::WealdAH15 => "weald-a-h-15",
            Self::WealdAH16 => "weald-a-h-16",
            Self::WealdAH17 => "weald-a-h-17",
            Self::WealdAH18 => "weald-a-h-18",
            Self::WealdAH19 => "weald-a-h-19",
            Self::WealdAH20 => "weald-a-h-20",
            Self::WealdAH21 => "weald-a-h-21",
            Self::WealdAR1 => "weald-a-r-1",
            Self::WealdAR2 => "weald-a-r-2",
            Self::WealdAR3 => "weald-a-r-3",
            Self::WealdAR4 => "weald-a-r-4",
            Self::WealdAR5 => "weald-a-r-5",
            Self::WealdAR6 => "weald-a-r-6",
            Self::WealdAR7 => "weald-a-r-7",
            Self::WealdAR8 => "weald-a-r-8",
            Self::WealdAR9 => "weald-a-r-9",
            Self::WealdAR10 => "weald-a-r-10",
            Self::WealdAR11 => "weald-a-r-11",
            Self::WealdAR12 => "weald-a-r-12",
            Self::WealdAR13 => "weald-a-r-13",
            Self::WealdAR14 => "weald-a-r-14",
            Self::WealdAR15 => "weald-a-r-15",
            Self::WealdAR16 => "weald-a-r-16",
            Self::WealdVH1 => "weald-v-h-1",
            Self::WealdVH2 => "weald-v-h-2",
            Self::WealdVH3 => "weald-v-h-3",
            Self::WealdVH4 => "weald-v-h-4",
            Self::WealdVH5 => "weald-v-h-5",
            Self::WealdVH6 => "weald-v-h-6",
            Self::WealdVH7 => "weald-v-h-7",
            Self::WealdVH8 => "weald-v-h-8",
            Self::WealdVH9 => "weald-v-h-9",
            Self::WealdVH10 => "weald-v-h-10",
            Self::WealdVH11 => "weald-v-h-11",
            Self::WealdVH12 => "weald-v-h-12",
            Self::WealdVH13 => "weald-v-h-13",
            Self::WealdVH14 => "weald-v-h-14",
            Self::WealdVH15 => "weald-v-h-15",
            Self::WealdVH16 => "weald-v-h-16",
            Self::WealdVH17 => "weald-v-h-17",
            Self::WealdVH18 => "weald-v-h-18",
            Self::WealdVH19 => "weald-v-h-19",
            Self::WealdVH20 => "weald-v-h-20",
            Self::WealdVH21 => "weald-v-h-21",
            Self::WealdVH22 => "weald-v-h-22",
            Self::WealdVH23 => "weald-v-h-23",
            Self::WealdVH24 => "weald-v-h-24",
            Self::WealdVH25 => "weald-v-h-25",
            Self::WealdVH26 => "weald-v-h-26",
            Self::WealdVH27 => "weald-v-h-27",
            Self::WealdVH28 => "weald-v-h-28",
            Self::WealdVH29 => "weald-v-h-29",
            Self::WealdVH30 => "weald-v-h-30",
            Self::WealdVH31 => "weald-v-h-31",
            Self::WealdVH32 => "weald-v-h-32",
            Self::WealdVH33 => "weald-v-h-33",
            Self::WealdVH34 => "weald-v-h-34",
            Self::WealdVH35 => "weald-v-h-35",
            Self::WealdVH36 => "weald-v-h-36",
            Self::WealdVR1 => "weald-v-r-1",
            Self::WealdVR2 => "weald-v-r-2",
            Self::WealdVR3 => "weald-v-r-3",
            Self::WealdVR4 => "weald-v-r-4",
            Self::WealdVR5 => "weald-v-r-5",
            Self::WealdVR6 => "weald-v-r-6",
            Self::WealdVR7 => "weald-v-r-7",
            Self::WealdVR8 => "weald-v-r-8",
            Self::WealdVR9 => "weald-v-r-9",
            Self::WealdVR10 => "weald-v-r-10",
            Self::WealdVR11 => "weald-v-r-11",
            Self::WealdVR12 => "weald-v-r-12",
            Self::WealdVR13 => "weald-v-r-13",
            Self::WealdVR14 => "weald-v-r-14",
            Self::WealdVR15 => "weald-v-r-15",
            Self::WealdVR16 => "weald-v-r-16",
            Self::WealdVR17 => "weald-v-r-17",
            Self::WealdVR18 => "weald-v-r-18",
            Self::WealdVR19 => "weald-v-r-19",
            Self::WealdVR20 => "weald-v-r-20",
            Self::WealdVR21 => "weald-v-r-21",
            Self::WealdVR22 => "weald-v-r-22",
            Self::WealdVR23 => "weald-v-r-23",
            Self::WealdCH1 => "weald-c-h-1",
            Self::WealdCH2 => "weald-c-h-2",
            Self::WealdCH3 => "weald-c-h-3",
            Self::WealdCH4 => "weald-c-h-4",
            Self::WealdCH5 => "weald-c-h-5",
            Self::WealdCH6 => "weald-c-h-6",
            Self::WealdCH7 => "weald-c-h-7",
            Self::WealdCH8 => "weald-c-h-8",
            Self::WealdCH9 => "weald-c-h-9",
            Self::WealdCH10 => "weald-c-h-10",
            Self::WealdCH11 => "weald-c-h-11",
            Self::WealdCH12 => "weald-c-h-12",
            Self::WealdCH13 => "weald-c-h-13",
            Self::WealdCH14 => "weald-c-h-14",
            Self::WealdCH15 => "weald-c-h-15",
            Self::WealdCH16 => "weald-c-h-16",
            Self::WealdCH17 => "weald-c-h-17",
            Self::WealdCH18 => "weald-c-h-18",
            Self::WealdCH19 => "weald-c-h-19",
            Self::WealdCH20 => "weald-c-h-20",
            Self::WealdCH21 => "weald-c-h-21",
            Self::WealdCH22 => "weald-c-h-22",
            Self::WealdCH23 => "weald-c-h-23",
            Self::WealdCH24 => "weald-c-h-24",
            Self::WealdCH25 => "weald-c-h-25",
            Self::WealdCH26 => "weald-c-h-26",
            Self::WealdCH27 => "weald-c-h-27",
            Self::WealdCH28 => "weald-c-h-28",
            Self::WealdCH29 => "weald-c-h-29",
            Self::WealdCH30 => "weald-c-h-30",
            Self::WealdCH31 => "weald-c-h-31",
            Self::WealdCH32 => "weald-c-h-32",
            Self::WealdCH33 => "weald-c-h-33",
            Self::WealdCH34 => "weald-c-h-34",
            Self::WealdCH35 => "weald-c-h-35",
            Self::WealdCH36 => "weald-c-h-36",
            Self::WealdCR1 => "weald-c-r-1",
            Self::WealdCR2 => "weald-c-r-2",
            Self::WealdCR3 => "weald-c-r-3",
            Self::WealdCR4 => "weald-c-r-4",
            Self::WealdCR5 => "weald-c-r-5",
            Self::WealdCR6 => "weald-c-r-6",
            Self::WealdCR7 => "weald-c-r-7",
            Self::WealdCR8 => "weald-c-r-8",
            Self::WealdCR9 => "weald-c-r-9",
            Self::WealdCR10 => "weald-c-r-10",
            Self::WealdCR11 => "weald-c-r-11",
            Self::WealdCR12 => "weald-c-r-12",
            Self::WealdCR13 => "weald-c-r-13",
            Self::WealdCR14 => "weald-c-r-14",
            Self::WealdCR15 => "weald-c-r-15",
            Self::WealdCR16 => "weald-c-r-16",
            Self::WealdCR17 => "weald-c-r-17",
            Self::WealdCR18 => "weald-c-r-18",
            Self::WealdCR19 => "weald-c-r-19",
            Self::WealdCR20 => "weald-c-r-20",
            Self::WealdCR21 => "weald-c-r-21",
            Self::WealdCR22 => "weald-c-r-22",
            Self::WealdCR23 => "weald-c-r-23",
            Self::WealdCR24 => "weald-c-r-24",
            Self::WealdCR25 => "weald-c-r-25",
            Self::WealdCR26 => "weald-c-r-26",
            Self::WealdCR27 => "weald-c-r-27",
            Self::WealdCR28 => "weald-c-r-28",
            Self::WealdCR29 => "weald-c-r-29",
            Self::DarkestDH1 => "darkest-d-h-1",
            Self::DarkestDH2 => "darkest-d-h-2",
            Self::DarkestDH3 => "darkest-d-h-3",
            Self::DarkestDH4 => "darkest-d-h-4",
            Self::DarkestDH5 => "darkest-d-h-5",
            Self::DarkestDH6 => "darkest-d-h-6",
            Self::DarkestDH7 => "darkest-d-h-7",
            Self::DarkestDH8 => "darkest-d-h-8",
            Self::DarkestDH9 => "darkest-d-h-9",
            Self::DarkestDH10 => "darkest-d-h-10",
            Self::DarkestDH11 => "darkest-d-h-11",
            Self::DarkestDH12 => "darkest-d-h-12",
            Self::DarkestDH13 => "darkest-d-h-13",
            Self::DarkestDH14 => "darkest-d-h-14",
            Self::DarkestDH15 => "darkest-d-h-15",
            Self::DarkestDH16 => "darkest-d-h-16",
            Self::DarkestDH17 => "darkest-d-h-17",
            Self::DarkestDH18 => "darkest-d-h-18",
            Self::DarkestDH19 => "darkest-d-h-19",
            Self::DarkestDH20 => "darkest-d-h-20",
            Self::DarkestDH21 => "darkest-d-h-21",
            Self::DarkestDH22 => "darkest-d-h-22",
            Self::DarkestDH23 => "darkest-d-h-23",
            Self::DarkestDH24 => "darkest-d-h-24",
            Self::DarkestDR1 => "darkest-d-r-1",
            Self::DarkestDR2 => "darkest-d-r-2",
            Self::DarkestDR3 => "darkest-d-r-3",
            Self::DarkestDR4 => "darkest-d-r-4",
            Self::DarkestDR5 => "darkest-d-r-5",
            Self::DarkestDR6 => "darkest-d-r-6",
            Self::DarkestDR7 => "darkest-d-r-7",
            Self::DarkestDR8 => "darkest-d-r-8",
            Self::DarkestDR9 => "darkest-d-r-9",
            Self::DarkestDR10 => "darkest-d-r-10",
            Self::DarkestDR11 => "darkest-d-r-11",
            Self::DarkestDR12 => "darkest-d-r-12",
            Self::DarkestDR13 => "darkest-d-r-13",
            Self::DarkestDR14 => "darkest-d-r-14",
            Self::DarkestDR15 => "darkest-d-r-15",
            Self::DarkestDR16 => "darkest-d-r-16",
            Self::DarkestDR17 => "darkest-d-r-17",
            Self::DarkestDR18 => "darkest-d-r-18",
            Self::DarkestDR19 => "darkest-d-r-19",
            Self::DarkestDR20 => "darkest-d-r-20",
            Self::DarkestDR21 => "darkest-d-r-21",
            Self::DarkestDR22 => "darkest-d-r-22",
            Self::DarkestDR23 => "darkest-d-r-23",
        }
    }
}
