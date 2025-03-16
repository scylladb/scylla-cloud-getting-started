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

    create_keyspace(&session, &keyspace_name).await?;
    println!("->........Keyspace setted.........<-");

    create_tables(&session, &keyspace_name, &tables).await?;
    println!("->.........Tables setted..........<-");
    println!("------------------------------------");

    Ok(())
}

async fn create_keyspace(session: &Session, keyspace_name: &String) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    let validate_keyspace_query = session
        .prepare("select keyspace_name from system_schema.keyspaces WHERE keyspace_name=?")
        .await?;

    let has_keyspace = session
        .execute_unpaged(&validate_keyspace_query, (keyspace_name,))
        .await?
        .into_rows_result()
        .unwrap()
        .rows_num();

    if has_keyspace == 0 {
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
    keyspace_name: &String,
    tables: &Vec<(String, String)>,
) -> Result<(), anyhow::Error> {
    // Verify if the table already exists in the specific Keyspace inside your Cluster
    let validate_keyspace_query = session
        .prepare("select keyspace_name, table_name from system_schema.tables where keyspace_name = ? AND table_name = ?")
        .await?;

    for table in tables {
        let (table_name, table_query) = table;
        let has_table = session
            .execute_unpaged(&validate_keyspace_query, (&keyspace_name, table_name))
            .await?
            .into_rows_result()
            .unwrap()
            .rows_num();

        if has_table == 0 {
            let prepared_table = session.prepare(table_query.as_str()).await?;
            session.execute_unpaged(&prepared_table, &[]).await?;
        }
    }

    Ok(())
}
