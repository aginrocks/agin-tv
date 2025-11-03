ARG BUILDPLATFORM

FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM rust:alpine AS chef
COPY --from=xx / /

RUN apk add clang lld pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS depcacher
COPY api api
COPY tmdb tmdb
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

FROM chef AS planner
COPY api api
COPY tmdb tmdb
WORKDIR /app/api
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Setup the environment for the target platform
ARG TARGETPLATFORM
RUN xx-cargo --setup-target-triple

# Copy workspace and tmdb files to maintain workspace structure
COPY Cargo.toml Cargo.toml
COPY tmdb/Cargo.toml tmdb/Cargo.toml
COPY tmdb/src tmdb/src

# Create dummy desktop/src-tauri workspace member to satisfy workspace requirements
RUN mkdir -p desktop/src-tauri/src && \
    printf '[package]\nname = "desktop"\nversion = "0.0.0"\nedition = "2021"\n\n[dependencies]\n' > desktop/src-tauri/Cargo.toml && \
    printf 'fn main() {}\n' > desktop/src-tauri/src/main.rs

# Copy recipe and build dependencies from api directory context
WORKDIR /app/api
COPY --from=planner /app/api/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    xx-cargo chef cook --release --recipe-path recipe.json

# Build the application
ARG PROJECT_NAME
COPY api .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    xx-cargo build --release --package ${PROJECT_NAME}

# Verify the build
RUN xx-verify --static /app/target/$(xx-cargo --print-target-triple)/release/${PROJECT_NAME}

# Copy the executable to an easily-findable path
RUN mkdir -p /app/target/release
RUN cp /app/target/$(xx-cargo --print-target-triple)/release/${PROJECT_NAME} /app/output

FROM scratch AS runtime
# Copy CA certificates for SSL/TLS verification
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/output /app/api

EXPOSE 42069
ENTRYPOINT ["/app/api"]
