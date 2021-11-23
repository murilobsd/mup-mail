# mup-user

Mup user micro service based on hexagonal architecture and rust language

## Development

- rust stable
- rustfmt (lint)
- clippy (lint)
- pre-commit (scripts pre commit)
- typos (source code spell checker)
- committed (enforce commit standards)

install pre-commit

```
pip install --user pre-commit
```

or archlinux pacman:

```
sudo pacman -S python-pre-commit
```

install git-hooks script:

```
pre-commit install
```

install typos

```
cargo install typos-cli
```

or arch linux (aur):

```
paru -S typos-bin
```

install committed

```
cargo install committed
```

### Terraform

create S3 bucket for storing terraform remote state

```
REGION=us-east-1
S3_NAME=mup-terraform-state
aws s3api create-bucket --bucket $S3_NAME --region $REGION
```

Execute apply without lock:

```
terraform apply -lock=false
```

### Docker-compose

create network

```
docker network create mup-network
```

create volumes folder to persite database data

```
mkdir -p volumes/db-data/
mkdir -p volumes/redis-data/
```

```
cp .env_example .env
```

```
docker-compose up
```

[access adminzer (web client) postrgresql](http://localhost:8081)

### Sqlx


```
# install
cargo install sqlx-cli --no-default-features --features postgres

# create db
sqlx database create
sqlx database drop

# migrate
sqlx migrate add -r <name>
sqlx migrate run
sqlx migrate revert

# build off line
cargo sqlx prepare
cargo sqlx prepare --check
```

### Extension

```sql
create extension "uuid-ossp";
create extension citext;
```

[For sql to Rust, you can cast to text: (<thing>::TEXT), for Rust to sql, you can help it figure out that it wants text and then cast it to citext: $1::TEXT::CITEXT](https://github.com/launchbadge/sqlx/issues/295#issuecomment-720100514)

https://argon2.online/

### Benchmark

install drill:

on archlinux:

```
paru -S drill 
```

or via cargo:

```
cargo install drill
```

execute:

```
drill-rs --benchmark benchmark.yml --stats
```
