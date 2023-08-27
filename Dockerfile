FROM rust:1.71 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM node:18 AS site
WORKDIR /app
COPY ./site .
RUN npm ci && npm run build

FROM debian:bullseye-slim as runner
COPY --from=builder /usr/local/cargo/bin/electric-sheep /usr/local/bin/electric-sheep
COPY --from=site /app/build /app/site/build
ENV ROCKET_ADDRESS=0.0.0.0
RUN apt-get update && apt-get install postgresql -y
EXPOSE 8000
CMD ["electric-sheep"]