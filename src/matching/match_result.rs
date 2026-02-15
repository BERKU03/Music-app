#[derive(Debug)]
pub struct MatchResult {
    pub local_song_id: i64,
    pub confidence: f32, // 0.0 - 1.0
}
