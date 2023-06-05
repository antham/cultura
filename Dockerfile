FROM alpine as builder

RUN apk add --no-cache gcc musl-dev curl openssl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

ENV PATH $PATH":/root/.cargo/bin"

WORKDIR /tmp
RUN USER=root cargo new cultura
COPY Cargo.toml Cargo.lock /tmp/cultura/

WORKDIR /tmp/cultura
COPY src src/
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --target=x86_64-unknown-linux-musl

FROM amd64/alpine AS runtime

RUN apk add libgcc
ENV TERM screen-256color
COPY --from=builder /tmp/cultura/target/x86_64-unknown-linux-musl/release/cultura /usr/local/bin
RUN cultura daemon start && \
    cp /root/.config/cultura/config.toml /tmp/ && \
    rm -rf /root/.config/cultura && \
    mkdir /root/.config/cultura && \
    mv /tmp/config.toml /root/.config/cultura/config.toml
ENTRYPOINT ["/usr/local/bin/cultura"]
CMD ["daemon", "start", "true"]
