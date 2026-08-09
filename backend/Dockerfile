FROM rust:1.92.0-alpine AS chef

WORKDIR /usr/workspace/src

RUN cargo install cargo-chef

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/workspace/src/recipe.json recipe.json

ENV CARGO_TARGET_DIR=/usr/workspace/target

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM alpine:3.20

COPY --from=builder /usr/workspace/target/release/backend ./

CMD ["./backend"]