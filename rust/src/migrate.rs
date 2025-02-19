use scylla::Session;
use std::sync::Arc;

pub async fn migrate_database(database: &Arc<Session>) -> anyhow::Result<()> {
    let keyspace_name = String::from("prod_media_player");
    let tables = vec![
        (
            String::from("songs"),
            String::from(
                "CREATE TABLE prod_media_player.songs (
                    id uuid,
                    title text,
                    album text,
                    artist text,
                    created_at timestamp,
                    PRIMARY KEY (id, created_at)
                )",
            ),
        ),
        (
            String::from("song_counter"),
            String::from(
                "CREATE TABLE prod_media_player.song_counter (
                    song_id uuid,
                    times_played counter,
                    PRIMARY KEY (song_id)
                )",
            ),
        ),
    ];

    println!("-----------------------------------");
    println!("->.......Verifying Database.......<-");

    create_keyspace(database, &keyspace_name).await?;
    println!("->........Keyspace setted.........<-");

    create_tables(database, &keyspace_name, &tables).await?;
    println!("->.........Tables setted..........<-");
    println!("------------------------------------");

    Ok(())
}

async fn create_keyspace(db: &Arc<Session>, keyspace_name: &String) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    let validate_keyspace_query = db
        .prepare("select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?")
        .await?;

    let has_keyspace = db
        .execute_unpaged(&validate_keyspace_query, (keyspace_name,))
        .await?
        .into_rows_result()?
        .rows_num()
        > 0;

    if !has_keyspace {
        let new_keyspace_query = format!(
            "
            CREATE KEYSPACE {} 
                WITH replication = {{
                    'class': 'NetworkTopologyStrategy',
                     'replication_factor': '3'
                }}
                AND durable_writes = true AND tablets = {{'enabled': false}}
        ",
            &keyspace_name
        );

        db.query_unpaged(new_keyspace_query, &[]).await?;
    }

    Ok(())
}

async fn create_tables(
    db: &Arc<Session>,
    keyspace_name: &String,
    tables: &Vec<(String, String)>,
) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    let validate_keyspace_query = db
        .prepare("select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?")
        .await?;

    for table in tables {
        let (table_name, table_query) = table;
        let has_table = db
            .execute_unpaged(&validate_keyspace_query, (&keyspace_name, table_name))
            .await?
            .into_rows_result()?
            .rows_num();

        if has_table == 0 {
            let prepared_table = db.prepare(table_query.as_str()).await?;
            db.execute_unpaged(&prepared_table, &[]).await?;
        }
    }

    Ok(())
}
