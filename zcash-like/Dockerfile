FROM rust:1.88

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    printf 'fn main() {}\n' > src/main.rs && \
    cargo build && \
    rm -rf src

COPY src ./src

RUN cargo build

CMD ["cargo", "run"]