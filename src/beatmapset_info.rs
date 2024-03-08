use rust_decimal::Decimal;

use crate::file_schema::SongInfo;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BeatmapInfo {
    pub songs_name: String,
    pub levels: Vec<Level>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Level {
    pub name: String,
    pub difficulty: Decimal,
}

impl From<super::file_schema::SongInfo> for BeatmapInfo {
    fn from(
        SongInfo {
            songs_name,
            difficulty,
            levels,
        }: SongInfo,
    ) -> Self {
        let levels = difficulty
            .into_iter()
            .zip(levels.into_iter())
            .map(|(difficulty, name)| Level { name, difficulty })
            .collect();
        Self { levels, songs_name }
    }
}
