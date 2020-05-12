use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Season {
    Previous,
    Current,
    Specific {
        expansion: crate::Expansion,
        number: u8,
    },
}
impl std::str::FromStr for Season {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("-");
        iter.next();
        let expansion = match iter.next() {
            None => Err("no expansion")?,
            Some("bfa") => crate::Expansion::BattleForAzeroth,
            Some(_) => Err("unknown expansion")?,
        };
        let number = match iter.next().map(|s| s.parse()) {
            None => Err("no season number")?,
            Some(Ok(n)) => n,
            Some(Err(_)) => Err("invalid number")?,
        };
        Ok(Season::Specific { expansion, number })
    }
}
fn deser_season<'de, D>(deserializer: D) -> Result<Season, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct SeasonVisitor;

    impl<'de> serde::de::Visitor<'de> for SeasonVisitor {
        type Value = Season;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "expected season")
        }

        fn visit_str<E>(self, value: &str) -> Result<Season, E>
        where
            E: serde::de::Error,
        {
            std::str::FromStr::from_str(value).map_err(E::custom)
        }
    }

    deserializer.deserialize_string(SeasonVisitor)
}

#[derive(Clone, Copy)]
pub(crate) enum SeasonBestRuns {
    All,
    Three,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Ranking {
    pub world: u64,
    pub region: u64,
    pub realm: u64,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct MythicPlusRanks {
    pub overall: Ranking,
    pub faction_overall: Ranking,

    pub class: Ranking,
    pub faction_class: Ranking,

    pub dps: Option<Ranking>,
    pub faction_dps: Option<Ranking>,

    pub class_dps: Option<Ranking>,
    pub faction_class_dps: Option<Ranking>,

    pub tank: Option<Ranking>,
    pub faction_tank: Option<Ranking>,

    pub class_tank: Option<Ranking>,
    pub faction_class_tank: Option<Ranking>,

    pub healer: Option<Ranking>,
    pub faction_healer: Option<Ranking>,

    pub class_healer: Option<Ranking>,
    pub faction_class_healer: Option<Ranking>,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct MythicPlusScores {
    #[serde(deserialize_with = "deser_season")]
    pub season: Season,
    pub scores: Scores,
}
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Scores {
    pub all: f32,
    pub dps: f32,
    pub healer: f32,
    pub tank: f32,
    pub spec_0: f32,
    pub spec_1: f32,
    pub spec_2: f32,
    pub spec_3: f32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Dungeon {
    #[serde(rename = "Mechagon Workshop")]
    MechagonWorkshop,
    #[serde(rename = "Mechagon Junkyard")]
    MechagonJunkyard,
    #[serde(rename = "Kings' Rest")]
    KingsRest,
    #[serde(rename = "Freehold")]
    Freehold,
    #[serde(rename = "Temple of Sethraliss")]
    TempleOfSethraliss,
    #[serde(rename = "Tol Dagor")]
    TolDagor,
    #[serde(rename = "The Underrot")]
    TheUnderrot,
    #[serde(rename = "Shrine of the Storm")]
    ShrineOfTheStorm,
    #[serde(rename = "Atal'dazar")]
    AtalDazar,
    #[serde(rename = "The MOTHERLODE!!")]
    TheMotherlode,
    #[serde(rename = "Siege of Boralus")]
    SiegeOfBoralus,
    #[serde(rename = "Waycrest Manor")]
    WaycrestManor,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum AffixName {
    Fortified,
    Tyrannical,
    Bolstering,
    Raging,
    Sanguine,
    Teeming,
    Bursting,
    Necrotic,
    Skittish,
    Volcanic,
    Explosive,
    Quaking,
    Grievous,
    Infested,
    Reaping,
    Beguiling,
    Awakened,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Affix {
    pub id: u64,
    pub name: AffixName,
    pub description: String,
    pub wowhead_url: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KeystoneRun {
    pub dungeon: Dungeon,
    pub mythic_level: u8,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub clear_time_ms: u64,
    pub num_keystone_upgrades: u8,
    pub map_challenge_mode_id: u64,
    pub score: f32,
    pub affixes: Vec<Affix>,
}
