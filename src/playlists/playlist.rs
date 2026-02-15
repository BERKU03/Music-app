use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub source: PlaylistSource,
    pub tracks: Vec<PlaylistTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaylistSource {
    Local,
    AppleMusic,
    Imported,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub song_id: i64,   // referencia a Song
    pub position: u32,  // orden en la playlist
}
