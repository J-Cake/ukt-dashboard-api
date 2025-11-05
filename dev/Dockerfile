FROM debian:trixie-slim AS builder

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl gnupg2 lsb-release \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN apt-get update # && apt upgrade -y
RUN apt-get install -y jq rustup

RUN rustup default stable
RUN rustup target add "aarch64-unknown-linux-gnu"

COPY "/" "/app/api"
WORKDIR "/app/api"
RUN mkdir -p "/app/api/out"

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/api/target \
    cp $(cargo build --release --message-format json | jq -sr '.[] | select(.reason == "compiler-artifact" and .executable).executable') ./out

FROM debian:trixie-slim AS api

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

RUN apt-get update

WORKDIR "/api"
COPY --from=builder "/app/api/out/*" "/bin"
COPY "./dev/config.toml" "/app"

VOLUME "/api/config.toml"
CMD ["/bin/api", "-c", "./config.toml"]

FROM debian:trixie-slim AS kiosk

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

COPY --from=builder "/app/api/out/*" "/bin"

RUN apt-get update # && apt upgrade -y
RUN apt-get install -y cage libwebkit2gtk-4.1-0 libgtk-3-0t64

ENV KIOSK_HOME="https://public"

ENTRYPOINT ["cage", "--", "/bin/webview", ""]

WORKDIR "/api"
COPY --from=builder "/app/api/out/*" "/bin"
RUN apt-get update

FROM node:trixie-slim AS app

LABEL authors="nico.kaiser@med.uni-tuebingen.de"