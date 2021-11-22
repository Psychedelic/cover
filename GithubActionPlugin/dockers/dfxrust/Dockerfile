FROM ubuntu:20.04

ENV TZ=EST
ENV DFX_VERSION=0.8.3
ENV DFX_CANDID_RELEASE=2021-08-20


RUN apt-get update
RUN DEBIAN_FRONTEND="noninteractive" apt-get -y install tzdata
RUN apt-get install build-essential curl cmake make less wget git rsync jq net-tools nano tree -y

# Install yarn and nodejs
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && apt-get install yarn nodejs -y
RUN node --version \
  && yarn --version

RUN mkdir -p /github/workspace/
WORKDIR /github/workspace/

# Install DFX tools
RUN yes Y | DFX_VERSION="$DFX_VERSION" sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

# Dfinity candid utils
RUN wget https://github.com/dfinity/candid/releases/download/"$DFX_CANDID_RELEASE"/didc-linux64
RUN chmod +x ./didc-linux64
RUN ln -s "$(pwd)/didc-linux64" /usr/local/bin/didc

# Rust (for building rust canisters)
RUN apt-get install python3-pip -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install ic-cdk-optimizer
RUN rustup install stable
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
