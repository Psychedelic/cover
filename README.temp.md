# Cover
Cover (short for Code Verification) is an open internet service that helps verify the code of canisters on the Internet Computer.

## How it works
- You will build your wasm and hash it with the Dockerfile we provided to make sure we have the same environment and the hash will come out deterministic.
- Then you will provide us with the config you used to build your wasm, these inputs will be validated by Cover-validate before Cover proceeds to build or save your config.
- You can log in with Plug and build your saved configs.
- Cover will create a verification for your build result. The verification will contain information about your build like build status and build URL for you to see the build process.
- The hash Cover built will be compared with the one on network IC
## Usage
   - add this Dockerfile to your repo
```docker
FROM ubuntu:20.04
# Install a basic environment needed for our build tools
ARG DEBIAN_FRONTEND=noninteractive
RUN \
    apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake git

# Replace your Rust version here
ARG rust_version=1.58
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH
RUN curl --fail https://sh.rustup.rs/ -sSf \
        | sh -s -- -y --default-toolchain ${rust_version}-x86_64-unknown-linux-gnu --no-modify-path && \
    rustup default ${rust_version}-x86_64-unknown-linux-gnu && \
    rustup target add wasm32-unknown-unknown
RUN cargo install ic-cdk-optimizer

# Install dfx; the version is picked up from the DFX_VERSION environment variable
# Replace your dfx version here
ENV DFX_VERSION=0.8.4
RUN sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

COPY . /canister
WORKDIR /canister
```
- Build docker image
```bash
$ docker build -t DOCKER_IMAGE_NAME DOCKERFILE_PATH
```
- Run docker image and build your wasm
```bash
$ docker run -it DOCKER_IMAGE_NAME

>>> dfx build --network ic YOUR_CANISTER_NAME
>>> ic-cdk-optimizer YOUR_IN_WASM.wasm -o YOUR_OUT_WASM.wasm

# To deploy and verify your canister wasm hash
>>> dfx canister --network ic install YOUR_CANISTER_NAME -m ACTION  
>>> dfx canister --network ic info YOUR_CANISTER_NAME

# To verify your local wasm hash
$ openssl dgst -sha256 YOUR_WASM | awk '{print $2}'

```
- Verify your wasm hash with Cover
  - **Method 1**: Go to [Cover site](). 
    - To immediately build your config, choose Build then input the field correctly.
    - To build and save your config for later use, log in with Plug, choose Save build config, and inputs required fields. Save and choose a config to build
    - After cover builds your wasm, the hash and the status of the build process will be updated to your verification.  
    - You can check out the Build URL field of the verification to see what went wrong with the build process if it failed. 
  - **Method 2**: Use API. 
    - You can use the API provided by [Cover-validator]() to validate and build your wasm 
    - After that you can use this command to check your verification (it may take a while to build wasm and update your verification):
```bash
$  dfx canister --network ic call COVER_CANISTER_ID getVerificationByCanisterId '(principal"YOUR_CANISTER_ID")'
```
