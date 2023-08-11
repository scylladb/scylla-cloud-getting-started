use std::time::Duration;

use scylla::{IntoTypedRows, Session, SessionBuilder};

use crate::{songs::Song, ConnectionDetails};

pub struct Database {
    pub session: Session,
}

impl Database {
    pub async fn new(config: &ConnectionDetails) -> Database {
        let nodes = config
            .nodes
            .iter()
            .filter(|i| !i.is_empty())
            .collect::<Vec<_>>();

        let session: Session = SessionBuilder::new()
            .known_nodes(nodes)
            .connection_timeout(Duration::from_secs(5))
            .user(config.username.to_string(), config.password.to_string())
            .build()
            .await
            .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.");

        return Self { session };
    }

    pub async fn list(&self) -> Result<Option<Vec<Song>>, anyhow::Error> {
        let query =
            "SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10";

        let result = self.session.query(query, &[]).await?.rows.map(|row| {
            row.into_typed::<Song>()
                .filter(|v| v.is_ok())
                .map(|v| v.unwrap())
                .collect::<Vec<_>>()
        });

        Ok(result)
    }

    pub async fn add(&self, item: Song) -> Result<(), anyhow::Error> {
        let new_song_query = "
            INSERT INTO prod_media_player.songs (id,title,artist,album,created_at)
            VALUES (?,?,?,?,?);
        ";

        let prepared_song = self.session.prepare(new_song_query).await.unwrap();
        self.session.execute(&prepared_song, item).await?;

        self.session
            .query(
                "UPDATE prod_media_player.added_songs_counter SET amount = amount + 1 WHERE id = 1",
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn remove(&self, item: Song) -> Result<(), anyhow::Error> {
        let prepared_delete = self
            .session
            .prepare("DELETE FROM prod_media_player.songs WHERE id = ?")
            .await?;

        self.session.execute(&prepared_delete, (item.id,)).await?;
        Ok(())
    }
}
