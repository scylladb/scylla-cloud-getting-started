use std::time::Duration;

use anyhow::Context;
use futures::TryStreamExt;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::statement::prepared::PreparedStatement;

use crate::{songs::Song, ConnectionDetails};

pub struct Database {
    pub session: Session,
    list_songs_statement: PreparedStatement,
    add_song_statement: PreparedStatement,
    remove_song_statement: PreparedStatement,
}

impl Database {
    pub async fn new_session(config: &ConnectionDetails) -> Result<Session, anyhow::Error> {
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
            .context("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.")?;

        Ok(session)
    }

    pub async fn new(session: Session) -> Result<Database, anyhow::Error> {
        let list_songs_statement = session
            .prepare(
                "SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10",
            )
            .await?;

        let add_song_statement = session
            .prepare(
                "
            INSERT INTO prod_media_player.songs (id,title,artist,album,created_at)
            VALUES (?,?,?,?,?);
        ",
            )
            .await?;

        let remove_song_statement = session
            .prepare("DELETE FROM prod_media_player.songs WHERE id = ?")
            .await?;

        Ok(Self {
            session,
            list_songs_statement,
            add_song_statement,
            remove_song_statement,
        })
    }

    pub async fn list(&self) -> Result<Vec<Song>, anyhow::Error> {
        let result = self
            .session
            .execute_iter(self.list_songs_statement.clone(), &[])
            .await?
            .rows_stream::<Song>()?
            .try_collect::<Vec<_>>()
            .await
            .map_err(|e| e.into());

        result
    }

    pub async fn add(&self, item: &Song) -> Result<(), anyhow::Error> {
        self.session
            .execute_unpaged(&self.add_song_statement, item)
            .await?;

        Ok(())
    }

    pub async fn remove(&self, item: &Song) -> Result<(), anyhow::Error> {
        self.session
            .execute_unpaged(&self.remove_song_statement, (item.id,))
            .await?;
        Ok(())
    }
}
