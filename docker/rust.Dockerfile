FROM rust:bookworm

ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["/bin/bash"]
