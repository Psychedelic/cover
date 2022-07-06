# Cover Canister

Code verification canister

## Local Development

```bash
$ cd $COVER_ROOT
$ make build

# create canister on Local/IC
$ make local
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
$ make format
$ make lint
$ make build
$ make test
# 3. commits
# 4. PR
```
