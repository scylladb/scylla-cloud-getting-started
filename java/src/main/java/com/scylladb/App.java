package com.scylladb;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;
import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.core.cql.Row;
import io.github.cdimascio.dotenv.Dotenv;

import java.net.InetSocketAddress;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.UUID;

public class App {

    private static CqlSession session;

    public static void main(String[] args) {
        Dotenv dotenv = Dotenv.configure().ignoreIfMissing().load();

        String nodesEnv = getEnv(dotenv, "SCYLLADB_NODES", "");
        String username = getEnv(dotenv, "SCYLLADB_USERNAME", "scylla");
        String password = getEnv(dotenv, "SCYLLADB_PASSWORD", "");
        String localDc  = getEnv(dotenv, "SCYLLADB_LOCAL_DC", "AWS_US_EAST_1");

        if (nodesEnv.isEmpty() || password.isEmpty()) {
            System.err.println("Error: SCYLLADB_NODES and SCYLLADB_PASSWORD must be set (in .env or environment).");
            System.exit(1);
        }

        String[] nodes = nodesEnv.split(",");

        CqlSessionBuilder builder = CqlSession.builder()
                .withAuthCredentials(username, password)
                .withLocalDatacenter(localDc);

        for (String node : nodes) {
            String host = node.trim();
            if (!host.isEmpty()) {
                builder.addContactPoint(new InetSocketAddress(host, 9042));
            }
        }

        session = builder.build();
        System.out.println("-------------------------");
        migrate();
        System.out.println("-------------------------");
        System.out.println("Admin: Welcome to MediaPlayer Metrics");

        Scanner scanner = new Scanner(System.in);
        while (true) {
            System.out.print("\nAvailable commands:\n!new - add a new song\n!delete - delete a specific song\n!listen - record a listen\n!stress - stress test the cluster\n!q - quit\n\nUser: ");
            if (!scanner.hasNextLine()) break;
            String command = scanner.nextLine().trim();

            switch (command) {
                case "!new":
                    addSong(scanner);
                    break;
                case "!delete":
                    deleteSong(scanner);
                    break;
                case "!listen":
                    listenSong(scanner);
                    break;
                case "!stress":
                    stressTest();
                    break;
                case "!q":
                    System.out.println("Exiting...");
                    session.close();
                    System.exit(0);
                    break;
                default:
                    break;
            }
        }

        session.close();
    }

    private static String getEnv(Dotenv dotenv, String key, String defaultVal) {
        String val = System.getenv(key);
        if (val != null && !val.isEmpty()) return val;
        val = dotenv.get(key);
        if (val != null && !val.isEmpty()) return val;
        return defaultVal;
    }

    private static void migrate() {
        System.out.println("Verifying Migrations...");

        session.execute(
            "CREATE KEYSPACE IF NOT EXISTS media_player " +
            "WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'} " +
            "AND durable_writes = true"
        );

        session.execute(
            "CREATE TABLE IF NOT EXISTS media_player.playlist (" +
            "  id uuid," +
            "  title text," +
            "  album text," +
            "  artist text," +
            "  created_at timestamp," +
            "  PRIMARY KEY (id, created_at)" +
            ") WITH CLUSTERING ORDER BY (created_at DESC)"
        );

        session.execute(
            "CREATE TABLE IF NOT EXISTS media_player.song_counter (" +
            "  song_id uuid," +
            "  times_played counter," +
            "  PRIMARY KEY (song_id)" +
            ")"
        );

        System.out.println("Schema setup complete!");
    }

    private static void addSong(Scanner scanner) {
        System.out.print("Which song do you want to add? ");
        String title = scanner.nextLine().trim();
        System.out.print("From which artist? ");
        String artist = scanner.nextLine().trim();
        System.out.print("From which album? ");
        String album = scanner.nextLine().trim();

        PreparedStatement ps = session.prepare(
            "INSERT INTO media_player.playlist (id, title, artist, album, created_at) VALUES (?, ?, ?, ?, ?)"
        );
        session.execute(ps.bind(UUID.randomUUID(), title, artist, album, Instant.now()));

        System.out.println("Admin: song \"" + title + "\" added successfully!");
    }

    private static void deleteSong(Scanner scanner) {
        System.out.println("Admin: Listing all songs registered so far...");
        ResultSet rs = session.execute("SELECT id, title, album, artist FROM media_player.playlist PER PARTITION LIMIT 1 LIMIT 100");
        List<Row> rows = rs.all();

        if (rows.isEmpty()) {
            System.out.println("Admin: No songs found.");
            return;
        }

        System.out.println("------------------------");
        for (int i = 0; i < rows.size(); i++) {
            Row row = rows.get(i);
            System.out.printf("Index: %d | Title: %s | Album: %s | Artist: %s%n",
                    i, row.getString("title"), row.getString("album"), row.getString("artist"));
        }
        System.out.println("------------------------");
        System.out.print("Select an index: ");
        int index;
        try {
            index = Integer.parseInt(scanner.nextLine().trim());
        } catch (NumberFormatException e) {
            System.out.println("Admin: Invalid index.");
            return;
        }

        if (index < 0 || index >= rows.size()) {
            System.out.println("Admin: Index out of range.");
            return;
        }

        UUID id = rows.get(index).getUuid("id");
        PreparedStatement ps = session.prepare("DELETE FROM media_player.playlist WHERE id = ?");
        session.execute(ps.bind(id));

        System.out.println("Admin: song deleted successfully!");
    }

    private static void listenSong(Scanner scanner) {
        System.out.println("Admin: Listing all songs registered so far...");
        ResultSet rs = session.execute("SELECT id, title, album, artist FROM media_player.playlist PER PARTITION LIMIT 1 LIMIT 100");
        List<Row> rows = rs.all();

        if (rows.isEmpty()) {
            System.out.println("Admin: No songs found.");
            return;
        }

        System.out.println("------------------------");
        for (int i = 0; i < rows.size(); i++) {
            Row row = rows.get(i);
            System.out.printf("Index: %d | Title: %s | Album: %s | Artist: %s%n",
                    i, row.getString("title"), row.getString("album"), row.getString("artist"));
        }
        System.out.println("------------------------");
        System.out.print("Select an index: ");
        int index;
        try {
            index = Integer.parseInt(scanner.nextLine().trim());
        } catch (NumberFormatException e) {
            System.out.println("Admin: Invalid index.");
            return;
        }

        if (index < 0 || index >= rows.size()) {
            System.out.println("Admin: Index out of range.");
            return;
        }

        UUID songId = rows.get(index).getUuid("id");

        PreparedStatement counterPs = session.prepare(
            "UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?"
        );
        session.execute(counterPs.bind(songId));

        System.out.println("Admin: Song listen count incremented successfully!");
    }

    private static void stressTest() {
        System.out.println("Looping through all songs...");
        System.out.println("Incrementing 'song_counter' table...");
        System.out.println("Check your ScyllaDB Cloud Monitoring to observe query throughput!");

        ResultSet rs = session.execute("SELECT id FROM media_player.playlist PER PARTITION LIMIT 1");
        List<UUID> ids = new ArrayList<>();
        for (Row row : rs) {
            ids.add(row.getUuid("id"));
        }

        if (ids.isEmpty()) {
            System.out.println("Admin: No songs found. Add some songs first with !new.");
            return;
        }

        PreparedStatement counterPs = session.prepare(
            "UPDATE media_player.song_counter SET times_played = times_played + 1 WHERE song_id = ?"
        );

        long count = 0;
        System.out.println("Press Ctrl+C to stop the stress test.");
        while (true) {
            for (UUID id : ids) {
                session.executeAsync(counterPs.bind(id));
                count++;
            }
            if (count % 1000 == 0) {
                System.out.printf("Executed %d updates...%n", count);
            }
        }
    }
}
