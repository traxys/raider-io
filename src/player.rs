use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Guild {
    pub name: String,
    pub realm: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Gender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Faction {
    #[serde(rename = "horde")]
    Horde,
    #[serde(rename = "alliance")]
    Alliance,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Race {
    Human,
    Dwarf,
    Gnome,
    Draenei,
    Worgen,
    Pandaren,
    Orc,
    Undead,
    Tauren,
    Troll,
    Goblin,
    Mechagnome,
    Nightborne,
    Vulpera,
    #[serde(rename = "Night Elf")]
    NightElf,
    #[serde(rename = "Blood Elf")]
    BloodElf,
    #[serde(rename = "Void Elf")]
    VoidElf,
    #[serde(rename = "Lightforged Draenei")]
    LightforgedDraenei,
    #[serde(rename = "Dark Iron Dwarf")]
    DarkIronDwarf,
    #[serde(rename = "Kul Tiran")]
    KulTiran,
    #[serde(rename = "Highmountain Tauren")]
    HighmountainTauren,
    #[serde(rename = "Mag'har Orc")]
    MagharOrc,
    #[serde(rename = "Zandalari Troll")]
    ZandalariTroll,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Class {
    Rogue,
    Warrior,
    Paladin,
    Hunter,
    Priest,
    Shaman,
    Mage,
    Warlock,
    Monk,
    Druid,
    #[serde(rename = "Demon Hunter")]
    DemonHunter,
    #[serde(rename = "Death Knight")]
    DeathKnight,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Spec {
    Arms,
    Fury,
    Protection,
    Holy,
    Retribution,
    #[serde(rename = "Beast Mastery")]
    BeastMastery,
    Marksmanship,
    Survival,
    Outlaw,
    Assassination,
    Subtlety,
    Discipline,
    Shadow,
    Elemental,
    Restoration,
    Enhancement,
    Arcane,
    Fire,
    Frost,
    Affliction,
    Demonology,
    Destruction,
    Brewmaster,
    Mistweaver,
    Windwalker,
    Balance,
    Feral,
    Guardian,
    Havoc,
    Vengeance,
    Blood,
    Unholy,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Role {
    DPS,
    #[serde(rename = "TANK")]
    Tank,
    #[serde(rename = "HEALING")]
    Healing,
}
