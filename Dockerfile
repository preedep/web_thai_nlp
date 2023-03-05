# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update
#RUN apt-get install pkg-config libssl-dev
RUN apt-get install musl-tools -y
RUN apt-get install -y build-essential cmake libcppunit-dev git subversion wget
RUN apt-get install -y zlib1g-dev && rm -rf /var/lib/apt/lists/*

#RUN apk add --no-cache make gcc musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/web_thai_nlp_x

COPY Cargo.toml Cargo.toml
COPY build.rs build.rs
RUN mkdir src/
COPY src/ src/

#RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .


#RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

#FROM debian:buster-slim
#FROM ubuntu:latest
FROM debian:stretch-slim
#FROM debian:latest
RUN apt-get update && apt-get -y install curl libssl-dev

RUN openssl s_client -connect centralus-2.in.applicationinsights.azure.com:443 -showcerts </dev/null 2>/dev/null | sed -e '/-----BEGIN/,/-----END/!d' | tee "/usr/local/share/ca-certificates/ca.crt" >/dev/null && \
update-ca-certificates


RUN mkdir -p /web_thai_nlp_x

COPY --from=cargo-build /usr/local/cargo/bin/web_thai_nlp_x /web_thai_nlp_x/web_thai_nlp_x.linux
RUN mkdir -p /web_thai_nlp_x/static
ADD static/index.html /web_thai_nlp_x/static/index.html


WORKDIR /web_thai_nlp_x

USER 1000
# default command
CMD ["/web_thai_nlp_x/web_thai_nlp_x.linux"]