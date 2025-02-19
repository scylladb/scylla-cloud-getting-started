use crate::songs::Song;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;
use scylla::_macro_internal::DeserializationError;
use std::sync::Arc;

pub struct SongRepository {
    session: Arc<Session>,
    select_song_query: PreparedStatement,
    increment_song_counter_query: PreparedStatement,
    delete_song_query: PreparedStatement,
    insert_song_query: PreparedStatement,
}

impl SongRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        let select_query = session
            .prepare(
                "SELECT id, title, album, artist, created_at FROM prod_media_player.songs LIMIT 10",
            )
            .await
            .expect("Error preparing query");

        let insert_query = session
            .prepare("INSERT INTO prod_media_player.songs (id,title,artist,album,created_at) VALUES (?,?,?,?,?);").await
            .expect("Error preparing query");

        let delete_query = session
            .prepare("DELETE FROM prod_media_player.songs WHERE id = ?")
            .await
            .expect("Error preparing query");

        let increment_query = session
            .prepare(
                "UPDATE prod_media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?",
            )
            .await
            .expect("Error preparing query");

        let repository = Self {
            session,
            select_song_query: select_query,
            increment_song_counter_query: increment_query,
            insert_song_query: insert_query,
            delete_song_query: delete_query,
        };

        repository
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Song>> {
        let response = self
            .session
            .execute_unpaged(&self.select_song_query, ())
            .await?;

        let result = response
            .into_rows_result()?
            .rows::<Song>()?
            .collect::<Result<Vec<Song>, DeserializationError>>()?;

        Ok(result)
    }

    pub async fn times_listened_increment(&self, item: Song) -> Result<(), anyhow::Error> {
        self.session
            .execute_unpaged(&self.increment_song_counter_query, (&item.id,))
            .await?;
        Ok(())
    }

    pub async fn add(&self, item: &Song) -> Result<(), anyhow::Error> {
        self.session
            .execute_unpaged(&self.insert_song_query, &item)
            .await?;
        Ok(())
    }

    pub async fn remove(&self, item: Song) -> Result<(), anyhow::Error> {
        self.session
            .execute_unpaged(&self.delete_song_query, (item.id,))
            .await?;
        Ok(())
    }
}
