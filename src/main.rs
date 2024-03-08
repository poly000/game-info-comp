use std::{collections::BTreeMap, env, error::Error, fs};

mod file_schema;
use beatmapset_info::BeatmapInfo;
use file_schema::{GameInformation, SongInfo, SongList};

use crate::beatmapset_info::Level;
mod beatmapset_info;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> Result<()> {
    let (old_game_info, new_game_info) = parse_game_info()?;
    let old_songs = song_info_map(list_song_info(old_game_info));
    let new_songs = song_info_map(list_song_info(new_game_info));
    for (name, new_info) in map_diff(&old_songs, &new_songs) {
        if let Some(old_info) = old_songs.get(name) {
            println!("name: {name}");
            let old_levels = BeatmapInfo::from(old_info.clone())
                .levels
                .into_iter()
                .map(|l| (l.name.clone(), l))
                .collect();
            let new_levels = BeatmapInfo::from(new_info.clone())
                .levels
                .into_iter()
                .map(|l| (l.name.clone(), l))
                .collect();
            for (level_name, new_level) in map_diff(&old_levels, &new_levels) {
                if let Some(old_level) = old_levels.get(level_name) {
                    println!(
                        "{level_name}: {} -> {}",
                        old_level.difficulty, new_level.difficulty
                    );
                } else {
                    println!("* {level_name}: {}", new_level.difficulty);
                }
            }
        } else {
            println!("* name: {name}");
            for Level { name, difficulty } in BeatmapInfo::from(new_info.clone()).levels {
                println!("{name}: {difficulty}");
            }
        }
        println!();
    }
    Ok(())
}

fn map_diff<'r, K, V>(
    left: &'r BTreeMap<K, V>,
    right: &'r BTreeMap<K, V>,
) -> impl Iterator<Item = (&'r K, &'r V)>
where
    K: std::hash::Hash + Eq + Ord,
    V: Eq,
{
    right.iter().filter(|(name, value)| {
        if !left.contains_key(*name) {
            true
        } else {
            &&left[*name] != value
        }
    })
}

fn parse_game_info() -> Result<(GameInformation, GameInformation)> {
    let old = env::args().nth(1).ok_or("old json is required")?;
    let old = fs::read_to_string(old)?;
    let new = env::args().nth(2).ok_or("new json is required")?;
    let new = fs::read_to_string(new)?;
    let old_data: GameInformation = serde_json::from_str(&old)?;
    let new_data: GameInformation = serde_json::from_str(&new)?;
    Ok((old_data, new_data))
}

fn song_info_map(song_infos: impl Iterator<Item = SongInfo>) -> BTreeMap<String, SongInfo> {
    song_infos
        .map(|song_info| (song_info.songs_name.clone(), song_info))
        .collect()
}

fn list_song_info(game_info: GameInformation) -> impl Iterator<Item = SongInfo> {
    let GameInformation { song } = game_info;
    let SongList {
        main_songs,
        extra_songs,
        side_story_songs,
        other_songs,
    } = song;
    let main = main_songs.into_iter();
    let extra = extra_songs.into_iter();
    let side_story = side_story_songs.into_iter();
    let other_songs = other_songs.into_iter();
    main.chain(extra).chain(side_story).chain(other_songs)
}
