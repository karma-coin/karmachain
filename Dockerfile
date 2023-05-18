# This is a base image to build substrate nodes
FROM docker.io/paritytech/ci-linux:production as builder

# Here we create the binary in a temporary image.
WORKDIR /karmachain-node
COPY . .
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the binary."
FROM docker.io/library/ubuntu:20.04

# Copy the node binary.
COPY --from=builder /karmachain-node/target/release/karmachain-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev && \
  mkdir -p /chain-data /node-dev/.local/share && \
  chown -R node-dev:node-dev /chain-data && \
  ln -s /chain-data /node-dev/.local/share/karmachain-node && \
  # unclutter and minimize the attack surface
  rm -rf /usr/bin /usr/sbin && \
  # check if executable works in this container
  /usr/local/bin/karmachain-node --version

USER node-dev

EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/usr/local/bin/karmachain-node"]
