FROM rust:1.44.0-alpine3.11 as rust_cargo_build

RUN addgroup --system motion && \ 
    adduser --system --ingroup motion motion

USER motion

ADD src /home/motion/src
ADD Cargo.lock /home/motion
ADD Cargo.toml /home/motion

WORKDIR /home/motion

RUN mkdir /home/motion/.cargo && cargo vendor > /home/motion/.cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose


FROM alpine:3.11 as rust_app

RUN addgroup --system motion && \ 
    adduser --system --ingroup motion motion

USER motion

COPY --from=rust_cargo_build /home/motion/target/release/motion /usr/local/bin

CMD ["motion"]