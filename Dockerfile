FROM rust:1.62.0 as builder

# FROM ekidd/rust-musl-builder:stable as builder

# WORKDIR /app
# COPY . .
# RUN cargo install --path .
# FROM debian:buster-slim

# RUN mkdir /app
# WORKDIR /app

# COPY --from=build /app/target/debug/rustly /app/

# COPY --from=build /app/migrations /app/migrations
# COPY --from=build /app/public /app/public
# COPY --from=build /app/templates /app/templates
# COPY --from=build /app/*.toml /app/

# CMD ["/app/rustly"]


# FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/rustly /app/rustly
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/public /app/public
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/*.toml /app/
ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000
CMD ["/app/rustly"]
