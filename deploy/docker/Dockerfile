# The build context must be built in the project root directory
#`docker build -f deploy/docker/Dockerfile`
#
# Also, buildx is required.

FROM rust:alpine as builder

RUN apk update \
    && apk add --no-cache \
    musl-dev

WORKDIR /usr/src/redmine-to-slack

# Build and cache dependencies earlier.
COPY ./Cargo.lock ./Cargo.toml ./
RUN mkdir src \
    && echo 'fn main(){println!("Hello, world!");}' > src/main.rs \
    && echo '' > src/lib.rs \
    && cargo build --release \
    && (cd target/release && rm -rf .fingerprint/redmine-to-slack-* deps/redmine_to_slack-* deps/redmine_to_slack_lib-* redmine-to-slack* libredmine_to_slack_lib.*) \
    && rm -rf src

# Build and install app
COPY ./resources ./resources
COPY ./src ./src
RUN ls ./ && cargo install --locked --path .

#===============================================================================
FROM alpine

RUN apk update \
    && apk add --no-cache \
    shadow \
    su-exec

ENV CONTAINER_UID=${CONTAINER_UID:-1000}
ENV CONTAINER_GID=${CONTAINER_GID:-1000}
RUN groupadd -g ${CONTAINER_GID} -o redmine_to_slack \
    && useradd -g redmine_to_slack -m -o -u ${CONTAINER_UID} redmine_to_slack \
    && mkdir /app \
    && chown redmine_to_slack:redmine_to_slack /app
WORKDIR /app/

COPY --chown=redmine_to_slack:redmine_to_slack --chmod=550 container/entrypoint.sh ./
ENTRYPOINT ["./entrypoint.sh"]

COPY --chown=redmine_to_slack:redmine_to_slack --chmod=550 --from=builder /usr/local/cargo/bin/redmine-to-slack ./
CMD ["redmine-to-slack"]
