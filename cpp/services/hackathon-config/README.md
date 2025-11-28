# Hackathon Config Service

This service reads the PostgreSQL connection string from the following environment variables (first non-empty wins):

1. `HACKATHON_CONFIG_DB_URL`
2. `DATABASE_URL`
3. `DB_URL`

If none are set, it falls back to `postgres://postgres:postgres@localhost:5432/hhton`.

Example for running locally (e.g., with Docker/WSL):

```bash
export HACKATHON_CONFIG_DB_URL="postgres://postgres:postgres@localhost:5432/hhton"
./hackathon_config
```
