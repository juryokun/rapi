FROM rust

WORKDIR /apps/rapi
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres \
    && cargo install sccache \
    && echo "RUSTC_WRAPPER=$(which sccache)" > ~/.bashrc \
    && rustup component add rust-analysis \
    && rustup component add rust-src

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

RUN cargo build --release --color never
