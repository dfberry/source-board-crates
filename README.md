# Rust APIs deployed to Azure Container Apps

This repo provides Rust APIs providing data integration with Postgres.

## Infrastructure

* Infra with Azure Developer CLI (AZD): Since Rust isn't supported as a language, I use AZD to only provision the resources in the [infra](infra) folder. AZD is already installed on this development container.
    1. `azd auth login`
    2. `azd provision`
* Deployment with GitHub Actions in the [.github/workflows/deploy_to_aca.yaml](.github/workflows/deploy_to_aca.yaml) file.

## Reuse for more API servers

1. Copy the `api-postgres-source-board` folder into the `crates` to make a new crate.
2. Change the crate name in the Cargo.toml file
3. Change the route in `main.rs` and the route files as needed. 

## Test PG connection

```
./psql -p port -U <database_user> -d <database_name> -h <hostname>
```

```
./psql -p port -U postgres -d postgres -h localhost
```