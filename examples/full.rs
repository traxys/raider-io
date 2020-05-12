use raider_io::{Client, Expansion, Region, mythic_plus::Season};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let details = client
        .character_details(Region::Europe, "Andybrew", "Draenor")
        .guild()
        .gear()
        .mythic_plus_score_by_season(Season::Current)
        .mythic_plus_score_by_season(Season::Specific {
            expansion: Expansion::BattleForAzeroth,
            number: 1,
        })
        .mythic_plus_ranks()
        .raid_progression()
        .mythic_plus_recent_runs()
        .mythic_plus_all_best_runs()
        .mythic_plus_three_best_runs()
        .mythic_plus_highest_runs()
        .mythic_plus_weekly_highest_level_runs()
        .mythic_plus_previous_weekly_highest_level_runs()
        .previous_mythic_plus_ranks()
        .get()
        .await?;

    dbg!(details);

    Ok(())
}
