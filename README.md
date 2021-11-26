![](https://docs.covercode.ooo/overview/imgs/mainn.png)

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-blue.svg)](https://conventionalcommits.org)
[![Client](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-client.yml)
[![Services](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml/badge.svg)](https://github.com/Psychedelic/fleek-ooo/actions/workflows/pr-test-runner-services.yml)


# Cover

Cover (short for Code Verification) is an open internet service that helps verify the code of canisters on the Internet Computer.

- Visit [our website](https://covercode.ooo)
- Visit [Cover's Docs](https://docs.covercode.ooo)
- Follow [Cover on Twitter](https://twitter.com/cover_ois)

> This is an alpha release ✨ so that developers can start to play around, test the general Cover architecture during the weekend, and provide feedback to us! The alpha registry shouldn't be considered dependable yet. We will follow-up next week with a release that will include the permissioning ruling necessary to ensure all submissions are fully trusted.

If you are Cover developer, please read the [Developer Readme](./README-dev.md)

## Requirements ⚙️

- Github action
- Canister Id

## Getting started 🤔

### Preparing Access Token 

To use the cover plugin we need to verify if you are the owner of the canister. We do it using 
Github Access Tokens. Here is a [tutorial](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
for creating an access token. 
The cover plugin needs the `repo` scope enabled. 

Once you have generated your access token, you can add it to your repository from the Settings / Secrets page. 
You can create a repository secret called something like `MY_ACCESS_TOKEN` and insert the access token value there. You

Now we are ready to create a build job. 


### Create Build Job

Inside of your canister repo create a directory `.github/workflows/` and add a `myBuild.yml` file,
with the following content. To see a full build example see [build.yml](.github/workflows/build.yml)
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

    container:
      image: fleek/dfxrust

    steps:
      - uses: actions/checkout@v2

      - name: Build WASM
          # HACK: set HOME to get github actions to execute correctly
          export HOME=/root
          export PATH="$HOME/.cargo/bin:${PATH}"
          # Start build
          yarn
          MODE=PRODUCTION dfx build cover --check

      - name: Cover Validator Plugin
        uses: Psychedelic/cover/GithubActionPlugin@main
        with:
          access_token: ${{ secrets.MY_ACCESS_TOKEN }}
          canister_id: "iftvq-niaaa-aaaai-qasga-cai"
          wasm_path: ".dfx/local/canisters/cover/cover.wasm"
```

Whenever you push your code using `production` or `main` branches, the above workflow will be triggered. 
If you successfully generated the `canister.wasm` the [Cover Validation Plugin](./GithubActionPlugin) 
will call an AWS Lambda Function that will add the validation results to the [Cover canister](https://ic.rocks/principal/iftvq-niaaa-aaaai-qasga-cai)

Note that if the provided access token does not have access to this repository, the request will be rejected.

### Build Canister

In order to get the same wasm files on github actions and locally, 
we need to ensure that the build environment on github actions is EXACTLY the same as the local one. 
Thus, if you want to generate a wasm file locally, you must use the same docker image as the github actions is using.

You can either provide your own docker image (We suggest you use ubuntu:20:04 at the base) or 
you use our fleek/dfxrust docker image that includes tools needed to build Rust based canisters. 
The fleek/dfxrust image is build with this [Dockerfile](GithubActionPlugin/dockers/dfxrust/Dockerfile). 

#### Executing local build 

To execute a local build using fleek/dfxrust image, in your local folder run 
 `GithubActionPlugin/dockers/docker-build.sh` to generate wasm files inside of folder `./dfx-build`.

You can tweak the docker-build.sh and the entrypoint.sh scripts to your needs. Just make sure that the 
entrypoint.sh matches your Buld.WASM section in github actions. 


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

( opt record { 
  wasm_checksum = "0xecb74c834fcd93d27dd2c0e35410c3b34cf9f7c45e4721a2fbd92a7babf11eaf"; 
  updated_at = "2021-11-19T15:00:00.280+00:00"; 
  updated_by = principal "6cu3r-liw3y-hmevf-e74z4-ogury-e7ur6-xpyka-764on-gcaqs-cbjps-7qe"; 
  source_snapshot_url = "NA"; 
  canister_id = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"; 
  created_at = "2021-11-19T15:00:00.280+00:00"; 
  created_by = principal "6cu3r-liw3y-hmevf-e74z4-ogury-e7ur6-xpyka-764on-gcaqs-cbjps-7qe";
  git_repo = "Psychedeleic/cover"; 
  git_ref = "refs/heads/main"; 
  git_sha = "ef9ff448ad0973a193d479e7842aa0f7e2bccfdf"; 
  build_log_url = "NA"; 
}, )
```

Now you can compare the returned `wasm_checksum` against the deployed canister Module hash.
To get the canister module hash run:
```sh
dfx canister --no-wallet --network ic info iftvq-niaaa-aaaai-qasga-cai          

Controllers: ique5-maaaa-aaaai-qasfq-cai rftgd-dz3se-hrufx-kwtpc-bc5hj-ha54l-lhxnm-chz5z-5tfmq-6th4y-eqe s4jec-wiaaa-aaaah-qch4q-cai
Module hash: 0xecb74c834fcd93d27dd2c0e35410c3b34cf9f7c45e4721a2fbd92a7babf11eaf
```

You can see that the checksums are equal. You can also use a tool called cover verification.
```
cover-verification rrkah-fqaaa-aaaaa-aaaaq-cai

Wasm checksum: 0xecb74c834fcd93d27dd2c0e35410c3b34cf9f7c45e4721a2fbd92a7babf11eaf
Module hash: 0xecb74c834fcd93d27dd2c0e35410c3b34cf9f7c45e4721a2fbd92a7babf11eaf
Status: Verified
```
