# This is a base image to build substrate nodes
FROM docker.io/paritytech/ci-linux:production as builder

# Here we create the binary in a temporary image.
WORKDIR /karmachain-node
COPY . .
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the binary."
# Set plaform to avoid M1 issue https://github.com/nodejs/help/issues/3239
FROM --platform=linux/amd64 docker.io/library/ubuntu:20.04

# Copy the node binary.
COPY --from=builder /karmachain-node/target/release/karmachain-node /usr/local/bin/karmachain-node
# Copy the chain specification files
COPY --from=builder /karmachain-node/chain-spec /chain-spec

RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev
RUN mkdir -p /chain-data /node-dev/.local/share
RUN chown -R node-dev:node-dev /chain-data
RUN ln -s /chain-data /node-dev/.local/share/karmachain-node
# check if executable works in this container
RUN /usr/local/bin/karmachain-node --version

# unclutter and minimize the attack surface
RUN rm -rf /usr/bin /usr/sbin

USER node-dev

EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/usr/local/bin/karmachain-node"]
