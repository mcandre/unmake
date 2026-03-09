FROM alpine:3.23 AS build
ENV PATH=$PATH:/root/.cargo/bin
RUN apk add -U \
        clang21 \
        curl \
        musl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- --no-modify-path -y
COPY . /src
WORKDIR /src
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target "$(uname -m)-unknown-linux-musl"

FROM scratch
COPY --from=build /src/target/*-unknown-linux-musl/release/unmake /unmake
ENTRYPOINT ["/unmake"]
