FROM rust:1.44.0-alpine3.11 as rust_cargo_build

RUN addgroup --system motion && \ 
    adduser --system --ingroup motion motion && \
    apk add --no-cache musl-dev

USER motion

ADD --chown=motion:motion src /home/motion/src
ADD --chown=motion:motion Cargo.lock /home/motion
ADD --chown=motion:motion Cargo.toml /home/motion

WORKDIR /home/motion

RUN mkdir /home/motion/.cargo
RUN cargo vendor > /home/motion/.cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose


FROM alpine:3.11 as rust_app

RUN addgroup --system motion && \ 
    adduser --system --ingroup motion motion

#
ADD --chown=motion:motion test/img/ /opt
#

USER motion

COPY --from=rust_cargo_build /home/motion/target/release/motion /usr/local/bin

ENV RUST_LOG=trace

CMD ["motion"]