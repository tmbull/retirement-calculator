# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update
RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install --force cargo-make

WORKDIR /usr/src/retirement-calculator
RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --root . microserver --version 0.1.7 --target=x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.toml
COPY Makefile.toml Makefile.toml

# Seems to be an issue detecting flie changes.
# Possibly related: https://github.com/rust-lang/cargo/issues/5918
#RUN mkdir src/

#RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
#RUN touch src/lib.rs
#
#RUN cargo make build
#RUN sleep 5s
#
#RUN rm -f target/wasm32-unknown-unknown/debug/deps/retirement-calculator*
#RUN rm -f pkg/*
#RUN rm -rf src

COPY src ./src

RUN cargo make build

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=cargo-build /usr/src/retirement-calculator/bin/microserver /microserver

COPY docker-entrypoint.sh /docker-entrypoint.sh
COPY index.html /app/
COPY --from=cargo-build /usr/src/retirement-calculator/pkg /app/pkg

RUN adduser -D microuser
USER microuser
ENTRYPOINT ["/docker-entrypoint.sh"]

#FROM httpd:2.4
#COPY --from=cargo-build /usr/src/retirement-calculator/index.html /usr/local/apache2/htdocs/
#COPY --from=cargo-build /usr/src/retirement-calculator/pkg /usr/local/apache2/htdocs/pkg

#COPY --from=cargo-build /usr/src/retirement-calculator/index.html /app/
#COPY --from=cargo-build /usr/src/retirement-calculator/pkg /app/pkg