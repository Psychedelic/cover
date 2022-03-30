# Cover Canister

Code verification canister

## Local Development

```bash
$ cd $COVER_ROOT
$ dfx start --clean
$ dfx deploy
```

## IC Build

```bash
$ cd $COVER_ROOT

# create canister on Local/IC
$ dfx build
```

## IC Deployment

```bash
# for first time deployment
$ dfx canister --network ic create cover
$ dfx canister --network ic install cover -m install

# reinstall
$ dfx canister --network ic install cover -m reinstall

# upgrade
$ dfx canister --network ic install cover -m upgrade
```

## Integration Test
```bash
$ make test
```

## Contribution

```bash
# 1. create a branch
# 2. contributes
$ cargo run > cover.did
$ cargo build
$ cargo test
$ cargo fmt --all
$ cargo clippy
# 3. commits
# 4. PR
```
