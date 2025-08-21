FROM neelzee/nmide-rust:latest

RUN apt-get update && apt-get install cmake libclang-14-dev -y

RUN rustup component add llvm-tools-preview

RUN cargo install grcov

CMD ["/bin/bash"]
