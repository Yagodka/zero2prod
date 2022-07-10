tokio
actix
serde
sqlx


cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

Hot to init new DB?
```shell
./scripts/init_db.sh
```

How to add new migration?
```shell
sqlx migrate add create_subscriptions_table
```

How to migrate manually?
```shell
sqlx migrate run
```