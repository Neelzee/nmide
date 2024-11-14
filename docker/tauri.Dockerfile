FROM node:lts-bookworm

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
  build-essential \
  curl

# Installing necessary dependencies
# Tauri
RUN apt-get install -y libwebkit2gtk-4.1-dev \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libjavascriptcoregtk-4.1-dev \
  libxdo-dev \
  libsoup-3.0-dev \
  nsis \
  lld \
  llvm \
  clang-tools-16


RUN rm -rf /var/lib/apt/lists/*

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Adds cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["/bin/bash"]