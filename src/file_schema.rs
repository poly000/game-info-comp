use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GameInformation {
    pub song: SongList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SongList {
    pub main_songs: Vec<SongInfo>,
    pub extra_songs: Vec<SongInfo>,
    pub side_story_songs: Vec<SongInfo>,
    pub other_songs: Vec<SongInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SongInfo {
    pub songs_name: String,
    pub difficulty: Vec<Decimal>,
    pub levels: Vec<String>,
}
