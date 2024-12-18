fn main() {
    println!("Hello, world!");

    // PGSQL Connection pool (only 2 pools) and timeout of 10 seconds

    // Endpoints:
    // /v1/health

    // /v1/pgsql-cluster/status (status of the cluster) [patroni status]  [Later]
    // /v1/pgsql-cluster/{cluster_id_number}/load (server Load and ram usage and disk usage) [Later]

    // /v1/pgsql-cluster/{cluster_id_number}/pools (SHOW POOLS) [pgbouncer]
    // /v1/pgsql-cluster/{cluster_id_number}/clients (SHOW CLIENTS) [pgbouncer]
    // /v1/pgsql-cluster/{cluster_id_number}/stats (SHOW STATS) [pgbouncer]
    // /v1/pgsql-cluster/{cluster_id_number}/databases (SHOW DATABASES) [pgbouncer]
    // /v1/pgsql-cluster/{cluster_id_number}/users (SHOW USERS) [pgbouncer]
}
