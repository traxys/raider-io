use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Corruption {
    pub added: u64,
    pub resisted: u64,
    pub total: u64,
    #[serde(rename = "cloakRank")]
    pub cloak_ranks: Option<u8>,
    pub spells: Option<Vec<Spell>>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Spell {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub school: u64,
    pub rank: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Items {
    pub head: Item,
    pub neck: Item,
    pub shoulder: Item,
    pub back: Item,
    pub chest: Item,
    pub waist: Item,
    pub shirt: Item,
    pub wrist: Item,
    pub hands: Item,
    pub legs: Item,
    pub feet: Item,
    pub finger1: Item,
    pub finger2: Item,
    pub trinket1: Item,
    pub trinket2: Item,
    pub mainhand: Item,
    pub offhand: Item,
}
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Quality {
    Poor = 0,
    Common = 1,
    Uncommon = 2,
    Rare = 3,
    Epic = 4,
    Legendary = 5,
    Artifact = 6,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AzeritePower {
    pub id: u64,
    pub spell: Spell,
    pub tier: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum EssenceName {
    #[serde(rename = "Blood of the Enemy")]
    BloodOfTheEnemy,
    #[serde(rename = "Vision of Perfection")]
    VisionOfPerfection,
    #[serde(rename = "Memory of Lucid Dreams")]
    MemoryOfLucidDreams,
    #[serde(rename = "Breath of the Dying")]
    BreathOfTheDying,
    #[serde(rename = "Formless Void")]
    FormlessVoid,
    #[serde(rename = "Strength of the Warden")]
    StrengthOfTheWarden,
    #[serde(rename = "Touch of the Everlasting")]
    TouchOfTheEverlasting,
    #[serde(rename = "Spark of Inspiration")]
    SparkOfInspiration,
    #[serde(rename = "Unwavering Ward")]
    UnwaveringWard,
    #[serde(rename = "Spirit of Preservation")]
    SpiritOfPreservation,
    #[serde(rename = "The Crucible of Flame")]
    TheCrucibleOfFlame,
    #[serde(rename = "Worldvein Resonance")]
    WorldveinResonance,
    #[serde(rename = "Ripple in Space")]
    RippleInSpace,
    #[serde(rename = "Conflict and Strife")]
    ConflictAndStrife,
    #[serde(rename = "Aegis of the Deep")]
    AegisOfTheDeep,
    #[serde(rename = "Nullification Dynamo")]
    NullificationDynamo,
    #[serde(rename = "Sphere of Suppression")]
    SphereOfSuppression,
    #[serde(rename = "Azeroth's Undying Gift")]
    AzerothsUndyingGift,
    #[serde(rename = "Anima of Life and Death")]
    AnimaOfLifeAndDeath,
    #[serde(rename = "The Ever-Rising Tide")]
    TheEverRisingTide,
    #[serde(rename = "The Well of Existence")]
    TheWellOfExistence,
    #[serde(rename = "Artifice of Time")]
    ArtificeOfTime,
    #[serde(rename = "Life-Binder's Invocation")]
    LifeBindersInvocation,
    #[serde(rename = "Vitality Conduit")]
    VitalityConduit,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Essence {
    pub id: u64,
    pub name: EssenceName,
    pub description: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EssencePower {
    pub id: u64,
    pub essence: Essence,
    #[serde(rename = "tierId")]
    pub tier_id: u64,
    #[serde(rename = "majorPowerSpell")]
    pub major_power_spell: Spell,
    #[serde(rename = "minorPowerSpell")]
    pub minor_power_spell: Spell,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EssenceSlot {
    pub slot: u8,
    pub id: u64,
    pub rank: u8,
    pub power: EssencePower,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HeartOfAzeroth {
    pub essences: Vec<EssenceSlot>,
    pub level: u64,
    pub progress: f64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    pub item_id: u64,
    pub item_level: u64,
    pub item_quality: Quality,
    pub icon: String,
    pub is_legion_legendary: bool,
    pub is_azerite_armor: bool,
    pub azerite_powers: Vec<AzeritePower>,
    pub corruption: Corruption,
    pub gems: Vec<u64>,
    pub bonuses: Vec<u64>,
    pub heart_of_azeroth: Option<HeartOfAzeroth>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Gear {
    pub item_level_equipped: u64,
    pub item_level_total: u64,
    pub artifact_traits: f32,
    pub corruption: Corruption,
    pub items: Items,
}
