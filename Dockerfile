FROM clux/muslrust AS build
WORKDIR /usr/src

# Update CA Certificates
RUN apt update -y && apt install -y ca-certificates
RUN update-ca-certificates

# Build dependencies and rely on cache if Cargo.toml
# or Cargo.lock haven't changed
RUN USER=root cargo new rust-notes-api
WORKDIR /usr/src/rust-notes-api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Second stage
FROM scratch

# Copy the CA certificates
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
# Copy the statically-linked binary to the second stage
COPY --from=build /root/.cargo/bin/rust-notes-api .
USER 1000

CMD ["./rust-notes-api"]