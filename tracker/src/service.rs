use crate::model::Track;
use crate::repository::TrackRepository;

pub struct TrackService {
    repository: Box<dyn TrackRepository>,
    tracks: Vec<Track>,
    current_track_index: i64,
}

impl TrackService {
    pub fn create(repository: Box<dyn TrackRepository>) -> TrackService {
        let tracks = match repository.find_all() {
            Ok(tracks) => tracks,
            Err(_) => Vec::new(),
        };
        let current_track_index = (tracks.len() as i64) - 1;
        TrackService {
            repository,
            tracks,
            current_track_index,
        }
    }

    pub fn stop_current_track(&mut self) -> Result<(), ()> {
        if let Some(last_track) = self.tracks.get_mut(self.current_track_index as usize) {
            if last_track.is_tracking() {
                last_track.stop_track();
                self.repository.save(last_track).unwrap();
            }
        }
        Ok(())
    }

    pub fn start_new_track(
        &mut self,
        name: String,
        project: String,
        workspace: String,
    ) -> Result<&Track, ()> {
        self.stop_current_track().unwrap();
        let new_track = Track::start_new_track(name, project, workspace);
        self.repository.save(&new_track).unwrap();
        self.tracks.push(new_track);
        self.current_track_index = self.current_track_index + 1;
        Ok(self.tracks.get(self.current_track_index as usize).unwrap())
    }

    pub fn list(&self) -> Vec<Track> {
        self.tracks.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_sqlite::{RepositorySQLite, SCHEME};
    use std::sync::Arc;

    fn create_connection() -> sqlite::Connection {
        let connection = sqlite::open(":memory:").unwrap();
        connection.execute(SCHEME).unwrap();
        connection
    }

    fn create_repository(connection: sqlite::Connection) -> RepositorySQLite {
        RepositorySQLite::create(Arc::new(connection).clone())
    }

    #[test]
    fn test_stop_current_track() {
        let repository = Box::new(create_repository(create_connection()));
        repository
            .save(&Track::start_new_track(
                String::from("MyTrack"),
                String::from("Project1"),
                String::from("Workspace"),
            ))
            .unwrap();
        let mut service = TrackService::create(repository);
        service.stop_current_track().unwrap();
        let track = service.tracks.get(0).unwrap();
        assert_eq!(track.is_tracking(), false);
    }

    #[test]
    fn test_start_new_track() {
        let repository = Box::new(create_repository(create_connection()));
        let mut service = TrackService::create(repository);
        service
            .start_new_track(
                String::from("MyTrack"),
                String::from("Project1"),
                String::from("Workspace"),
            )
            .unwrap();
        let track = service.tracks.get(0).unwrap();
        assert_eq!(track.is_tracking(), true);
        assert_eq!(service.stop_current_track(), Ok(()));
        let track = service.tracks.get(0).unwrap();
        assert_eq!(track.is_tracking(), false);
        service
            .start_new_track(
                String::from("MyTrack2"),
                String::from("Project2"),
                String::from("Workspace2"),
            )
            .unwrap();
        let old_track = service.tracks.get(0).unwrap();
        assert_eq!(old_track.is_tracking(), false);
        let track = service.tracks.get(1).unwrap();
        assert_eq!(track.is_tracking(), true);
        service.stop_current_track().unwrap();
        let track = service.tracks.get(1).unwrap();
        assert_eq!(track.is_tracking(), false);
    }

    #[test]
    fn test_list() {
        let repository = Box::new(create_repository(create_connection()));
        let mut service = TrackService::create(repository);
        let list = service.list();
        assert_eq!(list.len(), 0);
        service
            .start_new_track(
                String::from("MyTrack"),
                String::from("Project1"),
                String::from("Workspace"),
            )
            .unwrap();
        service
            .start_new_track(
                String::from("MyTrack2"),
                String::from("Project1"),
                String::from("Workspace"),
            )
            .unwrap();
        let list = service.list();
        assert_eq!(list.len(), 2);
    }
}
