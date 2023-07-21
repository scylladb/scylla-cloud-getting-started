use crate::songs::Song;

pub struct Database {
    pub songs: Vec<Song>,
}

impl Database {
    pub fn new() -> Self {
        return Self { songs: vec![] };
    }

    pub fn add(&mut self, item: Song) -> () {
        self.songs.push(item);
    }

    pub fn remove(&mut self, index: usize) -> () {
        
        

        self.songs.remove(index);
    }
}
