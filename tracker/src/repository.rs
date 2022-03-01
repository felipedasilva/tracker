use crate::model::Track;

pub trait TrackRepository {
    fn save(&self, track: &Track) -> Result<(), String>;
    fn find(&self, id: String) -> Result<Track, String>;
    fn find_all(&self) -> Result<Vec<Track>, String>;
}
