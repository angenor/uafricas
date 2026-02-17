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

# Les composables incluent deja /api/ dans leurs chemins
# On met une valeur vide pour eviter le doublement /api/api/
ENV NUXT_PUBLIC_API_BASE_URL=

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
