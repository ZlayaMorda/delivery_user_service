# generate dependencies file
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build apps
FROM rust as builder

COPY . /app
WORKDIR /app

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build
RUN chmod +x entrypoint.sh

FROM busybox:1.35.0-uclibc as busybox

# execute apps
FROM gcr.io/distroless/cc-debian11

COPY --from=busybox /bin/sh /bin/sh
COPY --from=busybox /bin/nc /bin/nc
COPY --from=busybox /bin/sleep /bin/sleep

COPY --from=builder /app/target/debug/user_service /app/user_service
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

WORKDIR /app


ENTRYPOINT ["./entrypoint.sh"]
