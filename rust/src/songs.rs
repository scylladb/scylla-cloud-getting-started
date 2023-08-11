
use scylla::{FromRow, ValueList};
use uuid::Uuid;

use crate::datetime::DateTime;

#[derive(Debug, FromRow, ValueList, Clone)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub created_at: DateTime,
}
