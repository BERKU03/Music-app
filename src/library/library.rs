use super::song::Song;

#[derive(Debug, Default)]
pub struct Library {
    pub songs: Vec<Song>,
}

impl Library {
    pub fn new() -> Self {
        Library { songs: Vec::new() }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub fn total_songs(&self) -> usize {
        self.songs.len()
    }
}
