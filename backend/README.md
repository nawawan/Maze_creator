# 概要

## テスト方法
databaseのテストではcockroachDBには互換性の問題によりsqlxのtestでアクセスができないため、containerに立てたpostgresにアクセスしてテストを実行する。
```
docker compose up -d
```
を実行してcontainerでpostgresを立てた後にbackendのルートディレクトリで
```
DATABASE_URL=postgres://user:postgres@localhost:5432/user SQLX_OFFLINE=true cargo test --workspace
```
を実行することでテストを行うことができる。

redisについても、docker compose up -dによって同時に建てることができ、
```
REDIS_URL=localhost REDIS_PORT=6379 cargo test --workspace
```
を実行することでテストができる。

## makeによるコマンド実行

- uuidを作成するコマンド
```
cargo make -p backend create-uuid  
```
- hashを作成するコマンド
```
cargo make -p backend create-hash {password} {pepper} {salt}
```