![](https://docs.covercode.ooo/overview/imgs/mainn.png)

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-blue.svg)](https://conventionalcommits.org)
[![Client](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml)
[![Services](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml)


# Cover

Cover (short for Code Verification) is an open internet service that helps verify the code of canisters on the Internet Computer.

- Visit [our website](https://covercode.ooo)
- Visit [Cover's Docs](https://docs.covercode.ooo)
- Follow [Cover on Twitter](https://twitter.com/cover_ois)

If you are Cover developer, please read the [Developer Readme](./README-dev.md)

## Requirements ⚙️

- Github action
- Canister Id

## Getting started 🤔

### Create Build Action 

Inside of your canister repo create a directory `.github/workflows/` and add a `myBuild.yml` file,
with the following content. To see a full build example see [dfx.yml](.github/workflows/dfx.yml)
```yaml

name: Example canister build using build.js 

on:
  push:
    branches:
      - production
      - main
jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v2

      - name: Setup Node
        uses: actions/setup-node@v2
        with:
          node-version: 16.x
      - run: npm install

      - name: Install cmake
        run: |
          sudo apt-get update
          sudo apt-get install cmake -y

      - name: Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          profile: minimal
          default: true
          override: true
          target: wasm32-unknown-unknown

      - name: IC CDK Optimizer
#        Generates target/wasm32-unknown-unknown/release/canister.wasm 
        run: cargo install ic-cdk-optimizer

      - name: Build WASM
        run: node build.js --name cover

      - name: Cover Validator Plugin
        uses: Psychedelic/cover/GithubActionPlugin@main
        with:
          canister_id: "iftvq-niaaa-aaaai-qasga-cai"
          wasm_path: "target/wasm32-unknown-unknown/release/canister.wasm"
```

Whenever you push your code using `production` or `main` branches, the above workflow will be triggered. 
If you successfully generated the canister.wasm the [Cover Validation Plugin](./GithubActionPlugin) 
will call an AWS Lambda Function that will add the validation results to the [Cover canister](https://ic.rocks/principal/iftvq-niaaa-aaaai-qasga-cai)   

### Checking canister status 

After a few minutes, you should be able to query the Cover canister. 
You can either call it directly
```sh
dfx canister --network=ic call iftvq-niaaa-aaaai-qasga-cai get_verification_by_canister_id '(principal"rrkah-fqaaa-aaaaa-aaaaq-cai")'
```

or you can save the cover canister id in `canister_ids.json`:
```json
{
  "cover": {
    "ic": "iftvq-niaaa-aaaai-qasga-cai"
  }
}
```
And enquire about any canister id:
```sh
dfx canister --network=ic call cover get_verification_by_canister_id '(principal"rrkah-fqaaa-aaaaa-aaaaq-cai")'


```

Now you can compare the returned `wasm_checksum` against the deployed canister Module hash.
To get the canister module hash run:
```sh
dfx canister --no-wallet --network ic info iftvq-niaaa-aaaai-qasga-cai          

Controllers: ique5-maaaa-aaaai-qasfq-cai rftgd-dz3se-hrufx-kwtpc-bc5hj-ha54l-lhxnm-chz5z-5tfmq-6th4y-eqe s4jec-wiaaa-aaaah-qch4q-cai
Module hash: 0xecb74c834fcd93d27dd2c0e35410c3b34cf9f7c45e4721a2fbd92a7babf11eaf
```
