FROM rust:1.62-buster

COPY ./rust-toolchain.toml .

# Kick off the installation of the toolchain specified in `rust-toolchain.toml`
RUN cargo

RUN apt update && \
  apt install -y qemu-system-x86 && \
  rm -rf /var/lib/apt/lists/*

RUN cargo install bootimage
