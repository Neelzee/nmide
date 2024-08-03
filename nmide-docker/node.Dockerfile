FROM node:bookworm

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Adds cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

# Installs just
RUN cargo install just


CMD ["/bin/bash"]
