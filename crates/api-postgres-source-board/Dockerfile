FROM rust as builder

RUN apt-get update && apt-get install -y \
    libpq-dev \
    postgresql-client

#https://www.reddit.com/r/rust/comments/1f0ibyq/rust_diesel_postgres_container/
#RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .

RUN ls -la
RUN cargo build --release
#---------------------------------------------
FROM rust as server

RUN apt-get update && apt-get install -y \
    libpq-dev \
    postgresql-client

WORKDIR /app

# Copy the built application from the first stage
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

RUN ls -la

# Expose port 4000
EXPOSE 4000

ENV PORT 4000

CMD ["/app/server"]