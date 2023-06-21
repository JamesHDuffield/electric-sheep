FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim as runner
COPY --from=builder /usr/local/cargo/bin/electric-sheep /usr/local/bin/electric-sheep
COPY Rocket.toml .
ENV ROCKET_ADDRESS=0.0.0.0
RUN apt-get update && apt-get install postgresql -y
EXPOSE 8000
CMD ["electric-sheep"]