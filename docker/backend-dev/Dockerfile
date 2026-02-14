FROM rust:1.90.0-slim-trixie
WORKDIR /var/tmp/backend

RUN apt update \
    && apt install -y curl
RUN cargo install cargo-watch
COPY --chmod=755 ./docker/backend-dev/cmd.sh /var/tmp/cmd.sh

CMD ["/var/tmp/cmd.sh"]
