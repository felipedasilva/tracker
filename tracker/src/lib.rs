pub mod model;
pub mod repository;
pub mod repository_sqlite;
pub mod service;

use repository_sqlite::{RepositorySQLite, SCHEME};
use service::TrackService;
use sqlite;
use std::sync::Arc;

pub fn init() -> TrackService {
    let connection = Arc::new(sqlite::open("bd.sqlite").unwrap());
    connection.execute(SCHEME).unwrap();
    let repository = Box::new(RepositorySQLite::create(connection));
    TrackService::create(repository)
}
