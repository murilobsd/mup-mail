FROM ekidd/rust-musl-builder:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

ARG EXE mail

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin ${EXE}

FROM alpine AS runtime
RUN apk --no-cache add ca-certificates
RUN addgroup -S mup && adduser -S mup -G mup
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${EXE} /usr/local/bin/
COPY config.toml .
USER mup
CMD ["/usr/local/bin/${EXE}"]
