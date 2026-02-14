# Running Entity Generator CLI

- Generate entity files from database schema
    ```sh
    sea-orm-cli generate models -o crates/infra-db/src/models \
        -u postgresql://migrator:password@localhost:5432/energy \
        -s generation
    ```
