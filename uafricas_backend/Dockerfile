# =============================================
# UAfricas Backend - Production Dockerfile
# Multi-stage build pour Rust + Actix-Web
# =============================================

# Stage 1: Build
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Installer les dependances systeme pour la compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copier les fichiers de dependances d'abord (cache Docker)
COPY Cargo.toml Cargo.lock ./

# Creer un main.rs factice pour compiler les dependances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copier le code source reel et recompiler
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime minimal
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier le binaire compile
COPY --from=builder /app/target/release/uafricas_backend .

# Creer les repertoires d'upload
RUN mkdir -p uploads/couvertures uploads/documents

EXPOSE 8080

# Variables d'environnement par defaut
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

CMD ["./uafricas_backend"]
