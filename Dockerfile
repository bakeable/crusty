# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/reference/dockerfile/

################################################################################
# Create a stage for building the application.
ARG RUST_VERSION=1.78.0
ARG APP_NAME=crusty
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev 

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF


################################################################################
# Run migration
FROM rust:${RUST_VERSION}-slim-bullseye AS migration
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

COPY ./.scripts/migration.sh /app/.scripts/migration.sh

RUN chmod +x /app/.scripts/migration.sh

CMD ["/app/.scripts/migration.sh"]


################################################################################
# Create a new stage for running the application that contains the minimal runtime dependencies for the application.
FROM debian:bullseye-slim AS final

# Install libpq library for runtime
RUN apt-get update && apt-get install -y libpq5

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/bin/server"]