from cassandra.cluster import Cluster
from datetime import datetime 
from cassandra.auth import PlainTextAuthProvider
import uuid

songToDelete = {
    "id": uuid.UUID('d754f8d5-e037-4898-af75-44587b9cc424'),
    "title": 'Glimpse of Us',
    "album": '2022 Em Uma Música',
    "artist": 'Lucas Inutilismo',
    "createdAt": datetime.now()
}

cluster = Cluster(
    contact_points=[
        "node-0.aws-sa-east-1.5c3451e0374e0987b65f.clusters.scylla.cloud",
    ],
    auth_provider=PlainTextAuthProvider(username='scylla', password='r4GnOL2QSDi1wqF')
)

session = cluster.connect('media_player')


deleteQuery = session.prepare("DELETE FROM songs WHERE id = ? ")
session.execute(deleteQuery, [songToDelete['id']])
