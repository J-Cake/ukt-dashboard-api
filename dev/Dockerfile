FROM debian:trixie-slim AS api

LABEL authors="jacob.schneider@med.uni-tuebingen.de"

ENV TZ=Europe/Berlin
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update
RUN apt-get install ca-certificates openssl -y

WORKDIR "/app"
COPY "/target/out/*" "/usr/bin"

CMD ["/usr/bin/api", "-c", "./config.toml"]