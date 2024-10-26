FROM rust as builder

ARG SUBDIR

RUN apt update && apt install -y libpq-dev

#https://www.reddit.com/r/rust/comments/1f0ibyq/rust_diesel_postgres_container/
#RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY ./$SUBDIR .
RUN ls -la

RUN cd $SUBDIR && cargo build --release
#---------------------------------------------
FROM rust as server

RUN apt update && apt install -y libpq-dev

WORKDIR /app

# Copy the built application from the first stage
COPY --from=builder /app/$SUBDIR/target/release/source-board-server /app/source-board-server
COPY --from=builder /app/ARG SUBDIR/Cargo.toml /app/Cargo.toml

RUN ls -la

CMD ["/app/source-board-server"]