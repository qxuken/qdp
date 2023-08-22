FROM node:20 as frontend-builder

WORKDIR /app

COPY . .

RUN npm ci
RUN npm run build

# ---

FROM rust:1.70-alpine as backend-builder

RUN apk update && apk upgrade
RUN apk add --no-cache sqlite sqlite-dev musl-dev

WORKDIR /app
COPY . .
COPY --from=frontend-builder /app/dist ./dist

RUN cargo build --release --locked

# ---

FROM alpine:latest

ENV RUST_LOG=info

ENV DATABASE_URL=/data/qdp.db
ENV APPLICATION_FRONTEND_PATH=/frontend/
ENV APPLICATION_HOST=0.0.0.0
ENV APPLICATION_PORT=8080

COPY --from=backend-builder /app/target/release/web /backend/web

EXPOSE 8080

CMD ["/backend/web"]
