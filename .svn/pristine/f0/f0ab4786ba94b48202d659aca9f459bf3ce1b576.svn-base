FROM gcc:bookworm

RUN apt-get update \
  && apt-get install -y cppcheck cmake libc-devtools valgrind \
  && rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]
