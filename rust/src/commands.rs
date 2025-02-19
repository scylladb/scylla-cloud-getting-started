use chrono::Utc;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    task::JoinSet,
};
use tokio::sync::Semaphore;
use crate::repository::SongRepository;
use crate::songs::Song;
use uuid::{self, Uuid};

pub async fn add_song(repository: &Arc<SongRepository>) -> Result<(), anyhow::Error> {
    let now = Utc::now();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    println!("Song name: ");

    let title = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    println!("Album: ");

    let album = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    println!("Artist: ");
    let artist = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap();

    let song = Song {
        id: Uuid::new_v4(),
        title,
        album,
        artist,
        created_at: now,
    };

    repository.add(&song).await?;
    println!("Song '{}' from artist '{}' Added!", song.title, song.artist);

    Ok(())
}

pub async fn list_songs(repository: &Arc<SongRepository>) -> Result<(), anyhow::Error> {
    println!("Here is the songs added so far: ");
    println!("-----------------------------------");

    repository
        .list()
        .await?
        .into_iter()
        .for_each(|row| {
            println!(
                "ID: {} | Song: {} | Album: {} | Created At: {}",
                row.id,
                row.title,
                row.album,
                row.created_at
            )
        });

    println!("-----------------------------------");

    Ok(())
}

pub async fn delete_song(repository: &SongRepository) -> Result<(), anyhow::Error> {
    let song_list = repository
        .list()
        .await?
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, Song)>>();

    let index_to_delete = get_song_index_to_delete(&song_list).await?;

    println!("{:?}", index_to_delete);

    let song_to_delete = song_list
        .into_iter()
        .find(|row| row.0 == index_to_delete)
        .unwrap();

    println!(
        "Song '{}' from artist '{}' Deleted!",
        song_to_delete.1.title, song_to_delete.1.artist
    );

    repository.remove(song_to_delete.1).await?;

    Ok(())
}

async fn get_song_index_to_delete(song_list: &Vec<(usize, Song)>) -> Result<usize, anyhow::Error> {
    song_list.into_iter().for_each(|(index, song)| {
        println!(
            "Index: {}  | Song: {} | Album: {} | Created At: {}",
            index,
            song.title,
            song.album,
            song.created_at
        )
    });
    println!("Select a index to be deleting:");
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let option = lines
        .next_line()
        .await?
        .ok_or_else(|| "".to_owned())
        .unwrap()
        .parse::<usize>()?;

    Ok(option)
}

pub async fn stress(database: Arc<SongRepository>) -> Result<(), anyhow::Error> {
    println!("------------------------------------");
    println!("Inserting 100.000 records into the database...");
    println!(">    Starting...");

    let start = std::time::Instant::now();
    let mut set = JoinSet::new();

    let semaphore = Arc::new(Semaphore::new(1000));
    (0..1_000_000).into_iter().for_each(|_| {
        let db = Arc::clone(&database);
        let semaphore = Arc::clone(&semaphore);

        set.spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            db.add(&Song {
                id: Uuid::new_v4(),
                title: String::from("Test Song"),
                album: String::from("Test Title"),
                artist: String::from("Test Artist"),
                created_at: Utc::now(),
            })
            .await.unwrap();

            drop(_permit);
        });
    });

    while let Some(res) = set.join_next().await {
        res?
    }

    println!(">    Time elapsed: {} seconds", start.elapsed().as_secs());

    Ok(())
}
