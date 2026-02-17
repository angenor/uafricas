# =============================================
# UAfricas Frontend - Production Dockerfile
# Multi-stage build pour Nuxt 4 (SSR)
# =============================================

# Stage 1: Build
FROM node:20-slim AS builder

RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copier les fichiers de dependances d'abord (cache Docker)
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Copier le code source et build
COPY . .

# L'URL de l'API sera /api car Nginx fait le reverse proxy
ENV NUXT_PUBLIC_API_BASE_URL=/api

RUN pnpm build

# Stage 2: Runtime minimal
FROM node:20-slim

WORKDIR /app

# Copier le build Nuxt
COPY --from=builder /app/.output ./.output

EXPOSE 3000

ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["node", ".output/server/index.mjs"]
