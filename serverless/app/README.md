# Serverless - Cover Infra

ToDo: botch

## Dev Local Dev

To deploy the lambda offline

```bash
sls offline start --stage dev
or
sls offline start
```

To deploy the serverless to dev

```bash
yarn deploy:dev
```

Check logs

```bash
sls logs -f publish -t
```

Test Publish Sqs
```bash
curl -X POST -H "Content-Type: application/json" \
    -d '{"canister_id":"REMOVE","created_at":"2021/11/15_11:28:01:103133356","git_ref":"refs/heads/feat/github-plugin","git_sha":"6d55a6d3288c708e0a68d8ac8c6277b2bbff3ff1","source_snapshot_url":"N/A","wasm_path":"services/cover/Cargo.toml","wasm_checksum":"0x4d80d6cd59573d16b368929d0754efb5b98eb7ffaaab6d4464218e25f8aaedf3","build_log_url":"TODO"}' \
     http://localhost:3000/dev/publishSqs
```
