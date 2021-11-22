Build fleek/f3o docker image that contains all tools needed to build and deploy IC canisters developed with Rust and Javascript.

The image is build on top of ubuntu:20.04 and contains:
- DFX Tool 0.8.3 
- Rust
  - Rust 1.56.1 
  - Cargo 1.56.0
- Javascript
  - Yarn 1.22.15 
  - node 12.22.5
- Other tools
  - wget 1.20.3
  - curl 7.68.0
  - jq 1.6
  
To build run:
```
docker build -t fleek/f3o:latest -t fleek/f3o:0.2.2 ./GithubActionPlugin/dockers/dfxrust
```
To update hub.docker.com
```
docker push fleek/f3o:0.2.2
docker push fleek/f3o:latest
```  
