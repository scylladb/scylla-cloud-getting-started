'use strict';

require('dotenv').config();
const cassandra = require('@scylladb/driver');
const readline = require('readline');

// Line reader that works correctly with both interactive TTY and piped stdin.
// readline.question() has a known issue where async callbacks and buffered pipe
// input can cause missed lines. This queue-based approach avoids that.
function createLineReader() {
  const lineQueue = [];
  const waitQueue = [];

  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

  rl.on('line', (line) => {
    if (waitQueue.length > 0) {
      const resolve = waitQueue.shift();
      resolve(line);
    } else {
      lineQueue.push(line);
    }
  });

  function readLine(prompt) {
    process.stdout.write(prompt);
    if (lineQueue.length > 0) {
      return Promise.resolve(lineQueue.shift());
    }
    return new Promise(resolve => waitQueue.push(resolve));
  }

  return { rl, readLine };
}

const KEYSPACE = process.env.SCYLLADB_KEYSPACE || 'media_player';

const MIGRATIONS = [
  `CREATE KEYSPACE IF NOT EXISTS ${KEYSPACE} WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'} AND durable_writes = true`,
  `CREATE TABLE IF NOT EXISTS ${KEYSPACE}.playlist (
    id uuid,
    title text,
    album text,
    artist text,
    created_at timestamp,
    PRIMARY KEY (id, created_at)
  ) WITH CLUSTERING ORDER BY (created_at DESC)`,
  `CREATE TABLE IF NOT EXISTS ${KEYSPACE}.song_counter (
    song_id uuid,
    times_played counter,
    PRIMARY KEY (song_id)
  )`,
];

function buildClient() {
  const hosts = (process.env.SCYLLADB_HOSTS || '').split(',').map(h => h.trim()).filter(Boolean);
  if (hosts.length === 0) {
    console.error('Error: SCYLLADB_HOSTS is not set. Copy .env.example to .env and fill in your credentials.');
    process.exit(1);
  }

  // Derive data-center from the first host name (e.g. node-0.aws-us-east-1.xxx → AWS_US_EAST_1)
  const dcMatch = hosts[0].match(/node-\d+\.([^.]+)\./);
  const localDataCenter = dcMatch
    ? dcMatch[1].toUpperCase().replace(/-/g, '_')
    : (process.env.SCYLLADB_DC || 'datacenter1');

  return new cassandra.Client({
    contactPoints: hosts,
    localDataCenter,
    credentials: {
      username: process.env.SCYLLADB_USERNAME || 'scylla',
      password: process.env.SCYLLADB_PASSWORD || '',
    },
    queryOptions: { consistency: cassandra.types.consistencies.localQuorum },
  });
}

async function migrate(client) {
  console.log('Verifying schema migrations...');
  for (const query of MIGRATIONS) {
    await client.execute(query);
  }
  console.log('Schema setup complete!\n');
}

async function listSongs(client) {
  const result = await client.execute(`SELECT id, title, album, artist, created_at FROM ${KEYSPACE}.playlist`);
  return result.rows;
}

async function printSongList(client) {
  const songs = await listSongs(client);
  if (songs.length === 0) {
    console.log('No songs found.');
    return songs;
  }
  songs.forEach((song, i) => {
    console.log(`  [${i}] ${song.title} — ${song.artist} | Album: ${song.album} | id: ${song.id}`);
  });
  return songs;
}

function ask(rl, question) {
  return rl.readLine(question);
}

async function cmdNew(client, rl) {
  const title  = await ask(rl, 'Song title: ');
  const artist = await ask(rl, 'Artist: ');
  const album  = await ask(rl, 'Album: ');

  const id = cassandra.types.Uuid.random();
  await client.execute(
    `INSERT INTO ${KEYSPACE}.playlist (id, title, album, artist, created_at) VALUES (?, ?, ?, ?, ?)`,
    [id, title.trim(), album.trim(), artist.trim(), new Date()],
    { prepare: true }
  );
  console.log(`✓ Song "${title.trim()}" added with id ${id}\n`);
}

async function cmdDelete(client, rl) {
  const songs = await printSongList(client);
  if (songs.length === 0) return;

  const indexStr = await ask(rl, 'Select index to delete: ');
  const index = parseInt(indexStr, 10);
  if (isNaN(index) || index < 0 || index >= songs.length) {
    console.log('Invalid index.\n');
    return;
  }
  const song = songs[index];
  await client.execute(
    `DELETE FROM ${KEYSPACE}.playlist WHERE id = ?`,
    [song.id],
    { prepare: true }
  );
  console.log(`✓ Song "${song.title}" deleted.\n`);
}

async function cmdListen(client, rl) {
  const songs = await printSongList(client);
  if (songs.length === 0) return;

  const indexStr = await ask(rl, 'Select index to mark as listened: ');
  const index = parseInt(indexStr, 10);
  if (isNaN(index) || index < 0 || index >= songs.length) {
    console.log('Invalid index.\n');
    return;
  }
  const song = songs[index];
  await client.execute(
    `UPDATE ${KEYSPACE}.song_counter SET times_played = times_played + 1 WHERE song_id = ?`,
    [song.id],
    { prepare: true }
  );
  console.log(`✓ Incremented play count for "${song.title}".\n`);
}

async function cmdStress(client) {
  console.log('Starting stress test — looping through all songs and incrementing play counts.');
  console.log('Check your ScyllaDB Cloud Monitoring dashboard to observe the query rate.');
  console.log('Press Ctrl+C to stop.\n');

  const songs = await listSongs(client);
  if (songs.length === 0) {
    console.log('No songs to stress-test with. Add some songs first with !new.\n');
    return;
  }

  const prepared = await client.prepare(
    `UPDATE ${KEYSPACE}.song_counter SET times_played = times_played + 1 WHERE song_id = ?`
  );

  let count = 0;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    for (const song of songs) {
      await client.execute(prepared, [song.id], { prepare: false });
      count++;
    }
    process.stdout.write(`\r  Executed ${count} updates...`);
  }
}

const HELP = `
Available commands:
  !new     — Add a new song to your playlist
  !delete  — Delete a song from your playlist
  !listen  — Mark a song as listened (increments counter)
  !stress  — Stress test: loop incrementing counters for all songs
  !q       — Quit
`;

async function main() {
  // Create line reader early so piped stdin lines are buffered from the start.
  const { rl, readLine } = createLineReader();
  const reader = { readLine };

  const client = buildClient();

  process.on('SIGINT', async () => {
    console.log('\nShutting down...');
    await client.shutdown();
    process.exit(0);
  });

  try {
    await client.connect();
    console.log('Connected to ScyllaDB.\n');
  } catch (err) {
    console.error('Connection failed:', err.message);
    process.exit(1);
  }

  await migrate(client);

  console.log('Welcome to ScyllaDB Media Player!');
  console.log(HELP);

  const loop = async () => {
    const input = await readLine('> ');
    const command = input.trim();
    try {
      switch (command) {
        case '!new':
          await cmdNew(client, reader);
          break;
        case '!delete':
          await cmdDelete(client, reader);
          break;
        case '!listen':
          await cmdListen(client, reader);
          break;
        case '!stress':
          await cmdStress(client);
          break;
        case '!q':
          console.log('Goodbye!');
          await client.shutdown();
          rl.close();
          return;
        default:
          if (command) console.log(`Unknown command: ${command}`);
          console.log(HELP);
      }
    } catch (err) {
      console.error('Error:', err.message);
    }
    loop();
  };

  loop();
}

main();
