FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /exif-rs
WORKDIR /exif-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /exif-rs/fuzz/target/x86_64-unknown-linux-gnu/release/exif-fuzz /