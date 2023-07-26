use std::time::Duration;

use scylla::{IntoTypedRows, Session, SessionBuilder};

use crate::songs::Song;

pub struct Database {
    pub songs: Vec<Song>,
    pub session: Session,
}

impl Database {
    pub async fn new() -> Database {
        let session: Session = SessionBuilder::new()
            .known_nodes(&[
                "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
                "node-1.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
                "node-2.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
            ])
            .connection_timeout(Duration::from_secs(5))
            .user("scylla", "r4GnOL2QSDi1wqF")
            .build()
            .await
            .expect("eae deu ruim");

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
                    dbg!(v);
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
