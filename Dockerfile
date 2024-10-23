FROM lukemathwalker/cargo-chef:0.1.68-rust-1.82.0-alpine3.20 as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --locked --release

FROM gcr.io/distroless/static-debian12:nonroot-amd64
COPY --from=builder /app/target/release/chrozone /
EXPOSE 3000
ENV PORT="3000"
CMD ["/chrozone"]
