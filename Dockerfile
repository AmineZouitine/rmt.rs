# The goal is to allow you to easily test rmt without being afraid of possible bugs 
FROM rust:alpine3.16

RUN mkdir /rmt
WORKDIR /rmt
COPY ./ /rmt
RUN apk add build-base
RUN cargo build --release
RUN mv /rmt/target/release/rmt /usr/local/bin

ENTRYPOINT ["tail", "-f", "/dev/null"]