FROM node:20 as frontend-builder

WORKDIR /app

COPY . .

WORKDIR /app/frontend
RUN npm ci
RUN npm run build
# ---

FROM rust:1.70-bookworm as backend-builder

ENV DATABASE_URL=/app/qdp.db

RUN apt-get update 
RUN apt-get install -y sqlite3 libsqlite3-dev 
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app
COPY . .

RUN diesel setup

RUN cargo install --path ./web

# ---

FROM debian:stable-slim

ARG RUST_LOG=info

ENV APPLICATION_DATABASE_URL=/data/qdp.db
ENV APPLICATION_FRONTEND_PATH=/frontend
ENV APPLICATION_HOST=0.0.0.0
ENV APPLICATION_PORT=8080

COPY --from=backend-builder /app/qdp.db /data/qdp.db
COPY --from=backend-builder /usr/local/cargo/bin/web /backend/web
COPY --from=frontend-builder /app/frontend/dist /frontend

EXPOSE 8080

CMD ["/backend/web"]
