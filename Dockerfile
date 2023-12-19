ARG RUST_VERSION=1.74.1

#FROM rust:${RUST_VERSION}-slim-bookworm AS builder
FROM rust:slim-buster AS builder

RUN USER=root cargo new --bin web-app
WORKDIR ./web-app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./


RUN rm ./target/release/deps/sandbox*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /web-app/target/release/sandbox ${APP}/web-app

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./web-app"]
