# Used for simulating fleek.co deployments of current dir
# docker build -t fleek/f3o-fe .
# docker run --rm -ti \
#  -e CANISTERIUM_CANISTER_ID="rrkah-fqaaa-aaaaa-aaaaq-cai" \
#  -e PAT=YOUR_PERSONAL_AT \
#  fleek/f3o-fe
FROM fleek/f3o:latest

RUN apt-get install nano -y
RUN yarn global add serve

WORKDIR /workspace
COPY . .

EXPOSE 5000
ENTRYPOINT [ "/bin/bash" ]
