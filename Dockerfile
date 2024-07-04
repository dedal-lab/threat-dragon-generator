# Étape de construction avec une image Rust officielle
FROM rust:latest as builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers de votre projet dans le répertoire de travail
COPY . .

# Installer les outils nécessaires pour une compilation statique
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

# Set the appropriate environment variables
ENV CC=musl-gcc
ENV CXX=musl-g++
ENV RUSTFLAGS='-C linker=musl-gcc'

# Compiler l'application en mode release avec une cible musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Étape finale avec une image scratch
FROM scratch

# Copier l'exécutable compilé depuis l'image de build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/threat-dragon-generator /usr/local/bin/threat-dragon-generator

# Définir le point d'entrée de l'image
CMD ["/usr/local/bin/threat-dragon-generator"]

# Définir les volumes pour le partage de fichiers
VOLUME ["/workdir"]
