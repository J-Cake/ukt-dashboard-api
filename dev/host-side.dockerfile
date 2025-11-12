FROM debian:trixie-slim AS api

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

RUN apt-get update

WORKDIR "/app"
COPY "/target/out/*" "/usr/bin"

CMD ["/usr/bin/api", "-c", "./config.toml"]