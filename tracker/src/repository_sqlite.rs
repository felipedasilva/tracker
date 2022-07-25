use crate::model::Track;
use crate::repository::TrackRepository;
use chrono::{DateTime, Utc};
use sqlite::Value;
use std::sync::Arc;

pub const SCHEME: &str = "
    CREATE TABLE IF NOT EXISTS tracks (
        id TEXT,
        name TEXT,
        start TEXT,
        end TEXT,
        project TEXT,
        workspace TEXT
    );
";

pub struct RepositorySQLite {
    connection: Arc<sqlite::Connection>,
}

impl RepositorySQLite {
    pub fn create(connection: Arc<sqlite::Connection>) -> RepositorySQLite {
        RepositorySQLite { connection }
    }

    fn save_in_sqlite(&self, track: &Track) -> Result<(), sqlite::Error> {
        let sql = match self.find_in_sqlite(track.id.clone()) {
            Ok(_) => "UPDATE tracks SET name = :name, start = :start, end = :end, project = :project, workspace = :workspace WHERE id = :id",
            Err(_) => "INSERT INTO tracks VALUES(:id, :name, :start, :end, :project, :workspace)"
        };
        let statement = self
            .connection
            .prepare(sql)
            .unwrap();
        let mut cursor = statement.into_cursor();
        cursor.bind_by_name(vec![
            (":id", Value::String(track.id.to_string())),
            (":name", Value::String(track.name.to_string())),
            (":start", Value::String(track.start.to_string())),
            (
                ":end",
                Value::String(match track.end {
                    Some(end) => end.to_string(),
                    _ => String::from(""),
                }),
            ),
            (":project", Value::String(track.project.to_string())),
            (":workspace", Value::String(track.workspace.to_string())),
        ])?;
        cursor.next()?;
        Ok(())
    }

    fn find_in_sqlite(&self, id: String) -> Result<Track, ()> {
        let statement = self
            .connection
            .prepare("SELECT * FROM tracks WHERE id = :id")
            .unwrap();
        let mut cursor = statement.into_cursor();
        cursor
            .bind_by_name(vec![(":id", Value::String(id))])
            .unwrap();
        if let Some(row) = cursor.next().unwrap() {
            self.convert_row_to_entity(&row)
        } else {
            Err(())
        }
    }

    fn find_all_in_sqlite(&self) -> Result<Vec<Track>, ()> {
        let statement = self
            .connection
            .prepare("SELECT * FROM tracks ORDER BY end ASC")
            .unwrap();
        let mut cursor = statement.into_cursor();
        let mut tasks = vec![];
        while let Some(row) = cursor.next().unwrap() {
            tasks.push(self.convert_row_to_entity(&row)?);
        }
        Ok(tasks)
    }

    fn convert_row_to_entity(&self, row: &[Value]) -> Result<Track, ()> {
        let start = row[2]
            .as_string()
            .unwrap()
            .parse::<DateTime<Utc>>()
            .unwrap();
        let mut end: Option<DateTime<Utc>> = None;
        let end_row = row[3].as_string().unwrap();
        if end_row.len() > 0 {
            let end_parsed = end_row.parse::<DateTime<Utc>>().unwrap();
            end = Some(end_parsed);
        }
        let track = Track::create(
            String::from(row[0].as_string().unwrap()),
            String::from(row[1].as_string().unwrap()),
            start,
            end,
            String::from(row[4].as_string().unwrap()),
            String::from(row[5].as_string().unwrap()),
        );
        Ok(track)
    }
}

impl TrackRepository for RepositorySQLite {
    fn save(&self, track: &Track) -> Result<(), String> {
        match self.save_in_sqlite(track) {
            Err(_) => Err(String::from(
                "An error happen when tried prepare to save the track",
            )),
            _ => Ok(()),
        }
    }

    fn find(&self, id: String) -> Result<Track, String> {
        match self.find_in_sqlite(id) {
            Ok(track) => Ok(track),
            // Todo: Not found error??
            Err(_) => Err(String::from("An error happen when tried find the track"))
        }
    }

    fn find_all(&self) -> Result<Vec<Track>, String> {
        match self.find_all_in_sqlite() {
            Ok(tracks) => Ok(tracks),
            Err(_) => Err(String::from("An error happen when tried find all the tracks"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn create_connection() -> sqlite::Connection {
        let connection = sqlite::open(":memory:").unwrap();
        connection.execute(SCHEME).unwrap();
        connection
    }

    fn create_repository(connection: sqlite::Connection) -> RepositorySQLite {
        RepositorySQLite::create(Arc::new(connection).clone())
    }

    #[test]
    fn test_save_task() {
        let repository = create_repository(create_connection());
        let track = Track::create(
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
            String::from("MyTrack"),
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()),
            String::from("Project1"),
            String::from("Workspace"),
        );
        match repository.save(&track) {
            Ok(_) => {
                assert!(true, "Task saved");
            }
            Err(_) => {
                assert!(false, "Task didn't saved");
            }
        };
    }

    #[test]
    fn test_find_task_notfound() {
        let repository = create_repository(create_connection());
        match repository.find(String::from("not-found-id")) {
            Ok(_) => {
                assert!(false, "It wasn't expected fond an task");
            }
            Err(_) => {
                assert!(true, "Task not found");
            }
        };
    }

    #[test]
    fn test_find_task() {
        let repository = create_repository(create_connection());
        let track = Track::create(
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
            String::from("MyTrack"),
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()),
            String::from("Project1"),
            String::from("Workspace"),
        );
        repository.save(&track).unwrap();

        match repository.find(String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")) {
            Ok(track) => {
                assert_eq!(track.id, String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"));
                assert_eq!(track.name, String::from("MyTrack"));
                assert_eq!(track.project, String::from("Project1"));
                assert_eq!(track.workspace, String::from("Workspace"));
                assert_eq!(track.start, "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap());
                assert_eq!(track.end, Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()));
            }
            Err(_) => {
                assert!(false, "It was expected fond an task");
            }
        };
    }

    #[test]
    fn test_find_all_tasks() {
        let repository = create_repository(create_connection());
        let track_one = Track::create(
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
            String::from("MyTrack"),
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()),
            String::from("Project1"),
            String::from("Workspace"),
        );
        let track_two = Track::create(
            String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d1"),
            String::from("MyTrack2"),
            "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()),
            String::from("Project2"),
            String::from("Workspace"),
        );
        repository.save(&track_one).unwrap();
        repository.save(&track_two).unwrap();

        match repository.find_all() {
            Ok(tracks) => {
                assert_eq!(tracks.len(), 2);
                let track_one = &tracks[0];
                assert_eq!(track_one.id, String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"));
                assert_eq!(track_one.name, String::from("MyTrack"));
                assert_eq!(track_one.project, String::from("Project1"));
                assert_eq!(track_one.workspace, String::from("Workspace"));
                assert_eq!(track_one.start, "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap());
                assert_eq!(track_one.end, Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()));
                let track_two = &tracks[1];
                assert_eq!(track_two.id, String::from("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d1"));
                assert_eq!(track_two.name, String::from("MyTrack2"));
                assert_eq!(track_two.project, String::from("Project2"));
                assert_eq!(track_two.workspace, String::from("Workspace"));
                assert_eq!(track_two.start, "2022-01-01T01:00:00Z".parse::<DateTime<Utc>>().unwrap());
                assert_eq!(track_two.end, Some("2022-01-01T01:01:00Z".parse::<DateTime<Utc>>().unwrap()));
            }
            Err(_) => {
                assert!(false, "It was expected fond an task");
            }
        };
    }
}
