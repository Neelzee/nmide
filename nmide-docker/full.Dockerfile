FROM node:lts-bookworm

# For SVN
RUN apt-get install -y subversion


# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
  build-essential \
  curl

# Installing necessary dependencies
# Tauri
RUN apt-get install -y libwebkit2gtk-4.0-dev \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Rust-Nmide-FFI
RUN apt-get install -y libclang-dev

# nmide-specific
RUN apt-get install -y cmake

RUN rm -rf /var/lib/apt/lists/*

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Adds cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

# Installs just
RUN cargo install just

CMD ["/bin/bash"]
