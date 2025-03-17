use scylla::client::session::Session;

pub async fn migrate_database(session: &Session) -> Result<(), anyhow::Error> {
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
        // (
        //     String::from("song_counter"),
        //     String::from(
        //         "CREATE TABLE prod_media_player.song_counter (
        //             song_id uuid,
        //             times_played counter,
        //             PRIMARY KEY (song_id)
        //         )",
        //     ),
        // ),
    ];

    println!("-----------------------------------");
    println!("->.......Verifying Database.......<-");

    create_keyspace(session, &keyspace_name).await?;
    println!("->........Keyspace setted.........<-");

    create_tables(session, &keyspace_name, &tables).await?;
    println!("->.........Tables setted..........<-");
    println!("------------------------------------");

    Ok(())
}

async fn create_keyspace(session: &Session, keyspace_name: &str) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    // Normally we could just use `CREATE KEYSPACE IF NOT EXISTS`.
    // However, this is a nice opportunity to showcase drivers metadata API.
    let has_keyspace = session
        .get_cluster_state()
        .get_keyspace(keyspace_name)
        .is_some();

    if !has_keyspace {
        let new_keyspace_query = format!(
            "
            CREATE KEYSPACE {} 
                WITH replication = {{
                    'class': 'NetworkTopologyStrategy',
                     'replication_factor': '3'
                }}
                AND durable_writes = true
        ",
            &keyspace_name
        );

        session.query_unpaged(new_keyspace_query, &[]).await?;
    }

    Ok(())
}

async fn create_tables(
    session: &Session,
    keyspace_name: &str,
    tables: &[(String, String)],
) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    // Normally we could just use `CREATE TABLE IF NOT EXISTS`.
    // However, this is a nice opportunity to showcase drivers metadata API.
    for table in tables {
        let (table_name, table_query) = table;
        let has_table = session
            .get_cluster_state()
            .get_keyspace(keyspace_name)
            .and_then(|ks| ks.tables.get(table_name))
            .is_some();

        if !has_table {
            session.query_unpaged(table_query.as_str(), &[]).await?;
        }
    }

    Ok(())
}
