# Translations (WIP)

Translation tool (api) build with rust

## Start docker with postgres database
```zsh
docker run --name translations-postgres -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres
```
### Init ORM
```zsh
diesel setup
```
### Start App
```zsh
cargo run
```