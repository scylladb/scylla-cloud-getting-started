use std::time::Duration;

use scylla::{IntoTypedRows, Session, SessionBuilder};

use crate::{songs::Song, ConnectionDetails};

pub struct Database {
    pub songs: Vec<Song>,
    pub session: Session,
}

impl Database {
    pub async fn new(config: &ConnectionDetails) -> Database {

        let nodes = config.nodes.iter().filter(|i| !i.is_empty()).collect::<Vec<_>>();
        let session: Session = SessionBuilder::new()
            .known_nodes(nodes)
            .connection_timeout(Duration::from_secs(5))
            .user(config.username.to_string(), config.password.to_string())
            .build()
            .await
            .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.");

        return Self {
            songs: vec![],
            session,
        };
    }

    pub async fn list(&self) -> Result<Option<Vec<Song>>, anyhow::Error> {
        let query = "SELECT id, title, album, artist, created_at FROM media_player.songs LIMIT 10";

        let result = self.session.query(query, &[]).await?.rows.map(|row| {
            row.into_typed::<Song>()
                .filter(|v| {
                    v.is_ok()
                })
                .map(|v| v.unwrap())
                .collect::<Vec<_>>()
        });

        Ok(result)
    }

    pub async fn add(&self, item: Song) -> Result<(), anyhow::Error> {
        let query = "
            INSERT INTO media_player.songs (id,title,artist,album,created_at)
            VALUES (?,?,?,?,?);
        ";

        let prepare = self.session.prepare(query).await.unwrap();

        self.session.execute(&prepare, item).await?;

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> () {
        self.songs.remove(index);
    }
}
