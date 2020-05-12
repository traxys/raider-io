use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Progression {
    pub summary: String,
    pub total_bosses: u8,
    pub normal_bosses_killed: u8,
    pub heroic_bosses_killed: u8,
    pub mythic_bosses_killed: u8,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RaidProgression {
    #[serde(rename = "battle-of-dazaralor")]
    pub battle_of_dazaralor: Progression,
    #[serde(rename = "crucible-of-storms")]
    pub crucible_of_storms: Progression,
    #[serde(rename = "nyalotha-the-waking-city")]
    pub nyalotha_the_waking_city: Progression,
    #[serde(rename = "the-eternal-palace")]
    pub the_eternal_palace: Progression,
    #[serde(rename = "uldir")]
    pub uldir: Progression,
}
