FROM rust:1.79-alpine3.19 as rust_cargo_build

RUN addgroup --system anzen2 && \ 
    adduser --system --ingroup anzen2 anzen2 && \
    apk add --no-cache musl-dev

USER anzen2

ADD --chown=anzen2:anzen2 src /home/anzen2/src
ADD --chown=anzen2:anzen2 static /home/anzen2/static
ADD --chown=anzen2:anzen2 Cargo.lock /home/anzen2
ADD --chown=anzen2:anzen2 Cargo.toml /home/anzen2

WORKDIR /home/anzen2

RUN mkdir /home/anzen2/.cargo
RUN cargo vendor > /home/anzen2/.cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose


FROM alpine:3.19 as rust_app

RUN addgroup --system anzen2 && \ 
    adduser --system --ingroup anzen2 anzen2

USER anzen2

COPY --from=rust_cargo_build /home/anzen2/target/release/anzen2 /usr/local/bin

ENV RUST_LOG=debug

CMD ["anzen2"]