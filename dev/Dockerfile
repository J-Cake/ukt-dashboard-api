ARG BUILD_VARIANT=hsb

FROM rust:slim-trixie AS builder

RUN apt-get update
RUN apt-get install -y jq

RUN mkdir -p "/app/out"
COPY "/" "/app/"
WORKDIR "/app/"

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    if [ "$BUILD_VARIANT" = "csb" ]; then \
      cp $(cargo build --release --message-format json | jq -sr '.[] | select(.reason == "compiler-artifact" and .executable).executable') /app/out; \
    else \
      ls -1 /app/target/out/* | xargs -I {} cp {} "/app/out"; \
    fi

WORKDIR "/app"

FROM debian:trixie-slim AS api

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

RUN apt-get update

WORKDIR "/app"
COPY --from=builder "/app/out/*" "/bin"
COPY "./dev/config.toml" "/app"

VOLUME "/api/config.toml"
CMD ["/bin/api", "-c", "./config.toml"]

FROM debian:trixie-slim AS kiosk

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

COPY --from=builder "/app/out/*" "/bin"

RUN apt-get update
RUN apt-get install -y cage libwebkit2gtk-4.1-0 libgtk-3-0t64

ENV KIOSK_HOME="https://public"

ENTRYPOINT ["cage", "--", "/bin/webview", "$KIOSK_HOME"]

FROM node:trixie-slim AS app

LABEL authors="nico.kaiser@med.uni-tuebingen.de"

WORKDIR "/app"
COPY --from=builder "/app/out/*" "/bin"
RUN apt-get update