import os
import re
from cassandra.cluster import Session


_SCHEMA_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'schema.cql')


def _extract_keyspace(cql: str) -> str:
    match = re.search(r'CREATE\s+KEYSPACE\s+(?:IF\s+NOT\s+EXISTS\s+)?(\w+)', cql, re.IGNORECASE)
    if not match:
        raise ValueError("No CREATE KEYSPACE statement found in schema.cql")
    return match.group(1)


def migrate(session: Session) -> str:
    print("Verifying Migrations...")
    cql = open(_SCHEMA_PATH).read()
    keyspace = _extract_keyspace(cql)
    statements = [s.strip() for s in cql.split(';') if s.strip()]
    for stmt in statements:
        session.execute(stmt)
    print("Schema setup complete!")
    return keyspace

    