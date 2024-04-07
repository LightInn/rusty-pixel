# Étape de construction
FROM rust:latest as builder
ENV SERVER_PORT=3000
ENV SERVER_HOST=0.0.0.0
ENV DATABASE_URL=/data/db.db3
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin rusty-pixel

#FROM scratch as runtime // todo : changer pour opti
FROM rust:latest as runtime
ENV SERVER_PORT=3000
ENV SERVER_HOST=0.0.0.0
ENV DATABASE_URL=/data/db.db3
COPY --from=builder /usr/src/app/target/release/rusty-pixel /app/rusty-pixel
WORKDIR /app/
RUN chmod +x rusty-pixel
RUN mkdir -p /data
ENTRYPOINT ["/app/rusty-pixel"]
