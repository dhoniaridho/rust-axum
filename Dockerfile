FROM rust:1.84.0 AS builder

ARG APP_NAME=api

COPY . /app
WORKDIR /app
RUN cargo build --release

FROM alpine:3.17.3 AS runtime
RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY --from=builder /app/target/release/$APP_NAME ./
RUN chmod +x /app/$APP_NAME

EXPOSE 3000

CMD [ "./$APP_NAME" ]