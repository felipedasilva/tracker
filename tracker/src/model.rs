use chrono::prelude::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub project: String,
    pub workspace: String,
}

impl Track {
    pub fn create(
        id: String,
        name: String,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        project: String,
        workspace: String,
    ) -> Track {
        Track {
            id,
            name,
            start,
            end,
            project,
            workspace,
        }
    }

    pub fn start_new_track(name: String, project: String, workspace: String) -> Track {
        let id = Uuid::new_v4().hyphenated().to_string();
        let start = Utc::now();
        let end = None;
        Track {
            id,
            name,
            start,
            end,
            project,
            workspace,
        }
    }

    pub fn stop_track(&mut self) {
        self.end = Some(Utc::now());
    }

    pub fn is_tracking(&self) -> bool {
        match self.end {
            Some(_) => false,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_create_track() {
        let track = Track::create(
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
            String::from("MyTrack"),
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()),
            String::from("Project1"),
            String::from("Workspace"),
        );
        assert_eq!(
            track.id,
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")
        );
        assert_eq!(track.name, String::from("MyTrack"));
        assert_eq!(track.project, String::from("Project1"));
        assert_eq!(track.workspace, String::from("Workspace"));
        assert_eq!(
            track.start,
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            track.end,
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap())
        );
    }

    #[test]
    fn test_start_new_track() {
        let track = Track::start_new_track(
            String::from("MyTrack"),
            String::from("Project1"),
            String::from("Workspace"),
        );
        assert_ne!(track.id.len(), 0);
        assert_eq!(track.name, String::from("MyTrack"));
        assert_eq!(track.project, String::from("Project1"));
        assert_eq!(track.workspace, String::from("Workspace"));
        assert_eq!(track.end, None);
        let datetime_that_should_be_lower_than_start = Utc::now() - Duration::seconds(1);
        assert!(track.start.gt(&datetime_that_should_be_lower_than_start));
        assert!(track.start.lt(&Utc::now()));
    }

    #[test]
    fn test_stop_track() {
        let mut track = Track::start_new_track(
            String::from("MyTrack"),
            String::from("Project1"),
            String::from("Workspace"),
        );
        track.stop_track();
        if let Some(end) = track.end {
            let datetime_that_should_be_lower_than_end = Utc::now() - Duration::seconds(1);
            assert!(end.gt(&datetime_that_should_be_lower_than_end));
            assert!(end.lt(&Utc::now()));
        } else {
            assert!(false, "It's expeced to has a DateTime");
        }
    }

    #[test]
    fn test_is_tracking() {
        let mut track = Track::start_new_track(
            String::from("MyTrack"),
            String::from("Project1"),
            String::from("Workspace"),
        );
        assert_eq!(track.is_tracking(), true);
        track.stop_track();
        assert_eq!(track.is_tracking(), false);
    }
}
