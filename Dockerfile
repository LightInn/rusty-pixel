# Étape de construction
FROM rust:latest as builder
ENV SERVER_PORT=3000
ENV SERVER_HOST=0.0.0.0
ENV DATABASE_URL=/data/db.db3
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Étape finale
FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rusty-pixel /usr/local/bin/rusty-pixel
COPY --from=builder /usr/src/app/.env /usr/local/bin/.env
# Créer un répertoire pour le fichier de base de données SQLite
# et définir la variable d'environnement pour le chemin
WORKDIR /data
ENV DATABASE_URL=/data/db.db3
CMD ["/usr/local/bin/rusty-pixel"]
