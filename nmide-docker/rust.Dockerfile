FROM rust:bookworm

RUN cargo install just

CMD ["/bin/bash"]
