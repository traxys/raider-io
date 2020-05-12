use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

static BASE_URL: &str = "https://raider.io/api/v1";

pub mod gear;
pub mod mythic_plus;
pub mod player;
pub mod raid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http request failed")]
    Http(#[from] reqwest::Error),
    #[error("api error: {message}")]
    Api {
        status: u64,
        error: String,
        message: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Region {
    #[serde(rename = "us")]
    UnitedStates,
    #[serde(rename = "eu")]
    Europe,
    #[serde(rename = "kr")]
    Korea,
    #[serde(rename = "tw")]
    Taiwan,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Expansion {
    BattleForAzeroth,
}
impl Expansion {
    fn text(&self) -> &'static str {
        match self {
            Expansion::BattleForAzeroth => "bfa",
        }
    }
}

pub struct Client {
    http_client: reqwest::Client,
}
impl Client {
    pub fn new() -> Client {
        Client {
            http_client: reqwest::Client::new(),
        }
    }
    pub fn character_details<'s, 'i: 's>(
        &'s self,
        region: Region,
        name: &'i str,
        realm: &'i str,
    ) -> CharacterDetailsRequest<'s> {
        CharacterDetailsRequest {
            client: self,
            name,
            region,
            realm,
            fields: CharacterDetailsFields::default(),
        }
    }
}

struct CharacterDetailsFields {
    gear: bool,
    guild: bool,
    raid_progression: bool,
    mythic_plus_by_season: Option<HashSet<mythic_plus::Season>>,
    mythic_plus_ranks: bool,
    mythic_plus_recent_runs: bool,
    mythic_plus_best_runs: Option<mythic_plus::SeasonBestRuns>,
    mythic_plus_highest_runs: bool,
    mythic_plus_weekly_higest_runs: bool,
    mythic_plus_previous_week_highest_runs: bool,
    mythic_plus_previous_week_ranking: bool,
    raid_achievement_meta: Option<Vec<u8>>,
    raid_achievement_curve: Option<Vec<u8>>,
}

impl Default for CharacterDetailsFields {
    fn default() -> Self {
        CharacterDetailsFields {
            gear: false,
            guild: false,
            raid_progression: false,
            mythic_plus_ranks: false,
            mythic_plus_recent_runs: false,
            mythic_plus_weekly_higest_runs: false,
            mythic_plus_best_runs: None,
            mythic_plus_highest_runs: false,
            mythic_plus_previous_week_ranking: false,
            mythic_plus_previous_week_highest_runs: false,
            mythic_plus_by_season: None,
            raid_achievement_curve: None,
            raid_achievement_meta: None,
        }
    }
}
impl CharacterDetailsFields {
    fn text(&self) -> String {
        CharacterDetailsFieldsIter {
            fields: self,
            index: 0,
        }
        .join(",")
    }
}

macro_rules! s_str {
    ($e:expr) => {
        Some($e.to_owned())
    };
}
struct CharacterDetailsFieldsIter<'c> {
    fields: &'c CharacterDetailsFields,
    index: u8,
}
impl<'c> Iterator for CharacterDetailsFieldsIter<'c> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 12 {
            return None;
        }
        loop {
            let index = self.index;
            self.index += 1;
            match index {
                0 if self.fields.gear => return s_str!("gear"),
                1 if self.fields.guild => return s_str!("guild"),
                2 if self.fields.raid_progression => return s_str!("raid_progression"),
                3 if self.fields.mythic_plus_by_season.is_some() => {
                    let mut seasons = String::new();
                    for season in self.fields.mythic_plus_by_season.clone().unwrap() {
                        match season {
                            mythic_plus::Season::Previous => {
                                seasons = format!("{}:previous", seasons);
                            }
                            mythic_plus::Season::Current => {
                                seasons = format!("{}:current", seasons);
                            }
                            mythic_plus::Season::Specific { expansion, number } => {
                                seasons =
                                    format!("{}:season-{}-{}", seasons, expansion.text(), number);
                            }
                        }
                    }
                    return Some(format!("mythic_plus_scores_by_season{}", seasons));
                }
                4 if self.fields.mythic_plus_ranks => return s_str!("mythic_plus_ranks"),
                5 if self.fields.mythic_plus_recent_runs => {
                    return s_str!("mythic_plus_recent_runs")
                }
                6 if self.fields.mythic_plus_best_runs.is_some() => {
                    let mut field = "mythic_plus_best_runs".to_owned();
                    if let mythic_plus::SeasonBestRuns::All =
                        self.fields.mythic_plus_best_runs.unwrap()
                    {
                        field = format!("{}:all", field);
                    }
                    return Some(field);
                }
                7 if self.fields.mythic_plus_highest_runs => {
                    return s_str!("mythic_plus_highest_level_runs")
                }
                8 if self.fields.mythic_plus_weekly_higest_runs => {
                    return s_str!("mythic_plus_weekly_highest_level_runs")
                }
                9 if self.fields.mythic_plus_previous_week_highest_runs => {
                    return s_str!("mythic_plus_previous_weekly_highest_level_runs")
                }
                10 if self.fields.mythic_plus_previous_week_ranking => {
                    return s_str!("previous_mythic_plus_ranks")
                }
                11 if self.fields.raid_achievement_meta.is_some() => todo!(),
                12 if self.fields.raid_achievement_curve.is_some() => todo!(),
                0..=12 => continue,
                _ => return None,
            }
        }
    }
}
pub struct CharacterDetailsRequest<'c> {
    client: &'c Client,
    name: &'c str,
    region: Region,
    realm: &'c str,
    fields: CharacterDetailsFields,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CharacterDetails {
    pub name: String,
    pub race: player::Race,
    pub class: player::Class,
    pub active_spec_name: player::Spec,
    pub active_spec_role: player::Role,
    pub gender: player::Gender,
    pub faction: player::Faction,
    pub region: Region,
    pub realm: String,
    pub profile_url: String,
    pub achievement_points: u64,
    pub honorable_kills: u64,
    pub thumbnail_url: String,
    pub gear: Option<gear::Gear>,
    pub guild: Option<player::Guild>,
    pub raid_progression: Option<raid::RaidProgression>,
    pub mythic_plus_ranks: Option<mythic_plus::MythicPlusRanks>,
    pub mythic_plus_scores_by_season: Option<Vec<mythic_plus::MythicPlusScores>>,
    pub mythic_plus_recent_runs: Option<Vec<mythic_plus::KeystoneRun>>,
    pub mythic_plus_best_runs: Option<Vec<mythic_plus::KeystoneRun>>,
    pub mythic_plus_highest_level_runs: Option<Vec<mythic_plus::KeystoneRun>>,
    pub mythic_plus_weekly_highest_level_runs: Option<Vec<mythic_plus::KeystoneRun>>,
    pub mythic_plus_previous_weekly_highest_level_runs: Option<Vec<mythic_plus::KeystoneRun>>,
    pub previous_mythic_plus_ranks: Option<mythic_plus::MythicPlusRanks>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CharacterQuery<'i> {
    name: &'i str,
    region: Region,
    realm: &'i str,
    fields: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    #[serde(rename = "statusCode")]
    status_code: u64,
    error: String,
    message: String,
}

impl<'c> CharacterDetailsRequest<'c> {
    /// Clear all the fields from the request
    pub fn clear(mut self) -> Self {
        self.fields = Default::default();
        self
    }
    /// retrieve basic information about guild the player is in
    pub fn guild(mut self) -> Self {
        self.fields.guild = true;
        self
    }
    /// retrieve high level item information for player
    pub fn gear(mut self) -> Self {
        self.fields.gear = true;
        self
    }
    /// retrieve scores by mythic plus season
    pub fn mythic_plus_score_by_season(mut self, season: mythic_plus::Season) -> Self {
        match &mut self.fields.mythic_plus_by_season {
            None => {
                let mut set = HashSet::new();
                set.insert(season);
                self.fields.mythic_plus_by_season = Some(set);
            }
            Some(ref mut set) => {
                set.insert(season);
            }
        };
        self
    }

    /// current season mythic plus rankings for player.
    pub fn mythic_plus_ranks(mut self) -> Self {
        self.fields.mythic_plus_ranks = true;
        self
    }

    /// retrieve raid progression data for character
    pub fn raid_progression(mut self) -> Self {
        self.fields.raid_progression = true;
        self
    }

    /// retrieve three most recent mythic plus runs for player (current season only).
    pub fn mythic_plus_recent_runs(mut self) -> Self {
        self.fields.mythic_plus_recent_runs = true;
        self
    }

    /// retrieve all of a character's best runs for the season
    pub fn mythic_plus_all_best_runs(mut self) -> Self {
        self.fields.mythic_plus_best_runs = Some(mythic_plus::SeasonBestRuns::All);
        self
    }
    /// retrieve three most high scoring mythic plus runs for player (current season only).
    pub fn mythic_plus_three_best_runs(mut self) -> Self {
        self.fields.mythic_plus_best_runs = Some(mythic_plus::SeasonBestRuns::Three);
        self
    }
    /// retrieve the player's three highest Mythic+ runs by Mythic+ level (current season only)
    pub fn mythic_plus_highest_runs(mut self) -> Self {
        self.fields.mythic_plus_highest_runs = true;
        self
    }
    /// retrieve the player's three highest Mythic+ runs by Mythic+ level for the current raid week (current season only)
    pub fn mythic_plus_weekly_highest_level_runs(mut self) -> Self {
        self.fields.mythic_plus_weekly_higest_runs = true;
        self
    }
    /// retrieve the player's three highest Mythic+ runs by Mythic+ level for the previous raid week (current season only)
    pub fn mythic_plus_previous_weekly_highest_level_runs(mut self) -> Self {
        self.fields.mythic_plus_previous_week_highest_runs = true;
        self
    }
    /// retrieve mythic plus rankings for player.
    pub fn previous_mythic_plus_ranks(mut self) -> Self {
        self.fields.mythic_plus_previous_week_ranking = true;
        self
    }

    /// Execute the query on the raider.io website
    pub async fn get(&self) -> Result<CharacterDetails, Error> {
        let fields = self.fields.text();
        let fields = if fields.is_empty() {
            None
        } else {
            Some(fields)
        };

        let query = CharacterQuery {
            name: self.name,
            region: self.region,
            realm: self.realm,
            fields,
        };
        let response = self
            .client
            .http_client
            .get(&format!("{}/characters/profile", BASE_URL))
            .query(&query)
            .send()
            .await?;

        if response.status().is_client_error() {
            let response: ApiError = response.json().await?;
            Err(Error::Api {
                status: response.status_code,
                error: response.error,
                message: response.message,
            })
        } else if response.status().is_success() {
            response
                .json::<CharacterDetails>()
                .await
                .map_err(From::from)
        } else {
            panic!("Unhandled status code");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
