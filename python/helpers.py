from cassandra.cluster import Session
from environment import enviroment



def migrate(session: Session) -> None:
    print("Verifying Migrations...")
    hasKeyspace = session.execute("select keyspace_name from system_schema.keyspaces WHERE keyspace_name=%s",([enviroment['keyspace']]))

    if not hasKeyspace:
        print('Creating keyspace:', enviroment['keyspace'])
        session.execute(f"""
        CREATE KEYSPACE {enviroment['keyspace']}
            WITH replication = {{'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}} 
            AND durable_writes = true;                  
        """)
        
    for tableName, tableQuery in enviroment['tables'].items():
        
        hasTable = session.execute('select keyspace_name,table_name from system_schema.tables where keyspace_name = %s AND table_name = %s', (
            str(enviroment['keyspace']),str(tableName)
        ))
        
        if not hasTable: 
            print(f'Creating table {tableName}...')
            session.execute(tableQuery)
            
    
    print("Schema setup complete!")
        
    